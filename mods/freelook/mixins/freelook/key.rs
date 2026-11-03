// The key went down or up: the player's own choice of key.
fn freelook_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_freelook as m;
    if !game.native_on(m::ID) {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let playing = game.native_playing();
    let toggle = m::is(|k| game.native_setting(m::ID, k), "toggle");
    m::key(playing, pressed, toggle, game.yaw, game.pitch);
}
