// The box in the top-right corner; the debug screen takes the top corners.
fn apple_skin_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_apple_skin as m;
    use horizon_client::extras::Corner;
    if game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "overlay") && game.riding.is_none() && !game.creative {
        if let Some(p) = game.player.as_ref() {
            let get = |k: &str| game.native_setting(m::ID, k);
            let (food, saturation) = (p.health.food.food, p.health.food.saturation);
            let after = if m::is(get, "preview") {
                game.held()
                    .and_then(|stack| game.items().get(&stack.item).and_then(|info| info.food))
                    .map(|(nutrition, gain)| m::preview(food, saturation, nutrition, gain).food)
            } else {
                None
            };
            for (rect, ink) in m::food_overlay(batch.width as i32, batch.height as i32, food, saturation, after, m::is(get, "saturation")) {
                game.native_fill(batch, rect, ink);
            }
        }
    }
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::TopRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let (want_preview, show_saturation) = (m::is(get, "preview"), m::is(get, "saturation"));
    let Some(p) = game.player.as_ref() else {
        return;
    };
    let (food, saturation) = (p.health.food.food, p.health.food.saturation);
    let held = if want_preview {
        game.held()
            .and_then(|stack| game.items().get(&stack.item).and_then(|info| info.food))
            .map(|(nutrition, gain)| m::preview(food, saturation, nutrition, gain))
    } else {
        None
    };
    let lines: Vec<(String, u32)> = m::lines(food, saturation, held, show_saturation).into_iter().map(|l| (l, m::INK)).collect();
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::TopRight, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (line, ink)) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], *ink, true);
    }
}
