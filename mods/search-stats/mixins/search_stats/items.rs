// The Items tab's list as Horizon builds it (items with a non-zero column, in the chosen sort), narrowed to the search.
fn search_stats_items(game: &crate::game::Game<'_>) -> std::ops::ControlFlow<Vec<String>> {
    let mut items: Vec<String> = Vec::new();
    for (ty, _) in COLUMNS {
        if let Some(m) = game.stats.values.get(&format!("minecraft:{ty}")) {
            for (k, v) in m {
                if *v > 0 && k != "minecraft:air" && !items.contains(k) {
                    items.push(k.clone());
                }
            }
        }
    }
    if let Some((col, order)) = game.stats_ui.sort {
        let ty = COLUMNS[col].0;
        items.sort_by_key(|i| game.stats.get(ty, i) * -order);
    }
    std::ops::ControlFlow::Break(lib_search_stats::filter(items))
}
