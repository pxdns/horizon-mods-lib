// Every key: remembered while held for the lit keys, and the toggle key shows or hides the keyboard.
fn visual_keys_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_visual_keys as m;
    m::held(key, pressed);
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if key == game.native_mod_key(m::ID, m::TOGGLE_KEY, m::TOGGLE_DEFAULT) {
        let stored = {
            let game = &*game;
            m::click(|k| game.native_setting(m::ID, k), 0)
        };
        if let Some((name, value)) = stored {
            game.native_set_setting(m::ID, name, &value);
        }
    }
}
