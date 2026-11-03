// FPS Display, a native Horizon mod: Frames per second in a corner of the screen, coloured by how smooth they are.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_fps_display`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "fps_display";
pub const NAME: &str = "FPS Display";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Frames per second in a corner of the screen, coloured by how smooth they are.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("slowest", "Slowest frame", false),
    ("low", "1% low", false),
    ("colour", "Colour by rate", true),
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
pub const INK: u32 = 0xFF_FFFF;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "fps";

/// The box's text colour: green from 60 fps, yellow from 30, red below; white when colouring is off.
pub fn ink(fps: u32, coloured: bool) -> u32 {
    match (coloured, fps) {
        (false, _) => INK,
        (true, 60..) => 0x55_FF55,
        (true, 30..=59) => 0xFF_FF55,
        (true, _) => 0xFF_5555,
    }
}

/// The box's lines: the rate, then the slowest frame and the 1% low when asked for.
pub fn lines(fps: u32, slowest: u32, low: u32, show_slowest: bool, show_low: bool) -> Vec<String> {
    let mut out = vec![format!("{fps} FPS")];
    if show_slowest {
        out.push(format!("Slowest {slowest}"));
    }
    if show_low {
        out.push(format!("1% low {low}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colours_follow_the_rate() {
        assert_eq!(ink(144, true), 0x55_FF55);
        assert_eq!(ink(60, true), 0x55_FF55);
        assert_eq!(ink(59, true), 0xFF_FF55);
        assert_eq!(ink(30, true), 0xFF_FF55);
        assert_eq!(ink(12, true), 0xFF_5555);
        assert_eq!(ink(12, false), INK);
    }

    #[test]
    fn lines_grow_with_the_options() {
        assert_eq!(lines(90, 70, 60, false, false), ["90 FPS"]);
        assert_eq!(
            lines(90, 70, 60, true, true),
            ["90 FPS", "Slowest 70", "1% low 60"]
        );
        assert_eq!(lines(90, 70, 60, false, true), ["90 FPS", "1% low 60"]);
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
