// Once a tick: tell the orb merge what the player switched on.
fn clumps_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::entities::lib_clumps as m;
    if !game.native_window() {
        return;
    }
    m::configure(game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "sizes"));
}
