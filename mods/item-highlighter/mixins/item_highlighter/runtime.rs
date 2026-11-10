// Item Highlighter, a native Horizon mod: Marks newly picked-up items with a twinkling star in your inventory.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_item_highlighter`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "item_highlighter";
pub const NAME: &str = "Item Highlighter";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Marks newly picked-up items with a twinkling star in your inventory.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("clear", "Clear the stars when the inventory closes", true)
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
use std::collections::{BTreeMap, BTreeSet};

thread_local! {
    /// What was carried at the last tick, by item id; None until a world is seen.
    static LAST: StateCell<Option<BTreeMap<String, i64>>> = const { StateCell::new(None) };
    /// The kinds of item picked up since the marks were last cleared.
    static NEW: StateCell<BTreeSet<String>> = const { StateCell::new(BTreeSet::new()) };
    static WAS_OPEN: Cell<bool> = const { Cell::new(false) };
    /// True while the mod draws a slot again from inside the hook.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// One tick: the totals now carried, whether a container screen is open, whether to clear the marks when it closes.
pub fn tick(now: BTreeMap<String, i64>, open: bool, clear_on_close: bool) {
    LAST.with_borrow_mut(|last| {
        if let Some(before) = last.as_ref() {
            NEW.with_borrow_mut(|new| {
                for (id, count) in &now {
                    if *count > before.get(id).copied().unwrap_or(0) {
                        new.insert(id.clone());
                    }
                }
            });
        }
        *last = Some(now);
    });
    let was = WAS_OPEN.with(|w| w.replace(open));
    if was && !open && clear_on_close {
        clear();
    }
}

/// No world: forget everything.
pub fn reset() {
    LAST.with_borrow_mut(|l| *l = None);
    clear();
    WAS_OPEN.with(|w| w.set(false));
}

pub fn clear() {
    NEW.with_borrow_mut(BTreeSet::clear);
}

/// Whether items of this kind carry a star.
pub fn is_new(id: &str) -> bool {
    NEW.with_borrow(|n| n.contains(id))
}

/// Marks the nested draw; false when this is the nested one.
pub fn enter() -> bool {
    if INSIDE.with(Cell::get) {
        return false;
    }
    INSIDE.with(|i| i.set(true));
    true
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// The star as rectangles `([x, y, w, h], argb)` relative to the slot's top-left corner; it twinkles over four phases.
pub fn star(millis: u128) -> Vec<([f32; 4], u32)> {
    let phase = (millis / 200) % 4;
    let gold = 0xFF_FFD83A_u32;
    let pale = 0xFF_FFF2A8_u32;
    let arm = if phase.is_multiple_of(2) { 2.0 } else { 1.0 };
    let (centre, arms) = if phase < 2 { (gold, pale) } else { (pale, gold) };
    vec![
        ([1.0, 1.0, 3.0, 3.0], centre),
        ([2.0 - arm + 1.0, 2.0, arm, 1.0], arms),
        ([4.0, 2.0, arm, 1.0], arms),
        ([2.0, 2.0 - arm + 1.0, 1.0, arm], arms),
        ([2.0, 4.0, 1.0, arm], arms),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn totals(items: &[(&str, i64)]) -> BTreeMap<String, i64> {
        items.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect()
    }

    #[test]
    fn picking_something_up_marks_its_kind() {
        reset();
        tick(totals(&[("stone", 10)]), false, true);
        assert!(!is_new("stone"), "the first look only records");
        tick(totals(&[("stone", 10), ("diamond", 1)]), false, true);
        assert!(is_new("diamond"));
        assert!(!is_new("stone"));
        tick(totals(&[("stone", 12), ("diamond", 1)]), false, true);
        assert!(is_new("stone"), "more of a kind counts");
        tick(totals(&[("stone", 5), ("diamond", 1)]), false, true);
        assert!(is_new("stone"), "losing some does not clear it");
        reset();
    }

    #[test]
    fn closing_the_inventory_clears_when_asked() {
        reset();
        tick(totals(&[]), false, true);
        tick(totals(&[("gold_ingot", 1)]), true, true);
        assert!(is_new("gold_ingot"));
        tick(totals(&[("gold_ingot", 1)]), false, true);
        assert!(!is_new("gold_ingot"), "closed");
        tick(totals(&[("gold_ingot", 2)]), true, false);
        tick(totals(&[("gold_ingot", 2)]), false, false);
        assert!(is_new("gold_ingot"), "kept when the setting is off");
        reset();
        assert!(!is_new("gold_ingot"));
    }

    #[test]
    fn the_nested_draw_is_not_decorated_again() {
        leave();
        assert!(enter());
        assert!(!enter());
        leave();
    }

    #[test]
    fn the_star_twinkles_and_fits_the_corner() {
        let a = star(0);
        let b = star(250);
        assert_ne!(a, b);
        assert_eq!(star(800), star(0), "a cycle is 800 ms");
        for (rect, _) in a.iter().chain(b.iter()) {
            assert!(rect[0] >= 0.0 && rect[1] >= 0.0 && rect[0] + rect[2] <= 8.0 && rect[1] + rect[3] <= 8.0, "{rect:?}");
        }
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
