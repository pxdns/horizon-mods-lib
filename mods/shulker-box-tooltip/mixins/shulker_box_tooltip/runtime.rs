// Shulker Box Tooltip, a native Horizon mod: Shows what is inside a shulker box in its tooltip: totals by item, and a graphical 9 by 3 grid of the items.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_shulker_box_tooltip`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "shulker_box_tooltip";
pub const NAME: &str = "Shulker Box Tooltip";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows what is inside a shulker box in its tooltip: totals by item, and a graphical 9 by 3 grid of the items.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("summary", "Totals by item", true),
    ("grid", "Slot grid with item icons", true),
    ("all", "Other containers too", false)
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

use std::collections::BTreeMap;

/// How many item lines the totals show before `and N more kinds`.
pub const MAX_LINES: usize = 8;
/// A shulker box holds 27 slots, three rows of nine.
pub const COLUMNS: usize = 9;
pub const SLOTS: usize = 27;

/// One filled slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub slot: u8,
    pub item: String,
    pub count: u32,
}

/// `minecraft:purple_shulker_box` as `Purple Shulker Box`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Whether the item is a shulker box of any colour.
pub fn is_shulker(id: &str) -> bool {
    id.ends_with("shulker_box")
}

/// The contents totalled by item, most first: `64 x Cobblestone`, then `and N more kinds` past `MAX_LINES`, then the slots used.
pub fn summary(slots: &[Slot]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for s in slots {
        *totals.entry(s.item.as_str()).or_insert(0) += s.count;
    }
    let mut rows: Vec<(&str, u32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let mut out: Vec<String> = rows.iter().take(MAX_LINES).map(|(id, n)| format!("{n} x {}", title(id))).collect();
    if rows.len() > MAX_LINES {
        out.push(format!("and {} more kinds", rows.len() - MAX_LINES));
    }
    out.push(format!("{} of {SLOTS} slots used", slots.len()));
    out
}

thread_local! {
    /// Whether the tooltip being built should get the item grid (set while its lines are made).
    static GRID: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn set_grid(on: bool) {
    GRID.with(|g| g.set(on));
}

pub fn grid_on() -> bool {
    GRID.with(std::cell::Cell::get)
}

/// A slot's cell in the grid is this many pixels square, like a bundle's.
pub const CELL: f32 = 24.0;
/// The grid's size: nine cells across, three down.
pub const GRID_WIDTH: f32 = COLUMNS as f32 * CELL;
pub const GRID_HEIGHT: f32 = (SLOTS / COLUMNS) as f32 * CELL;
/// Spaces on a line of their own that make the tooltip wide enough for the grid.
pub const PAD: usize = 60;

/// The top-left of slot `n` for a tooltip whose box starts at `(x, y)` and is `box_width` wide, the grid centred in it.
pub fn slot_at(n: u8, x: f32, y: f32, box_width: f32) -> Option<[f32; 2]> {
    let n = usize::from(n);
    if n >= SLOTS {
        return None;
    }
    let left = x + ((box_width - GRID_WIDTH) / 2.0).floor().max(0.0);
    Some([left + (n % COLUMNS) as f32 * CELL, y + (n / COLUMNS) as f32 * CELL])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(slot: u8, item: &str, count: u32) -> Slot {
        Slot { slot, item: item.to_owned(), count }
    }

    #[test]
    fn ids_become_titles_and_shulkers_are_found() {
        assert_eq!(title("minecraft:purple_shulker_box"), "Purple Shulker Box");
        assert!(is_shulker("minecraft:shulker_box"));
        assert!(is_shulker("minecraft:red_shulker_box"));
        assert!(!is_shulker("minecraft:chest"));
    }

    #[test]
    fn the_summary_totals_by_item_most_first() {
        let slots = [slot(0, "minecraft:cobblestone", 64), slot(1, "minecraft:cobblestone", 20), slot(5, "minecraft:diamond", 3)];
        assert_eq!(summary(&slots), ["84 x Cobblestone", "3 x Diamond", "3 of 27 slots used"]);
        assert_eq!(summary(&[]), ["0 of 27 slots used"]);
    }

    #[test]
    fn many_kinds_are_cut_off_with_a_count() {
        let slots: Vec<Slot> = (0..12).map(|i| slot(i, &format!("minecraft:item_{i:02}"), 1)).collect();
        let lines = summary(&slots);
        assert_eq!(lines.len(), MAX_LINES + 2);
        assert_eq!(lines[MAX_LINES], "and 4 more kinds");
    }

    #[test]
    fn slots_sit_on_a_nine_by_three_grid_centred_in_the_box() {
        assert_eq!(slot_at(0, 10.0, 20.0, 216.0), Some([10.0, 20.0]));
        assert_eq!(slot_at(10, 10.0, 20.0, 216.0), Some([34.0, 44.0]));
        assert_eq!(slot_at(26, 10.0, 20.0, 216.0), Some([202.0, 68.0]));
        assert_eq!(slot_at(0, 0.0, 0.0, 240.0), Some([12.0, 0.0]), "a wider box centres it");
        assert_eq!(slot_at(0, 0.0, 0.0, 100.0), Some([0.0, 0.0]), "a narrow box never pushes it left");
        assert_eq!(slot_at(27, 0.0, 0.0, 216.0), None);
        assert_eq!((GRID_WIDTH, GRID_HEIGHT), (216.0, 72.0));
    }

    #[test]
    fn the_grid_flag_is_per_tooltip() {
        set_grid(true);
        assert!(grid_on());
        set_grid(false);
        assert!(!grid_on());
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
