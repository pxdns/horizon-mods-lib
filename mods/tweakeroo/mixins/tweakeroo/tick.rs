// Once a tick: hand restock, fast right click and the tool break warning.
fn tweakeroo_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_tweakeroo as m;
    use horizon_client::menu::Input;
    if !game.native_on(m::ID) || !game.native_window() || game.native_player().is_none() {
        m::reset();
        return;
    }
    let (restock, fast, warn) = {
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "restock"), m::is(get, "fast"), m::is(get, "warn"))
    };
    let playing = game.native_playing();
    if fast && playing && game.right {
        game.use_delay = 0;
    }
    let selected = game.selected;
    let held = game.inventory.get(selected).and_then(|s| s.as_ref()).map(|s| s.item.to_string());
    if restock && playing && game.remote.is_none() && game.carried.is_none() {
        let inventory: Vec<Option<String>> = game.inventory.iter().take(36).map(|s| s.as_ref().map(|s| s.item.to_string())).collect();
        if let Some(from) = m::restock((selected, held.as_deref()), &inventory) {
            game.tweak_click(from, 0, Input::Pickup);
            game.tweak_click(selected, 0, Input::Pickup);
        }
    } else {
        m::reset_held();
    }
    let left = game.inventory.get(selected).and_then(|s| s.as_ref()).and_then(|s| {
        let max = game.items().get(&s.item).map_or(0, |i| i.max_damage);
        (max > 0).then(|| max.saturating_sub(s.damage))
    });
    if m::warn(if warn { left } else { None }) {
        game.native_chat("Your tool is about to break!");
    }
}
