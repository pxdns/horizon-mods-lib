// The box in the top-right corner; the debug screen takes the top corners.
fn fps_display_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_fps_display as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::TopRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let (fps, slowest, low) = game.native_frame_rate();
    let get = |k: &str| game.native_setting(m::ID, k);
    let lines = m::lines(fps, slowest, low, m::is(get, "slowest"), m::is(get, "low"));
    let ink = m::ink(fps, m::is(get, "colour"));
    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::TopRight, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], ink, true);
    }
}
