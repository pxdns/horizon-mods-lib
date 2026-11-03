// The box in the bottom-left corner; the debug screen takes the top corners.
fn potion_counter_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_potion_counter as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let (split, empty) = (m::is(get, "split"), m::is(get, "empty"));
    let mut counts = m::Counts::default();
    for stack in game.inventory.iter().flatten().chain(game.offhand.iter()) {
        counts.add(&stack.item, stack.count);
    }
    let lines: Vec<(String, u32)> = m::lines(counts, split, empty).into_iter().map(|l| (l, m::INK)).collect();
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
