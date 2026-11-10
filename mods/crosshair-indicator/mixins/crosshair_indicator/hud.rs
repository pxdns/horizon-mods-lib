// The brackets round the crosshair, drawn after it through its inverting blend.
fn crosshair_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_crosshair_indicator as m;
    let _ = &corners;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let Some(target) = game.native_crosshair_target() else {
        return;
    };
    if !m::active(&target.kind, target.spectator) {
        return;
    }
    let (x0, y0) = m::corner(batch.width as i32, batch.height as i32);
    let start = batch.vertices.len();
    for (x, y) in m::PIXELS {
        game.native_fill(batch, [(x0 + x) as f32, (y0 + y) as f32, 1.0, 1.0], 0xFFFF_FFFF);
    }
    batch.invert_since(start);
}
