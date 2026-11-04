// Shulker Box Tooltip, a native Horizon mod: Shows what is inside a shulker box in its tooltip: totals by item, and a slot grid.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_shulker_box_tooltip`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "shulker_box_tooltip";
pub const NAME: &str = "Shulker Box Tooltip";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Shows what is inside a shulker box in its tooltip: totals by item, and a slot grid.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("summary", "Totals by item", true),
    ("grid", "Slot grid", false),
    ("all", "Other containers too", false),
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
            c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            })
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
    let mut out: Vec<String> = rows
        .iter()
        .take(MAX_LINES)
        .map(|(id, n)| format!("{n} x {}", title(id)))
        .collect();
    if rows.len() > MAX_LINES {
        out.push(format!("and {} more kinds", rows.len() - MAX_LINES));
    }
    out.push(format!("{} of {SLOTS} slots used", slots.len()));
    out
}

/// A slot's two-letter code: the first letters of the item's first two words, or `..` for an empty slot.
fn code(slot: Option<&Slot>) -> String {
    match slot {
        None => "..".to_owned(),
        Some(s) => {
            let t = title(&s.item);
            let mut words = t.split(' ');
            let first = words.next().and_then(|w| w.chars().next()).unwrap_or('?');
            let second = words
                .next()
                .and_then(|w| w.chars().next())
                .or_else(|| t.chars().nth(1))
                .unwrap_or(' ');
            format!("{first}{}", second.to_ascii_lowercase())
        }
    }
}

/// The three rows of the box as slot codes.
pub fn grid(slots: &[Slot]) -> Vec<String> {
    (0..SLOTS / COLUMNS)
        .map(|row| {
            (0..COLUMNS)
                .map(|col| {
                    let index = (row * COLUMNS + col) as u8;
                    code(slots.iter().find(|s| s.slot == index))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(slot: u8, item: &str, count: u32) -> Slot {
        Slot {
            slot,
            item: item.to_owned(),
            count,
        }
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
        let slots = [
            slot(0, "minecraft:cobblestone", 64),
            slot(1, "minecraft:cobblestone", 20),
            slot(5, "minecraft:diamond", 3),
        ];
        assert_eq!(
            summary(&slots),
            ["84 x Cobblestone", "3 x Diamond", "3 of 27 slots used"]
        );
        assert_eq!(summary(&[]), ["0 of 27 slots used"]);
    }

    #[test]
    fn many_kinds_are_cut_off_with_a_count() {
        let slots: Vec<Slot> = (0..12)
            .map(|i| slot(i, &format!("minecraft:item_{i:02}"), 1))
            .collect();
        let lines = summary(&slots);
        assert_eq!(lines.len(), MAX_LINES + 2);
        assert_eq!(lines[MAX_LINES], "and 4 more kinds");
    }

    #[test]
    fn the_grid_has_three_rows_of_nine_codes() {
        let slots = [
            slot(0, "minecraft:cobblestone", 1),
            slot(10, "minecraft:diamond_pickaxe", 1),
            slot(26, "minecraft:stone", 1),
        ];
        let rows = grid(&slots);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], "Co .. .. .. .. .. .. .. ..");
        assert_eq!(rows[1], ".. Dp .. .. .. .. .. .. ..");
        assert_eq!(rows[2], ".. .. .. .. .. .. .. .. St");
        assert!(rows.iter().all(|r| r.split(' ').count() == COLUMNS));
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
