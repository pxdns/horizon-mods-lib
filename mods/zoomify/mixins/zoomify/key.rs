// The zoom key went down or up: the player's own choice of key.
fn zoomify_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_zoomify as m;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(m::ID) {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    m::key(playing, pressed, |name| game.native_setting(m::ID, name));
}
