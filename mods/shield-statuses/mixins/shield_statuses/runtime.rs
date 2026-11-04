// Shield Statuses, a native Horizon mod: Your shield's state as a coloured line: ready, blocking, or disabled with the seconds left.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_shield_statuses`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "shield_statuses";
pub const NAME: &str = "Shield Statuses";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Your shield's state as a coloured line: ready, blocking, or disabled with the seconds left.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] =
    &[("shown", "Shown", true), ("hide", "Hide when ready", false)];

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
pub const COMMAND: &str = "shield";

/// A shield's state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    Blocking,
    /// Disabled by an axe: ticks left.
    Disabled(u32),
}

/// The state: a cooldown wins over blocking, which wins over ready.
pub fn status(blocking: bool, cooldown_ticks: u32) -> Status {
    if cooldown_ticks > 0 {
        Status::Disabled(cooldown_ticks)
    } else if blocking {
        Status::Blocking
    } else {
        Status::Ready
    }
}

/// The line and its colour; none when `hide_ready` and the shield is ready.
pub fn line(status: Status, hide_ready: bool) -> Option<(String, u32)> {
    match status {
        Status::Ready if hide_ready => None,
        Status::Ready => Some(("Shield: ready".to_owned(), 0x55_FF55)),
        Status::Blocking => Some(("Shield: blocking".to_owned(), 0x55_FFFF)),
        Status::Disabled(ticks) => Some((
            format!("Shield: disabled {:.1} s", f64::from(ticks) / 20.0),
            0xFF_5555,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_state_follows_cooldown_then_blocking() {
        assert_eq!(status(false, 0), Status::Ready);
        assert_eq!(status(true, 0), Status::Blocking);
        assert_eq!(status(false, 70), Status::Disabled(70));
        assert_eq!(
            status(true, 70),
            Status::Disabled(70),
            "a disabled shield is not blocking"
        );
    }

    #[test]
    fn lines_and_colours() {
        assert_eq!(
            line(Status::Ready, false),
            Some(("Shield: ready".into(), 0x55_FF55))
        );
        assert_eq!(line(Status::Ready, true), None);
        assert_eq!(
            line(Status::Blocking, true),
            Some(("Shield: blocking".into(), 0x55_FFFF))
        );
        assert_eq!(
            line(Status::Disabled(70), false),
            Some(("Shield: disabled 3.5 s".into(), 0xFF_5555))
        );
        assert_eq!(
            line(Status::Disabled(1), false).unwrap().0,
            "Shield: disabled 0.1 s"
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
