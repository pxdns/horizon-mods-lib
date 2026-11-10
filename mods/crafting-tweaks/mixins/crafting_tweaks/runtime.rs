// Crafting Tweaks, a native Horizon mod: Keys that rotate, balance and clear the crafting grid.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_crafting_tweaks`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "crafting_tweaks";
pub const NAME: &str = "Crafting Tweaks";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Keys that rotate, balance and clear the crafting grid.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("say", "Say in chat when an action cannot be done", true)
];

thread_local! {
    /// Read once through the game's settings files, then kept.
    static CHOSEN: RefCell<Option<Vec<bool>>> = const { RefCell::new(None) };
}

/// A stored value: `true` or `false`, else the default.
pub fn parse(stored: Option<String>, default: bool) -> bool {
    stored.map_or(default, |v| match v.trim() {
        "true" => true,
        "false" => false,
        _ => default,
    })
}

fn chosen(get: impl Fn(&str) -> Option<String>) -> Vec<bool> {
    CHOSEN.with_borrow_mut(|c| {
        c.get_or_insert_with(|| {
            OPTIONS
                .iter()
                .map(|(key, _, default)| parse(get(key), *default))
                .collect()
        })
        .clone()
    })
}

/// Whether option `key` is on.
pub fn is(get: impl Fn(&str) -> Option<String>, key: &str) -> bool {
    OPTIONS
        .iter()
        .position(|o| o.0 == key)
        .is_some_and(|i| chosen(get)[i])
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let on = chosen(get);
    OPTIONS
        .iter()
        .zip(on)
        .map(|((_, label, _), on)| format!("{label}: {}", if on { "ON" } else { "OFF" }))
        .collect()
}

/// A click on button `i`: flips it and returns what to store (key, value).
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut on = chosen(get);
    let (key, _, _) = *OPTIONS.get(i)?;
    on[i] = !on[i];
    let value = on[i].to_string();
    CHOSEN.with_borrow_mut(|c| *c = Some(on));
    Some((key, value))
}

/// Forgets what was read (tests only).
#[cfg(test)]
pub fn forget_options() {
    CHOSEN.with_borrow_mut(|c| *c = None);
}

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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(
            labels(none).len(),
            OPTIONS.len(),
            "one button per option"
        );
        assert_eq!(is(none, OPTIONS[0].0), OPTIONS[0].2);
        forget_options();
        let stored = |k: &str| (k == OPTIONS[0].0).then(|| "false".to_owned());
        assert!(!is(stored, OPTIONS[0].0));
        forget_options();
        assert!(parse(Some("junk".into()), true));
        assert!(!parse(Some(" false\n".into()), true));
    }

    #[test]
    fn a_click_flips_and_reports_what_to_store() {
        forget_options();
        let none = |_: &str| None;
        let (key, value) = click(none, 0).expect("button 0 exists");
        assert_eq!(key, OPTIONS[0].0);
        assert_eq!(value, (!OPTIONS[0].2).to_string());
        assert_eq!(is(none, key), !OPTIONS[0].2);
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
