// A red mark over the crosshair while it is over a living entity.
fn crosshair_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_crosshair_indicator as m;
    let _ = &corners;
    if !game.native_on(m::ID) || !game.native_playing() || !game.camera_type.first_person() {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") || !m::active(game.crosshair_living()) {
        return;
    }
    let (x0, y0) = m::corner(batch.width as i32, batch.height as i32);
    for (x, y, w, h) in m::rects(m::is(get, "dot")) {
        game.native_fill(batch, [(x0 + x) as f32, (y0 + y) as f32, w as f32, h as f32], m::RED);
    }
}
