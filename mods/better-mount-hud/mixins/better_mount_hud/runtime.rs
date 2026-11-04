// Better Mount HUD, a native Horizon mod: Numbers for the mount you ride: health, speed in blocks per second, jump height and jump charge.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_better_mount_hud`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "better_mount_hud";
pub const NAME: &str = "Better Mount HUD";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Numbers for the mount you ride: health, speed in blocks per second, jump height and jump charge.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("speed", "Speed", true),
    ("jump", "Jump height and charge", true),
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
pub const COMMAND: &str = "mount";

/// A mount's health line; red at a quarter or less.
pub fn health_line(health: f64, max: f64) -> (String, u32) {
    let ink = if max > 0.0 && health / max <= 0.25 {
        0xFF_5555
    } else {
        INK
    };
    (format!("Health {:.1} / {:.1}", health.max(0.0), max), ink)
}

/// Blocks per second from the horizontal move of one tick.
pub fn speed(dx: f64, dz: f64) -> f64 {
    dx.hypot(dz) * 20.0
}

pub fn speed_line(dx: f64, dz: f64) -> (String, u32) {
    (format!("Speed {:.1} b/s", speed(dx, dz)), INK)
}

/// How high a horse with this jump strength jumps, in blocks (the polynomial fitted to the game's physics).
pub fn jump_height(strength: f64) -> f64 {
    let s = strength.clamp(0.0, 2.0);
    (-0.181_758_495_2 * s * s * s + 3.689_713_992 * s * s + 2.128_599_134 * s - 0.343_930_367)
        .max(0.0)
}

pub fn jump_line(strength: f64, charge: f32) -> (String, u32) {
    let pct = (charge.clamp(0.0, 1.0) * 100.0).round() as u32;
    (
        format!("Jump {:.2} blocks, charge {pct}%", jump_height(strength)),
        INK,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_goes_red_at_a_quarter() {
        assert_eq!(
            health_line(30.0, 30.0),
            ("Health 30.0 / 30.0".to_owned(), INK)
        );
        assert_eq!(health_line(7.5, 30.0).1, 0xFF_5555);
        assert_eq!(health_line(-1.0, 30.0).0, "Health 0.0 / 30.0");
    }

    #[test]
    fn speed_is_blocks_per_second() {
        assert!((speed(0.3, 0.4) - 10.0).abs() < 1e-9);
        assert_eq!(speed_line(0.0, 0.0).0, "Speed 0.0 b/s");
    }

    #[test]
    fn a_stronger_horse_jumps_higher() {
        assert!(jump_height(0.5) < jump_height(0.8));
        assert!(
            (jump_height(1.0) - 5.29).abs() < 0.05,
            "a top horse clears about five blocks"
        );
        assert_eq!(jump_height(-3.0), 0.0);
    }

    #[test]
    fn the_jump_line_shows_the_charge() {
        assert!(jump_line(0.7, 0.5).0.ends_with("charge 50%"));
        assert!(jump_line(0.7, 5.0).0.ends_with("charge 100%"));
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
