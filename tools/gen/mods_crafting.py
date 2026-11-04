"""Crafting Tweaks: keys that rotate, balance and clear the crafting grid, made of ordinary slot clicks. Pinned to one Horizon commit."""
CRAFT = []

CRAFT.append(dict(
    slug="crafting-tweaks", id="crafting_tweaks", name="Crafting Tweaks", title="Crafting Tweaks", raw=True,
    usage="| Key | Default | What it does |\n|---|---|---|\n| Rotate | `]` | Turns the outer ring of the crafting grid one step clockwise |\n| Balance | `[` | Evens out stacks of the same item across the grid (when they fit in one stack) |\n| Clear | `\\` | Moves everything in the grid back to your inventory |\n\nOpen your inventory or a crafting table and press the key. They work in single player and as a LAN host.",
    description="Keys that rotate, balance and clear the crafting grid.",
    upstream="Crafting Tweaks (craftingtweaks)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command=None, hooks=["key", "list", "settings", "click"],
    options=[("say", "Say in chat when an action cannot be done", True)],
    keys_expr='vec![[m::ROTATE_KEY.into(), "Rotate".into(), m::ROTATE_DEFAULT.into()], [m::BALANCE_KEY.into(), "Balance".into(), m::BALANCE_DEFAULT.into()], [m::CLEAR_KEY.into(), "Clear".into(), m::CLEAR_DEFAULT.into()]]',
    raw_note=("It reads the crafting grid (`Game::craft_grid`) and clicks slots with `Game::tweak_click`, the call Horizon's own inventory extras use for an ordinary click, so every action is the clicks a player could make. "
              "Neither is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. It works when Horizon runs the world (single player, LAN host), not on a server, and not for the original's buttons, "
              "right-click-for-a-full-stack on the result, hold-W transfer or compress keys. Stacks are matched by item id. It was written from the original's public description, not its source, whose licence does not allow reuse."),
    runtime_extra=r'''
pub const ROTATE_KEY: &str = "rotate";
pub const ROTATE_DEFAULT: &str = "key.keyboard.right.bracket";
pub const BALANCE_KEY: &str = "balance";
pub const BALANCE_DEFAULT: &str = "key.keyboard.left.bracket";
pub const CLEAR_KEY: &str = "clear";
pub const CLEAR_DEFAULT: &str = "key.keyboard.backslash";

/// One cell of the grid, row by row: the item and how many.
pub type Cell = Option<(String, u32)>;

/// One click on a grid cell: 0 picks up, places or swaps, 1 puts one down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Click {
    pub cell: usize,
    pub button: i32,
}

/// The cells of the outer ring, clockwise from the top left, of an `n` by `n` grid.
pub fn ring(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    out.extend(0..n);
    out.extend((1..n).map(|r| r * n + n - 1));
    out.extend((0..n - 1).rev().map(|c| (n - 1) * n + c));
    out.extend((1..n - 1).rev().map(|r| r * n));
    out
}

/// The clicks that turn the ring one step clockwise, or why not: a click on a cell holding the same item as the hand merges
/// instead of swapping, so neighbours holding the same item stop it.
pub fn rotate(cells: &[Cell], n: usize) -> Result<Vec<Click>, String> {
    let ring = ring(n);
    for (i, &a) in ring.iter().enumerate() {
        let b = ring[(i + 1) % ring.len()];
        if let (Some((x, _)), Some((y, _))) = (&cells[a], &cells[b])
            && x == y
        {
            return Err("Cannot rotate: two neighbouring cells hold the same item".to_owned());
        }
    }
    if ring.iter().all(|&c| cells[c].is_none()) {
        return Err("Nothing to rotate".to_owned());
    }
    let last = ring[ring.len() - 1];
    let mut clicks = vec![Click { cell: last, button: 0 }];
    clicks.extend(ring.iter().map(|&cell| Click { cell, button: 0 }));
    Ok(clicks)
}

/// The clicks that even out every item that appears in more than one cell, when all of it fits one stack (`max`): pick all up,
/// then put one down at a time. The cells with the remainder are the first ones.
pub fn balance(cells: &[Cell], max: impl Fn(&str) -> u32) -> Result<Vec<Click>, String> {
    let mut kinds: Vec<&str> = Vec::new();
    for (kind, _) in cells.iter().flatten() {
        if !kinds.contains(&kind.as_str()) {
            kinds.push(kind);
        }
    }
    let mut clicks = Vec::new();
    let mut too_big = false;
    for kind in kinds {
        let spots: Vec<(usize, u32)> = cells.iter().enumerate().filter_map(|(i, c)| c.as_ref().filter(|(k, _)| k == kind).map(|(_, n)| (i, *n))).collect();
        if spots.len() < 2 {
            continue;
        }
        let total: u32 = spots.iter().map(|(_, n)| n).sum();
        if total > max(kind) {
            too_big = true;
            continue;
        }
        let each = total / spots.len() as u32;
        let extra = (total % spots.len() as u32) as usize;
        let target = |position: usize| each + u32::from(position < extra);
        if spots.iter().enumerate().all(|(p, (_, n))| *n == target(p)) {
            continue;
        }
        clicks.extend(spots.iter().map(|&(cell, _)| Click { cell, button: 0 }));
        for (p, &(cell, _)) in spots.iter().enumerate() {
            clicks.extend((0..target(p)).map(|_| Click { cell, button: 1 }));
        }
    }
    if clicks.is_empty() {
        return Err(if too_big { "Cannot balance: the stacks do not fit in one" } else { "Nothing to balance" }.to_owned());
    }
    Ok(clicks)
}

/// The cells to move out, for Clear.
pub fn clear(cells: &[Cell]) -> Vec<usize> {
    cells.iter().enumerate().filter(|(_, c)| c.is_some()).map(|(i, _)| i).collect()
}
''',
    tests=r'''
    fn cell(kind: &str, n: u32) -> Cell {
        Some((kind.to_owned(), n))
    }

    #[test]
    fn the_ring_goes_clockwise_round_the_edge() {
        assert_eq!(ring(3), [0, 1, 2, 5, 8, 7, 6, 3]);
        assert_eq!(ring(2), [0, 1, 3, 2]);
    }

    /// Plays the clicks on a grid the way the game does, for different items: swap with the hand, place into an empty cell.
    fn play(cells: &[Cell], clicks: &[Click]) -> Vec<Cell> {
        let mut cells = cells.to_vec();
        let mut hand: Cell = None;
        for c in clicks {
            std::mem::swap(&mut hand, &mut cells[c.cell]);
        }
        assert!(hand.is_none(), "the hand is empty at the end");
        cells
    }

    #[test]
    fn rotating_moves_every_item_one_step_clockwise() {
        let mut cells: Vec<Cell> = vec![None; 9];
        cells[0] = cell("a", 1);
        cells[1] = cell("b", 2);
        cells[5] = cell("c", 3);
        cells[4] = cell("centre", 1);
        let after = play(&cells, &rotate(&cells, 3).unwrap());
        assert_eq!(after[1], cell("a", 1));
        assert_eq!(after[2], cell("b", 2));
        assert_eq!(after[8], cell("c", 3));
        assert_eq!(after[4], cell("centre", 1), "the middle stays");
        assert_eq!(after[0], None);
    }

    #[test]
    fn rotating_a_full_ring_and_a_two_by_two() {
        let cells: Vec<Cell> = ["a", "b", "c", "d"].iter().map(|k| cell(k, 1)).collect();
        let after = play(&cells, &rotate(&cells, 2).unwrap());
        assert_eq!(after, vec![cell("c", 1), cell("a", 1), cell("d", 1), cell("b", 1)]);
    }

    #[test]
    fn rotating_refuses_equal_neighbours_and_empty_grids() {
        let mut cells: Vec<Cell> = vec![None; 9];
        assert_eq!(rotate(&cells, 3), Err("Nothing to rotate".to_owned()));
        cells[0] = cell("a", 1);
        cells[1] = cell("a", 5);
        assert!(rotate(&cells, 3).unwrap_err().contains("same item"));
    }

    #[test]
    fn balancing_evens_out_one_kind() {
        let cells = vec![cell("log", 7), None, cell("log", 1), cell("stick", 3), None, None, None, None, None];
        let clicks = balance(&cells, |_| 64).unwrap();
        // 8 logs in two cells: 4 and 4. Both picked up, then four put down in each.
        assert_eq!(clicks.iter().filter(|c| c.button == 1 && c.cell == 0).count(), 4);
        assert_eq!(clicks.iter().filter(|c| c.button == 1 && c.cell == 2).count(), 4);
        assert_eq!(clicks.iter().filter(|c| c.button == 0).count(), 2);
        assert!(clicks.iter().all(|c| c.cell != 3), "a single stick is left alone");
    }

    #[test]
    fn the_remainder_goes_to_the_first_cells() {
        let cells = vec![cell("log", 1), cell("log", 1), cell("log", 5)];
        let clicks = balance(&cells, |_| 64).unwrap();
        let put = |cell| clicks.iter().filter(|c| c.button == 1 && c.cell == cell).count();
        assert_eq!((put(0), put(1), put(2)), (3, 2, 2), "7 over three cells");
    }

    #[test]
    fn balancing_says_why_it_did_nothing() {
        assert_eq!(balance(&[cell("log", 2), cell("log", 2)], |_| 64), Err("Nothing to balance".to_owned()));
        assert_eq!(balance(&[cell("log", 60), cell("log", 60)], |_| 64), Err("Cannot balance: the stacks do not fit in one".to_owned()));
        assert_eq!(balance(&[cell("pick", 1), cell("pick", 1)], |_| 1), Err("Cannot balance: the stacks do not fit in one".to_owned()));
    }

    #[test]
    fn clear_lists_the_filled_cells() {
        assert_eq!(clear(&[None, cell("a", 1), None, cell("b", 1)]), [1, 3]);
        assert!(clear(&[None, None]).is_empty());
    }
''',
    helpers={
        "key": '''// The rotate, balance and clear keys while a crafting grid is open.
fn crafting_tweaks_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::@MOD@ as m;
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
''',
    },
))
