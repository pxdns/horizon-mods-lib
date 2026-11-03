// The box in the bottom-left corner; the debug screen takes the top corners.
fn totem_counter_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_totem_counter as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let hide = m::is(|k| game.native_setting(m::ID, k), "hide");
    let count: u32 = game
        .inventory
        .iter()
        .flatten()
        .chain(game.offhand.iter())
        .filter(|s| m::is_totem(&s.item))
        .map(|s| s.count)
        .sum();
    let lines: Vec<(String, u32)> = m::line(count, hide).into_iter().collect();
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::BottomLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (line, ink)) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], *ink, true);
    }
}
