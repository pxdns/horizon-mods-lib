// Once a tick: clumping is on while the mod is.
fn clumps_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_clumps as m;
    game.native_clumps(game.native_on(m::ID));
}
