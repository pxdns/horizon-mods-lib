// Freelook, a native Horizon mod: Hold a key and move the mouse to look around without turning your player.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_freelook`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "freelook";
pub const NAME: &str = "Freelook";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Hold a key and move the mouse to look around without turning your player.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("toggle", "Toggle instead of hold", false),
    ("invert", "Invert pitch", false)
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

/// The key's name in Controls, and its default.
pub const KEY: &str = "freelook";
pub const DEFAULT_KEY: &str = "key.keyboard.left.alt";

/// The view while freelook is on: yaw and pitch of the camera, in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
}

thread_local! {
    static VIEW: Cell<Option<View>> = const { Cell::new(None) };
    /// True while the camera function runs for the mod, so its nested call is the original.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// The freelook key went down or up; `yaw`/`pitch` are the player's now.
pub fn key(playing: bool, pressed: bool, toggle: bool, yaw: f32, pitch: f32) {
    let on = VIEW.with(Cell::get).is_some();
    let want = if toggle {
        if !pressed {
            return;
        }
        !on
    } else {
        pressed
    };
    if want && !on {
        if playing {
            VIEW.with(|v| v.set(Some(View { yaw, pitch })));
        }
    } else if !want && on {
        VIEW.with(|v| v.set(None));
    }
}

/// Whether freelook is on.
#[allow(dead_code)]
pub fn active() -> bool {
    VIEW.with(Cell::get).is_some()
}

/// Mouse deltas as `Entity.turn` takes them: 0.15 degrees per unit, pitch kept within 90. True when freelook took them.
pub fn turn(xo: f64, yo: f64, invert: bool) -> bool {
    VIEW.with(|v| match v.get() {
        Some(mut view) => {
            let pitch_delta = (yo as f32) * 0.15 * if invert { -1.0 } else { 1.0 };
            view.yaw += (xo as f32) * 0.15;
            view.pitch = (view.pitch + pitch_delta).clamp(-90.0, 90.0);
            v.set(Some(view));
            true
        }
        None => false,
    })
}

/// The angles to render from, and marks the nested call; None when freelook is off or the call is already nested.
pub fn enter() -> Option<(f32, f32)> {
    if INSIDE.with(Cell::get) {
        return None;
    }
    let view = VIEW.with(Cell::get)?;
    INSIDE.with(|i| i.set(true));
    Some((view.yaw, view.pitch))
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// Turns freelook off (tests only).
#[cfg(test)]
pub fn forget() {
    VIEW.with(|v| v.set(None));
    INSIDE.with(|i| i.set(false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_starts_on_press_and_stops_on_release() {
        forget();
        key(true, true, false, 30.0, 10.0);
        assert!(active());
        assert_eq!(enter(), Some((30.0, 10.0)));
        leave();
        key(false, false, false, 0.0, 0.0);
        assert!(!active(), "released on any screen");
        forget();
    }

    #[test]
    fn a_screen_cannot_start_it() {
        forget();
        key(false, true, false, 0.0, 0.0);
        assert!(!active());
        forget();
    }

    #[test]
    fn toggle_flips_on_each_press_and_ignores_release() {
        forget();
        key(true, true, true, 0.0, 0.0);
        key(true, false, true, 0.0, 0.0);
        assert!(active());
        key(true, true, true, 0.0, 0.0);
        assert!(!active());
        forget();
    }

    #[test]
    fn the_mouse_turns_the_view_not_the_player() {
        forget();
        assert!(!turn(10.0, 10.0, false), "off: the player turns");
        key(true, true, false, 0.0, 0.0);
        assert!(turn(100.0, 100.0, false));
        let (yaw, pitch) = enter().unwrap();
        assert!((yaw - 15.0).abs() < 1e-4 && (pitch - 15.0).abs() < 1e-4);
        leave();
        assert!(turn(0.0, 1000.0, false));
        assert_eq!(enter().unwrap().1, 90.0, "pitch stops at 90");
        leave();
        assert!(turn(0.0, 10_000.0, true));
        assert_eq!(enter().unwrap().1, -90.0, "inverted pitch goes the other way");
        leave();
        forget();
    }

    #[test]
    fn the_nested_camera_call_runs_the_original() {
        forget();
        key(true, true, false, 5.0, 6.0);
        assert!(enter().is_some());
        assert_eq!(enter(), None, "nested");
        leave();
        assert!(enter().is_some());
        leave();
        forget();
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
