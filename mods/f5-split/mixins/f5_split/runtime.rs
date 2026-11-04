// F5 Split, a native Horizon mod: Separate keys for the back and the front third-person view, as a toggle or while held.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_f5_split`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "f5_split";
pub const NAME: &str = "F5 Split";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Separate keys for the back and the front third-person view, as a toggle or while held.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[("hold", "Hold the key instead of toggling", false)];

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

pub const BACK_KEY: &str = "back";
pub const BACK_DEFAULT: &str = "key.keyboard.f6";
pub const FRONT_KEY: &str = "front";
pub const FRONT_DEFAULT: &str = "key.keyboard.f7";

/// The three views of the game's camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    First,
    Back,
    Front,
}

/// The view after `target`'s key went down (`down`) or up. A toggle changes on press only: the same key again
/// returns to first person. Held, the view lasts while the key is down.
pub fn after(current: View, target: View, down: bool, hold: bool) -> View {
    match (hold, down) {
        (false, true) if current == target => View::First,
        (false, true) => target,
        (false, false) => current,
        (true, true) => target,
        (true, false) if current == target => View::First,
        (true, false) => current,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_toggle_flips_on_press_and_ignores_release() {
        assert_eq!(after(View::First, View::Back, true, false), View::Back);
        assert_eq!(
            after(View::Back, View::Back, true, false),
            View::First,
            "the same key again"
        );
        assert_eq!(
            after(View::Back, View::Front, true, false),
            View::Front,
            "the other key switches"
        );
        assert_eq!(
            after(View::Back, View::Back, false, false),
            View::Back,
            "release does nothing"
        );
    }

    #[test]
    fn held_lasts_while_the_key_is_down() {
        assert_eq!(after(View::First, View::Front, true, true), View::Front);
        assert_eq!(after(View::Front, View::Front, false, true), View::First);
        assert_eq!(
            after(View::Back, View::Front, false, true),
            View::Back,
            "letting go of a key that is not the current view changes nothing"
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
