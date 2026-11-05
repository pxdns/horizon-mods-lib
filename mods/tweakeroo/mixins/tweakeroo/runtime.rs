// Tweakeroo, a native Horizon mod: Four of Tweakeroo's tweaks: hand restock, hotbar cycle, fast right click and a tool break warning.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_tweakeroo`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "tweakeroo";
pub const NAME: &str = "Tweakeroo";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Four of Tweakeroo's tweaks: hand restock, hotbar cycle, fast right click and a tool break warning.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("restock", "Hand restock", true),
    ("cycle", "Hotbar cycle key", true),
    ("fast", "Fast right click", false),
    ("warn", "Tool break warning", true),
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

use std::cell::{Cell, RefCell as StateCell};

pub const CYCLE_KEY: &str = "cycle";
pub const CYCLE_DEFAULT: &str = "key.keyboard.semicolon";
/// A tool is about to break at or below this much durability left.
pub const WARN_AT: u32 = 10;

thread_local! {
    /// The selected slot and what it held at the last tick: (slot, item).
    static LAST_HELD: StateCell<Option<(usize, String)>> = const { StateCell::new(None) };
    /// The durability left on the held tool at the last tick, for the warning to fire once when it crosses `WARN_AT`.
    static LAST_LEFT: Cell<Option<u32>> = const { Cell::new(None) };
}

/// Forgets what was held (no world, or the mod is off).
pub fn reset() {
    LAST_HELD.with_borrow_mut(|l| *l = None);
    LAST_LEFT.with(|l| l.set(None));
}

/// Hand restock for one tick. `held` is the selected slot and the item in it now (none when empty); `inventory` is the 36 slots,
/// hotbar first. Returns the inventory slot to move into the selected slot, when the stack in hand has just run out.
pub fn restock(held: (usize, Option<&str>), inventory: &[Option<String>]) -> Option<usize> {
    let (slot, now) = held;
    let before = LAST_HELD.with_borrow_mut(|l| {
        let before = l.clone();
        *l = now.map(|item| (slot, item.to_owned()));
        before
    });
    let (before_slot, item) = before?;
    if now.is_some() || before_slot != slot {
        return None;
    }
    (9..inventory.len()).find(|&i| inventory[i].as_deref() == Some(item.as_str()))
}

/// The (inventory slot, hotbar slot) swaps that rotate the rows: the hotbar swaps with row 1, then row 2, then row 3, which
/// moves the hotbar to row 1, row 1 to row 2, row 2 to row 3 and row 3 to the hotbar.
pub fn cycle_plan() -> Vec<(usize, usize)> {
    (1..=3)
        .flat_map(|row| (0..9).map(move |i| (9 * row + i, i)))
        .collect()
}

/// Forgets what was held, keeping the warning state (the restock is paused: a screen is open, or it is off).
pub fn reset_held() {
    LAST_HELD.with_borrow_mut(|l| *l = None);
}

/// The tool-break warning: true once, when the durability left falls to `WARN_AT` or less from above it.
pub fn warn(left: Option<u32>) -> bool {
    let before = LAST_LEFT.with(|l| l.replace(left));
    matches!((before, left), (Some(b), Some(l)) if b > WARN_AT && l <= WARN_AT)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inv(items: &[(usize, &str)]) -> Vec<Option<String>> {
        let mut v = vec![None; 36];
        for (i, item) in items {
            v[*i] = Some((*item).to_owned());
        }
        v
    }

    #[test]
    fn a_used_up_stack_is_replaced_from_the_inventory() {
        reset();
        let inventory = inv(&[(3, "stone"), (12, "stone"), (20, "dirt")]);
        assert_eq!(
            restock((3, Some("stone")), &inventory),
            None,
            "still holding it"
        );
        let after = inv(&[(12, "stone"), (20, "dirt")]);
        assert_eq!(
            restock((3, None), &after),
            Some(12),
            "the first matching stack past the hotbar"
        );
        assert_eq!(restock((3, None), &after), None, "once");
    }

    #[test]
    fn nothing_to_restock_with_changes_of_slot_or_no_match() {
        reset();
        let a = inv(&[(3, "stone"), (12, "dirt")]);
        restock((3, Some("stone")), &a);
        assert_eq!(restock((4, None), &a), None, "the player changed slot");
        reset();
        restock((3, Some("stone")), &a);
        assert_eq!(
            restock((3, None), &inv(&[(12, "dirt")])),
            None,
            "no stone left"
        );
        reset();
        assert_eq!(
            restock((3, None), &a),
            None,
            "an empty hand that was never full"
        );
    }

    #[test]
    fn the_cycle_swaps_each_row_through_the_hotbar() {
        let plan = cycle_plan();
        assert_eq!(plan.len(), 27);
        assert_eq!(plan[0], (9, 0));
        assert_eq!(plan[26], (35, 8));
        // Play it on rows named by letters: hotbar a, rows b c d.
        let mut slots: Vec<char> = "aaaaaaaaabbbbbbbbbcccccccccddddddddd".chars().collect();
        for (inventory, hotbar) in plan {
            slots.swap(inventory, hotbar);
        }
        let rows: Vec<String> = slots.chunks(9).map(|r| r.iter().collect()).collect();
        assert_eq!(
            rows,
            ["ddddddddd", "aaaaaaaaa", "bbbbbbbbb", "ccccccccc"],
            "the hotbar goes to row 1, each row down, row 3 comes up"
        );
    }

    #[test]
    fn the_warning_fires_once_on_crossing() {
        reset();
        assert!(!warn(Some(200)));
        assert!(!warn(Some(12)));
        assert!(warn(Some(9)), "crossed 10");
        assert!(!warn(Some(8)), "not again");
        assert!(!warn(None));
        assert!(
            !warn(Some(9)),
            "a new tool already below the line does not warn"
        );
        assert!(!warn(Some(300)));
        assert!(warn(Some(10)));
        reset();
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
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
