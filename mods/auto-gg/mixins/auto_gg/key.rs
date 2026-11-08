// The send key: one message, only when the player presses it, with a pause between sends.
fn auto_gg_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_auto_gg as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::KEY_DEFAULT) {
        return;
    }
    let (message, pause) = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        if !m::is(get, "shown") {
            return;
        }
        (m::value(get, "message"), m::value(get, "pause").parse().unwrap_or(5))
    };
    if let Some(text) = m::send(message, pause, game.ticks) {
        game.send_chat(&text);
    }
}
