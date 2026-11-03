// Chat Animation, a native Horizon mod: New chat lines slide up into place instead of appearing at once.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_chat_animation`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "chat_animation";
pub const NAME: &str = "Chat Animation";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "New chat lines slide up into place instead of appearing at once.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[("slide", "Slide in", true)];

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

/// How long a line slides, in ticks.
pub const SLIDE_TICKS: u64 = 6;

thread_local! {
    static ON: Cell<bool> = const { Cell::new(true) };
}

/// Switches the slide on or off (the game tells the chat each tick).
pub fn configure(on: bool) {
    ON.with(|o| o.set(on));
}

/// Pixels a line added at `added` is still below its place at tick `ticks`: the whole line height at first, easing to nothing.
pub fn slide(ticks: u64, added: u64, line_height: i32) -> i32 {
    if !ON.with(Cell::get) {
        return 0;
    }
    let age = ticks.saturating_sub(added);
    if age >= SLIDE_TICKS {
        return 0;
    }
    let left = 1.0 - age as f32 / SLIDE_TICKS as f32;
    // Ease out: fast at the start, settling at the end.
    (left * left * line_height as f32).round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_line_starts_a_full_line_below_and_settles() {
        configure(true);
        assert_eq!(slide(10, 10, 9), 9);
        assert!(slide(11, 10, 9) < 9 && slide(11, 10, 9) > slide(12, 10, 9));
        assert_eq!(slide(10 + SLIDE_TICKS, 10, 9), 0);
        assert_eq!(slide(500, 10, 9), 0, "old lines do not move");
        assert_eq!(slide(5, 10, 9), 9, "a line from the future counts as new");
    }

    #[test]
    fn the_slide_can_be_switched_off() {
        configure(false);
        assert_eq!(slide(10, 10, 9), 0);
        configure(true);
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
