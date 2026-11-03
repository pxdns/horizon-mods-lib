// Once a tick: the player's speed, from how far they moved.
fn info_hud_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_info_hud as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    match game.native_player() {
        Some(p) => m::tick(p.x, p.z),
        None => m::reset(),
    }
}
