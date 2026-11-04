// The box in the bottom-left corner; the debug screen takes the top corners.
fn armor_hud_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_armor_hud as m;
    use horizon_client::extras::Corner;
    if game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "bar") && !game.creative {
        {
            let armor = game.player.as_mut().map_or(0, |p| p.attribute("minecraft:armor").or_else(|_| p.attribute("armor")).unwrap_or(0.0) as i32);
            if armor > 0 {
                let (wi, hi) = (batch.width as i32, batch.height as i32);
                let row = (hi - game.health_hud.left_height + 10) as f32;
                let label = m::label(armor);
                let x = (wi / 2 - 91) as f32 - game.native_text_width(&label) - 3.0;
                game.native_text(batch, &label, [x, row + 1.0], m::INK, true);
                for (rect, ink) in m::extra_rows(wi, row as i32, armor) {
                    game.native_fill(batch, rect, ink);
                }
            }
        }
    }
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let (percent, hide_full) = (m::is(get, "percent"), m::is(get, "full"));
    let mut pieces = Vec::new();
    for (i, slot) in ["Head", "Chest", "Legs", "Feet"].into_iter().enumerate() {
        if let Some(stack) = &game.armor[i] {
            let max = game.items().get(&stack.item).map_or(0, |info| info.max_damage);
            pieces.push(m::Piece::new(slot, &stack.item, stack.damage, max));
        }
    }
    let lines = m::lines(&pieces, percent, hide_full);
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
