// Durability Tooltip, a native Horizon mod: Adds a Durability line to the tooltip of tools, weapons and armor.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_durability_tooltip`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "durability_tooltip";
pub const NAME: &str = "Durability Tooltip";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Adds a Durability line to the tooltip of tools, weapons and armor.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("percent", "Show percent", true),
    ("full", "Show on undamaged items", false),
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

/// A tooltip line's colour as red, green, blue.
pub type Rgb = (u8, u8, u8);

/// The line to add: `Durability: 120 / 250 (48%)`, coloured by how much is left. None for items that do not wear out,
/// and for undamaged ones unless `show_full`.
pub fn line(damage: u32, max: u32, show_percent: bool, show_full: bool) -> Option<(String, Rgb)> {
    if max == 0 || (damage == 0 && !show_full) {
        return None;
    }
    let left = max.saturating_sub(damage);
    let percent = left * 100 / max;
    let colour = match percent {
        51.. => (85, 255, 85),
        26..=50 => (255, 255, 85),
        _ => (255, 85, 85),
    };
    let text = if show_percent {
        format!("Durability: {left} / {max} ({percent}%)")
    } else {
        format!("Durability: {left} / {max}")
    };
    Some((text, colour))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_damaged_wearing_items_get_a_line() {
        assert_eq!(line(0, 250, true, false), None);
        assert_eq!(line(10, 0, true, false), None, "no durability");
        assert_eq!(
            line(0, 250, true, true).unwrap().0,
            "Durability: 250 / 250 (100%)"
        );
    }

    #[test]
    fn the_line_and_colour_follow_what_is_left() {
        assert_eq!(
            line(130, 250, true, false),
            Some(("Durability: 120 / 250 (48%)".to_owned(), (255, 255, 85)))
        );
        assert_eq!(
            line(10, 250, false, false),
            Some(("Durability: 240 / 250".to_owned(), (85, 255, 85)))
        );
        assert_eq!(line(240, 250, true, false).unwrap().1, (255, 85, 85));
        assert_eq!(
            line(999, 250, true, false).unwrap().0,
            "Durability: 0 / 250 (0%)",
            "never below zero"
        );
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
