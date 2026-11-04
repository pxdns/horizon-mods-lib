// Each frame: tell the mod which logger is the ping logger.
fn ping_view_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use horizon_client::debug_chart::lib_ping_view as m;
    let _ = elapsed;
    let ping: &horizon_client::debug_chart::SampleLogger = &game.debug_charts.ping;
    m::watch(ping as *const horizon_client::debug_chart::SampleLogger as usize);
}
