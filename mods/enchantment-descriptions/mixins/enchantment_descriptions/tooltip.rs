// ItemStack.getTooltipLines, then what each enchantment on the item does.
fn enchantment_descriptions_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::lib_enchantment_descriptions as m;
    if !game.native_on(m::ID) {
        return lines;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let Some(data) = stack.data() else {
        return lines;
    };
    let mut found: Vec<String> = Vec::new();
    if m::is(get, "items") {
        found.extend(data.enchantments.iter().map(|(id, _)| id.to_string()));
    }
    if m::is(get, "books") {
        found.extend(data.stored_enchantments.iter().map(|(id, _)| id.to_string()));
    }
    let gray = rgba(170, 170, 170, 255);
    for line in m::lines(&found) {
        lines.push((horizon_client::text::Text::literal(line), gray));
    }
    lines
}
