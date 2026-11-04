// The ping box.
fn ping_view_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use horizon_client::debug_chart::lib_ping_view as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") {
        return;
    }
    let Some(stats) = m::stats() else {
        return;
    };
    let lines = m::lines(stats, m::is(get, "average"), m::is(get, "range"), m::is(get, "colour"));
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::BottomRight, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (line, ink)) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], *ink, true);
    }
}
