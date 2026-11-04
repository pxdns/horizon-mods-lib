// Once a tick: what was picked up, and whether a container screen is open.
fn item_highlighter_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_item_highlighter as m;
    if !game.native_on(m::ID) || !game.native_window() || game.native_player().is_none() {
        m::reset();
        return;
    }
    let mut now: std::collections::BTreeMap<String, i64> = std::collections::BTreeMap::new();
    for stack in game.inventory.iter().flatten().chain(game.offhand.iter()) {
        *now.entry(stack.item.to_string()).or_insert(0) += i64::from(stack.count);
    }
    let open = matches!(game.screen, Screen::Inventory | Screen::Creative);
    m::tick(now, open, m::is(|k| game.native_setting(m::ID, k), "clear"));
}
