// The rotate, balance and clear keys while a crafting grid is open.
fn crafting_tweaks_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_crafting_tweaks as m;
    use horizon_client::menu::Input;
    if !pressed || !game.native_on(m::ID) || game.screen != Screen::Inventory || game.remote.is_some() || game.recipe_search_focused() {
        return;
    }
    let grid = game.craft_size;
    if !(grid == 3 || (game.container.is_none() && !game.mount_active())) {
        return;
    }
    let action = if key == game.native_mod_key(m::ID, m::ROTATE_KEY, m::ROTATE_DEFAULT) {
        0
    } else if key == game.native_mod_key(m::ID, m::BALANCE_KEY, m::BALANCE_DEFAULT) {
        1
    } else if key == game.native_mod_key(m::ID, m::CLEAR_KEY, m::CLEAR_DEFAULT) {
        2
    } else {
        return;
    };
    let cells: Vec<m::Cell> = (0..grid * grid).map(|i| game.craft_grid[i].as_ref().map(|s| (s.item.to_string(), s.count))).collect();
    let say = m::is(|k| game.native_setting(m::ID, k), "say");
    let plan = match action {
        0 => m::rotate(&cells, grid),
        1 => m::balance(&cells, |kind| game.max_stack(&horizon_client::items::ItemStack::new(kind, 1))),
        _ => {
            for cell in m::clear(&cells) {
                game.tweak_click(100 + cell, 0, Input::QuickMove);
            }
            return;
        }
    };
    match plan {
        Ok(clicks) => {
            for c in clicks {
                game.tweak_click(100 + c.cell, c.button, Input::Pickup);
            }
        }
        Err(why) if say => game.native_chat(&why),
        Err(_) => {}
    }
}
