// The zoom key went down or up: the player's own choice of key.
fn simple_zoom_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_simple_zoom as m;
    if !game.native_on(m::ID) || key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    m::key(game.native_playing(), pressed);
}
