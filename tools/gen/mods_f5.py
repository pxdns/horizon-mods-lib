"""F5 Split: separate keys for the back and the front third-person view, as toggle or hold. Hook points plus the game's camera field; pinned."""
F5 = []

F5.append(dict(
    slug="f5-split", id="f5_split", name="F5 Split", title="F5 Split", raw=True,
    usage="| Key | Default | What it does |\n|---|---|---|\n| Back view | `F6` | Press for the view from behind, press again to return to first person |\n| Front view | `F7` | The same for the view facing you |\n\nWith the setting \"Hold the key instead of toggling\" on, the view lasts only while the key is held. F5 keeps cycling as before.",
    description="Separate keys for the back and the front third-person view, as a toggle or while held.",
    upstream="F5 Split (f5split)", upstream_licence="see the upstream project",
    command=None, hooks=["key", "list", "settings", "click"],
    options=[("hold", "Hold the key instead of toggling", False)],
    keys_expr='vec![[m::BACK_KEY.into(), "Back view".into(), m::BACK_DEFAULT.into()], [m::FRONT_KEY.into(), "Front view".into(), m::FRONT_DEFAULT.into()]]',
    raw_note=("It sets the game's camera type (`Game::camera_type`), a private field, from the key hook, which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. "
              "Custom field of view per view and the original's smart return are not included."),
    runtime_extra=r'''
pub const BACK_KEY: &str = "back";
pub const BACK_DEFAULT: &str = "key.keyboard.f6";
pub const FRONT_KEY: &str = "front";
pub const FRONT_DEFAULT: &str = "key.keyboard.f7";

/// The three views of the game's camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    First,
    Back,
    Front,
}

/// The view after `target`'s key went down (`down`) or up. A toggle changes on press only: the same key again
/// returns to first person. Held, the view lasts while the key is down.
pub fn after(current: View, target: View, down: bool, hold: bool) -> View {
    match (hold, down) {
        (false, true) if current == target => View::First,
        (false, true) => target,
        (false, false) => current,
        (true, true) => target,
        (true, false) if current == target => View::First,
        (true, false) => current,
    }
}
''',
    tests=r'''
    #[test]
    fn a_toggle_flips_on_press_and_ignores_release() {
        assert_eq!(after(View::First, View::Back, true, false), View::Back);
        assert_eq!(after(View::Back, View::Back, true, false), View::First, "the same key again");
        assert_eq!(after(View::Back, View::Front, true, false), View::Front, "the other key switches");
        assert_eq!(after(View::Back, View::Back, false, false), View::Back, "release does nothing");
    }

    #[test]
    fn held_lasts_while_the_key_is_down() {
        assert_eq!(after(View::First, View::Front, true, true), View::Front);
        assert_eq!(after(View::Front, View::Front, false, true), View::First);
        assert_eq!(after(View::Back, View::Front, false, true), View::Back, "letting go of a key that is not the current view changes nothing");
    }
''',
    helpers={
        "key": '''// The back-view and front-view keys: the player's own choice of key.
fn @ID@_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::@MOD@ as m;
    use crate::game::perspective::CameraType;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let target = if key == game.native_mod_key(m::ID, m::BACK_KEY, m::BACK_DEFAULT) {
        m::View::Back
    } else if key == game.native_mod_key(m::ID, m::FRONT_KEY, m::FRONT_DEFAULT) {
        m::View::Front
    } else {
        return;
    };
    let current = match game.camera_type {
        CameraType::FirstPerson => m::View::First,
        CameraType::ThirdPersonBack => m::View::Back,
        CameraType::ThirdPersonFront => m::View::Front,
    };
    let hold = {
        let game = &*game;
        m::is(|k| game.native_setting(m::ID, k), "hold")
    };
    game.camera_type = match m::after(current, target, pressed, hold) {
        m::View::First => CameraType::FirstPerson,
        m::View::Back => CameraType::ThirdPersonBack,
        m::View::Front => CameraType::ThirdPersonFront,
    };
}
''',
    },
))
