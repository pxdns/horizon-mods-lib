// The box in the bottom-left corner; the debug screen takes the top corners.
fn sprint_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_sprint_indicator as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    let mut held = |name: &str| game.native_mapping(name).is_some_and(|(down, _)| down);
    let keys = m::Keys {
        forward: held("key.forward"),
        left: held("key.left"),
        back: held("key.back"),
        right: held("key.right"),
        sprint: held("key.sprint"),
        sneak: held("key.sneak"),
    };
    let game = &*game;
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") {
        return;
    }
    let Some(text) = m::status(keys, m::is(get, "walking"), m::is(get, "idle")) else {
        return;
    };
    let lines = [text.to_owned()];
    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::BottomLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], m::INK, true);
    }
}
