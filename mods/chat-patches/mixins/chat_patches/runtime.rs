// Chat Patches, a native Horizon mod: Timestamps in front of chat lines, and an option to drop immediate repeats.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_chat_patches`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "chat_patches";
pub const NAME: &str = "Chat Patches";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Timestamps in front of chat lines, and an option to drop immediate repeats.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("timestamps", "Timestamps", true),
    ("seconds", "Seconds in the timestamp", false),
    ("repeats", "Drop immediate repeats", false),
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

/// A message repeating the last one within this many ticks (five seconds) is a repeat.
pub const REPEAT_TICKS: u64 = 100;

thread_local! {
    static STAMPS: Cell<bool> = const { Cell::new(true) };
    static SECONDS: Cell<bool> = const { Cell::new(false) };
    static REPEATS: Cell<bool> = const { Cell::new(false) };
    /// True while the mod adds a message again with its stamp.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
    static LAST: RefCell<Option<(String, u64)>> = const { RefCell::new(None) };
}

/// Sets what the mod does, from the player's settings (the game tells the chat each tick).
pub fn configure(stamps: bool, seconds: bool, repeats: bool) {
    STAMPS.with(|v| v.set(stamps));
    SECONDS.with(|v| v.set(seconds));
    REPEATS.with(|v| v.set(repeats));
}

pub fn stamps_on() -> bool {
    STAMPS.with(Cell::get)
}

pub fn seconds_on() -> bool {
    SECONDS.with(Cell::get)
}

pub fn repeats_on() -> bool {
    REPEATS.with(Cell::get)
}

/// `[14:32]` or `[14:32:07]`, with a trailing space.
pub fn stamp(hour: u32, minute: u32, second: u32, with_seconds: bool) -> String {
    if with_seconds {
        format!("[{hour:02}:{minute:02}:{second:02}] ")
    } else {
        format!("[{hour:02}:{minute:02}] ")
    }
}

/// Marks the nested call; false when the call is the nested one.
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

/// Whether `text` at `ticks` repeats the previous message, and remembers it as the previous one either way.
pub fn repeat(text: &str, ticks: u64) -> bool {
    LAST.with_borrow_mut(|last| {
        let same = last
            .as_ref()
            .is_some_and(|(t, at)| t == text && ticks.saturating_sub(*at) <= REPEAT_TICKS);
        *last = Some((text.to_owned(), ticks));
        same
    })
}

/// Forgets the previous message (tests).
#[cfg(test)]
pub fn forget() {
    LAST.with_borrow_mut(|l| *l = None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamps_are_zero_padded() {
        assert_eq!(stamp(9, 5, 3, false), "[09:05] ");
        assert_eq!(stamp(14, 32, 7, true), "[14:32:07] ");
    }

    #[test]
    fn the_nested_call_is_not_stamped_again() {
        leave();
        assert!(enter());
        assert!(!enter(), "nested");
        leave();
        assert!(enter());
        leave();
    }

    #[test]
    fn a_repeat_is_the_same_text_soon_after() {
        forget();
        assert!(!repeat("hello", 10), "the first one is new");
        assert!(repeat("hello", 50), "same text, 2 seconds later");
        assert!(!repeat("bye", 60));
        assert!(!repeat("hello", 70), "something else came between");
        assert!(!repeat("hello", 70 + REPEAT_TICKS + 1), "too long ago");
        forget();
    }

    #[test]
    fn the_switches_work() {
        configure(false, true, true);
        assert!(!stamps_on() && seconds_on() && repeats_on());
        configure(true, false, false);
        assert!(stamps_on() && !seconds_on() && !repeats_on());
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
