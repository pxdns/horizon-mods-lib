// The box in the top-right corner; the debug screen takes the top corners.
fn shield_statuses_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_shield_statuses as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::TopRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let hide = m::is(get, "hide");
    let shield = |stack: Option<&horizon_client::items::ItemStack>| stack.is_some_and(|s| game.blocks_attacks(s).is_some());
    let carrying = shield(game.stack_in(game.selected)) || shield(game.offhand.as_ref());
    if !carrying {
        return;
    }
    let blocking = game.using.is_some_and(|(slot, _)| shield(game.stack_in(slot)));
    let lines: Vec<(String, u32)> = m::line(m::status(blocking, game.shield_cooldown), hide).into_iter().collect();
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
