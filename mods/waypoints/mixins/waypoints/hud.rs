// The box in the bottom-right corner; the debug screen takes the top corners.
fn waypoints_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_waypoints as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let Some(p) = game.native_player() else {
        return;
    };
    m::load(|| game.native_setting(m::ID, m::FILE));
    let here = m::Here { x: p.x, y: p.y, z: p.z, dimension: p.dimension.to_owned() };
    let Some(text) = m::tracked_line(&here, p.yaw) else {
        return;
    };
    let lines = [text];
    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::BottomRight, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], m::INK, true);
    }
}
