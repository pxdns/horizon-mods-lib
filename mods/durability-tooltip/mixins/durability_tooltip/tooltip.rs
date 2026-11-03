// ItemStack.getTooltipLines, then a Durability line for items that wear out.
fn durability_tooltip_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::lib_durability_tooltip as m;
    if !game.native_on(m::ID) {
        return lines;
    }
    let max = game.items().get(&stack.item).map_or(0, |info| info.max_damage);
    let get = |k: &str| game.native_setting(m::ID, k);
    if let Some((text, (r, g, b))) = m::line(stack.damage, max, m::is(get, "percent"), m::is(get, "full")) {
        lines.push((horizon_client::text::Text::literal(text), rgba(r, g, b, 255)));
    }
    lines
}
