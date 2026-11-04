// ItemStack.getTooltipLines, then what is inside a shulker box.
fn shulker_box_tooltip_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::lib_shulker_box_tooltip as m;
    if !game.native_on(m::ID) {
        return lines;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !(m::is(get, "all") || m::is_shulker(&stack.item)) {
        return lines;
    }
    let Some(data) = stack.data() else {
        return lines;
    };
    if data.container.is_empty() {
        return lines;
    }
    let slots: Vec<m::Slot> = data
        .container
        .iter()
        .map(|(slot, s)| m::Slot { slot: *slot, item: s.item.to_string(), count: s.count })
        .collect();
    let gray = rgba(170, 170, 170, 255);
    let mut add = |text: String| lines.push((horizon_client::text::Text::literal(text), gray));
    if m::is(get, "summary") {
        for line in m::summary(&slots) {
            add(line);
        }
    }
    if m::is(get, "grid") {
        for line in m::grid(&slots) {
            add(line);
        }
    }
    lines
}
