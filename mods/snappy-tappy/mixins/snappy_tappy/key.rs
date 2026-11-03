// A key went down or up: remember which of a pair was pressed last.
fn snappy_tappy_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_snappy_tappy as m;
    if game.native_on(m::ID) {
        m::pressed(key, pressed);
    }
}
