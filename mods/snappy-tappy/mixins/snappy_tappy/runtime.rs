// SnappyTappy, a native Horizon mod: Last-input priority for movement keys: pressing D while holding A moves you right at once, then back to A when you let go.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_snappy_tappy`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "snappy_tappy";
pub const NAME: &str = "SnappyTappy";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Last-input priority for movement keys: pressing D while holding A moves you right at once, then back to A when you let go.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("enabled", "Enabled", true)
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

use std::cell::Cell;
use std::collections::HashMap;

/// `key.snappy_tappy.toggle`, default `5` (GLFW 53).
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.5";

thread_local! {
    /// The tick each key last went down, by its name (`key.keyboard.a`), as `pressTick`.
    static PRESSED: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static TICK: Cell<u64> = const { Cell::new(0) };
}

/// One game tick passed (`InputInterceptor.tick`).
pub fn next_tick() {
    TICK.with(|t| t.set(t.get() + 1));
}

/// A key went down (`down`) or up: a press records the tick.
pub fn pressed(key: &str, down: bool) {
    let tick = TICK.with(Cell::get);
    PRESSED.with_borrow_mut(|p| {
        if down {
            p.entry(key.to_owned()).or_insert(tick);
        } else {
            p.remove(key);
        }
    });
}

/// When `key` went down, 0 when it is not known.
pub fn press_tick(key: &str) -> u64 {
    PRESSED.with_borrow(|p| p.get(key).copied().unwrap_or(0))
}

/// `resolvePair`: of two opposite keys held together, the one pressed later counts; at the same tick the first (left, forward) does.
/// Returns (first counts as held, second counts as held). One key held is just held.
pub fn resolve(first_held: bool, second_held: bool, first_tick: u64, second_tick: u64) -> (bool, bool) {
    if first_held && second_held {
        if first_tick >= second_tick { (true, false) } else { (false, true) }
    } else {
        (first_held, second_held)
    }
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget() {
    PRESSED.with_borrow_mut(HashMap::clear);
    TICK.with(|t| t.set(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_later_press_of_two_held_keys_wins() {
        assert_eq!(resolve(true, true, 5, 9), (false, true));
        assert_eq!(resolve(true, true, 9, 5), (true, false));
        assert_eq!(resolve(true, true, 7, 7), (true, false), "the same tick: the first key");
        assert_eq!(resolve(true, false, 9, 1), (true, false), "one key held is just held");
        assert_eq!(resolve(false, false, 0, 0), (false, false));
    }

    #[test]
    fn presses_are_timed_and_a_release_forgets_them() {
        forget();
        next_tick();
        pressed("key.keyboard.a", true);
        next_tick();
        next_tick();
        pressed("key.keyboard.d", true);
        assert_eq!((press_tick("key.keyboard.a"), press_tick("key.keyboard.d")), (1, 3));
        pressed("key.keyboard.d", false);
        assert_eq!(press_tick("key.keyboard.d"), 0);
        pressed("key.keyboard.a", true);
        assert_eq!(press_tick("key.keyboard.a"), 1, "a key repeat does not move its press");
        forget();
    }

    #[test]
    fn a_pair_switches_to_the_newer_key_and_back() {
        forget();
        next_tick();
        pressed("key.keyboard.a", true);
        next_tick();
        pressed("key.keyboard.d", true);
        let now = || resolve(true, true, press_tick("key.keyboard.a"), press_tick("key.keyboard.d"));
        assert_eq!(now(), (false, true), "D pressed last: right");
        pressed("key.keyboard.d", false);
        assert_eq!(resolve(true, false, press_tick("key.keyboard.a"), 0), (true, false), "back to A");
        forget();
    }

    #[test]
    fn the_toggle_key_is_five() {
        assert_eq!(DEFAULT_KEY, "key.keyboard.5");
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
