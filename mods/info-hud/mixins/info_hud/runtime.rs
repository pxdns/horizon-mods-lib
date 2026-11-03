// Info HUD, a native Horizon mod: Your speed, the way you face and your dimension in a corner of the screen, in the spirit of MiniHUD's info lines.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_info_hud`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "info_hud";
pub const NAME: &str = "Info HUD";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Your speed, the way you face and your dimension in a corner of the screen, in the spirit of MiniHUD's info lines.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("speed", "Speed", true),
    ("facing", "Facing", true),
    ("dimension", "Dimension", false),
    ("angles", "Yaw and pitch", false),
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

use std::cell::Cell as StateCell;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "info";
/// The game ticks 20 times a second.
const TICKS_PER_SECOND: f64 = 20.0;
/// A jump this far in one tick is a teleport, not speed.
const TELEPORT: f64 = 20.0;

thread_local! {
    static LAST: StateCell<Option<(f64, f64)>> = const { StateCell::new(None) };
    static SPEED: StateCell<f64> = const { StateCell::new(0.0) };
}

/// One game tick at `(x, z)`: the horizontal speed in blocks per second, smoothed over about a quarter second.
pub fn tick(x: f64, z: f64) {
    let moved = LAST.with(|last| {
        let moved = last.get().map(|(px, pz)| (x - px).hypot(z - pz));
        last.set(Some((x, z)));
        moved
    });
    let now = match moved {
        Some(d) if d < TELEPORT => d * TICKS_PER_SECOND,
        _ => 0.0,
    };
    SPEED.with(|s| s.set(s.get() * 0.75 + now * 0.25));
}

/// Forgets the last position (no world).
pub fn reset() {
    LAST.with(|l| l.set(None));
    SPEED.with(|s| s.set(0.0));
}

/// The smoothed speed, blocks per second.
pub fn speed() -> f64 {
    SPEED.with(StateCell::get)
}

/// Direction.fromYRot: yaw 0 faces south, 90 west.
pub fn facing(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South (+Z)", "West (-X)", "North (-Z)", "East (+X)"][i as usize]
}

/// A yaw as vanilla shows it: -180 to 180.
pub fn wrap(yaw: f32) -> f32 {
    let y = yaw.rem_euclid(360.0);
    if y > 180.0 { y - 360.0 } else { y }
}

/// The box's lines for the options that are on.
pub fn lines(speed: f64, yaw: f32, pitch: f32, dimension: &str, show: [bool; 4]) -> Vec<String> {
    let mut out = Vec::new();
    if show[0] {
        out.push(format!("Speed {speed:.1} b/s"));
    }
    if show[1] {
        out.push(format!("Facing {}", facing(yaw)));
    }
    if show[2] {
        out.push(dimension.trim_start_matches("minecraft:").replace('_', " "));
    }
    if show[3] {
        out.push(format!("Yaw {:.1} Pitch {:.1}", wrap(yaw), pitch));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_settles_on_the_walking_pace() {
        reset();
        // Walking: 0.2158 blocks a tick = 4.3 b/s.
        for i in 0..40 {
            tick(f64::from(i) * 0.2158, 0.0);
        }
        assert!((speed() - 4.316).abs() < 0.05, "{}", speed());
        // Standing still decays to nothing.
        for _ in 0..60 {
            tick(40.0 * 0.2158, 0.0);
        }
        assert!(speed() < 0.01);
        reset();
        assert_eq!(speed(), 0.0);
    }

    #[test]
    fn a_teleport_is_not_speed() {
        reset();
        tick(0.0, 0.0);
        tick(500.0, 500.0);
        assert_eq!(speed(), 0.0);
    }

    #[test]
    fn facing_and_wrap_follow_vanilla() {
        assert_eq!(facing(0.0), "South (+Z)");
        assert_eq!(facing(90.0), "West (-X)");
        assert_eq!(facing(-180.0), "North (-Z)");
        assert_eq!(facing(-90.0), "East (+X)");
        assert_eq!(wrap(270.0), -90.0);
        assert_eq!(wrap(-190.0), 170.0);
    }

    #[test]
    fn lines_follow_the_toggles() {
        assert_eq!(
            lines(
                4.32,
                180.0,
                10.0,
                "minecraft:the_nether",
                [true, true, true, true]
            ),
            [
                "Speed 4.3 b/s",
                "Facing North (-Z)",
                "the nether",
                "Yaw 180.0 Pitch 10.0"
            ]
        );
        assert!(lines(0.0, 0.0, 0.0, "x", [false; 4]).is_empty());
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
