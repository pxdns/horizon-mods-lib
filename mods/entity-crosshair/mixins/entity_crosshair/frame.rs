// Each frame: tell the render code the mod's settings and whether a living entity is under the crosshair.
fn entity_crosshair_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use horizon_client::hud_layout::lib_entity_crosshair as m;
    let _ = elapsed;
    if !game.native_on(m::ID) {
        m::configure(None, false);
        return;
    }
    let settings = m::Settings::read(|k| game.native_setting(m::ID, k));
    m::configure(Some(settings), m::active_target(game.crosshair_living()));
}
