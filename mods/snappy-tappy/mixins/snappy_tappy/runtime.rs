// Snappy Tappy, a native Horizon mod: When you hold two opposite movement keys, the one pressed last wins, so you switch direction at once.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_snappy_tappy`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "snappy_tappy";
pub const NAME: &str = "Snappy Tappy";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "When you hold two opposite movement keys, the one pressed last wins, so you switch direction at once.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] =
    &[("sideways", "A and D", true), ("forwards", "W and S", true)];

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

use std::cell::Cell;

/// Which key of a pair went down last.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Last {
    #[default]
    None,
    First,
    Second,
}

thread_local! {
    static SIDEWAYS: Cell<Last> = const { Cell::new(Last::None) };
    static FORWARDS: Cell<Last> = const { Cell::new(Last::None) };
}

/// A movement key went down or up, by its vanilla name (`key.keyboard.a`).
pub fn pressed(key: &str, down: bool) {
    let (cell, which) = match key {
        "key.keyboard.a" => (&SIDEWAYS, Last::First),
        "key.keyboard.d" => (&SIDEWAYS, Last::Second),
        "key.keyboard.w" => (&FORWARDS, Last::First),
        "key.keyboard.s" => (&FORWARDS, Last::Second),
        _ => return,
    };
    cell.with(|c| {
        if down {
            c.set(which);
        } else if c.get() == which {
            c.set(Last::None);
        }
    });
}

/// The last-pressed state of the A/D pair and the W/S pair.
pub fn last() -> (Last, Last) {
    (SIDEWAYS.with(Cell::get), FORWARDS.with(Cell::get))
}

/// Which of a pair counts as held: both held means the last pressed one only.
pub fn resolve(first: bool, second: bool, last: Last) -> (bool, bool) {
    match (first, second) {
        (true, true) => match last {
            Last::Second => (false, true),
            _ => (true, false),
        },
        other => other,
    }
}

/// Forgets what was pressed (tests only).
#[cfg(test)]
pub fn forget_keys() {
    SIDEWAYS.with(|c| c.set(Last::None));
    FORWARDS.with(|c| c.set(Last::None));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_last_pressed_of_two_held_keys_wins() {
        assert_eq!(resolve(true, true, Last::First), (true, false));
        assert_eq!(resolve(true, true, Last::Second), (false, true));
        assert_eq!(
            resolve(true, true, Last::None),
            (true, false),
            "unknown order: the first key"
        );
        assert_eq!(
            resolve(true, false, Last::Second),
            (true, false),
            "one key held is just held"
        );
        assert_eq!(resolve(false, false, Last::First), (false, false));
    }

    #[test]
    fn presses_and_releases_are_tracked_per_pair() {
        forget_keys();
        pressed("key.keyboard.a", true);
        pressed("key.keyboard.d", true);
        assert_eq!(last(), (Last::Second, Last::None));
        pressed("key.keyboard.d", false);
        assert_eq!(
            last().0,
            Last::None,
            "releasing the newest leaves the older one to carry on"
        );
        pressed("key.keyboard.w", true);
        pressed("key.keyboard.s", true);
        pressed("key.keyboard.a", true);
        assert_eq!(last(), (Last::First, Last::Second));
        pressed("key.keyboard.space", true);
        assert_eq!(
            last(),
            (Last::First, Last::Second),
            "other keys do not matter"
        );
        forget_keys();
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
