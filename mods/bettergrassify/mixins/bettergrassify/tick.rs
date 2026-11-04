// Once a tick: tell the mesher what the player switched on.
fn bettergrassify_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::mesher::lib_better_grassify as m;
    if !game.native_window() {
        return;
    }
    m::configure(game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "fancy"));
}
