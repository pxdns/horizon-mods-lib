// Sprint Indicator, a native Horizon mod: Shows Sprinting, Walking or Sneaking in a corner of the screen while you move.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_sprint_indicator`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "sprint_indicator";
pub const NAME: &str = "Sprint Indicator";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Shows Sprinting, Walking or Sneaking in a corner of the screen while you move.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("walking", "Show walking", true),
    ("idle", "Show standing still", false),
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
/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "sprint";

/// What the player is doing, from the held movement keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Keys {
    pub forward: bool,
    pub left: bool,
    pub back: bool,
    pub right: bool,
    pub sprint: bool,
    pub sneak: bool,
}

impl Keys {
    pub fn moving(&self) -> bool {
        self.forward || self.left || self.back || self.right
    }
}

/// The label to show, or nothing.
pub fn status(keys: Keys, walking: bool, idle: bool) -> Option<&'static str> {
    if keys.sneak {
        Some("Sneaking")
    } else if keys.sprint && keys.moving() {
        Some("Sprinting")
    } else if keys.moving() {
        walking.then_some("Walking")
    } else {
        idle.then_some("Standing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sneaking_wins_then_sprinting_then_walking() {
        let k = |forward, sprint, sneak| Keys {
            forward,
            sprint,
            sneak,
            ..Keys::default()
        };
        assert_eq!(status(k(true, true, true), true, true), Some("Sneaking"));
        assert_eq!(status(k(true, true, false), true, true), Some("Sprinting"));
        assert_eq!(status(k(true, false, false), true, true), Some("Walking"));
        assert_eq!(status(k(true, false, false), false, true), None);
    }

    #[test]
    fn sprint_without_moving_is_standing() {
        let keys = Keys {
            sprint: true,
            ..Keys::default()
        };
        assert_eq!(status(keys, true, false), None);
        assert_eq!(status(keys, true, true), Some("Standing"));
        assert!(!keys.moving());
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
