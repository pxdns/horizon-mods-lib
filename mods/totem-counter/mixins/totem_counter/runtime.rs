// Totem Counter, a native Horizon mod: How many totems of undying you carry, red when you have none.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_totem_counter`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "totem_counter";
pub const NAME: &str = "Totem Counter";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "How many totems of undying you carry, red when you have none.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] =
    &[("shown", "Shown", true), ("hide", "Hide when none", false)];

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
/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "totems";
/// The item the box counts.
pub const TOTEM: &str = "minecraft:totem_of_undying";

/// Whether `id` is a totem of undying.
pub fn is_totem(id: &str) -> bool {
    id == TOTEM || id == "totem_of_undying"
}

/// The line and its colour: white with totems, red with none; nothing when `hide_none` and there are none.
pub fn line(count: u32, hide_none: bool) -> Option<(String, u32)> {
    match (count, hide_none) {
        (0, true) => None,
        (0, false) => Some(("Totems 0".to_owned(), 0xFF_5555)),
        (n, _) => Some((format!("Totems {n}"), INK)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_totems_count() {
        assert!(is_totem("minecraft:totem_of_undying"));
        assert!(is_totem("totem_of_undying"));
        assert!(!is_totem("minecraft:diamond"));
    }

    #[test]
    fn the_line_goes_red_at_zero_and_can_hide() {
        assert_eq!(line(3, false), Some(("Totems 3".into(), INK)));
        assert_eq!(line(0, false), Some(("Totems 0".into(), 0xFF_5555)));
        assert_eq!(line(0, true), None);
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
