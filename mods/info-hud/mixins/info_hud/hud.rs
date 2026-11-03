// The box in the top-left corner; the debug screen takes the top corners.
fn info_hud_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_info_hud as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::TopLeft) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let Some(p) = game.native_player() else {
        return;
    };
    let get = |k: &str| game.native_setting(m::ID, k);
    let show = [m::is(get, "speed"), m::is(get, "facing"), m::is(get, "dimension"), m::is(get, "angles")];
    let lines = m::lines(m::speed(), p.yaw, p.pitch, p.dimension, show);
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::TopLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], m::INK, true);
    }
}
