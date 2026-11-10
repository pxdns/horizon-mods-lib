// Custom HUD, a native Horizon mod: One box with the lines you pick: FPS, coordinates, facing and clicks per second, on the side you choose.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_custom_hud`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "custom_hud";
pub const NAME: &str = "Custom HUD";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "One box with the lines you pick: FPS, coordinates, facing and clicks per second, on the side you choose.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("right", "Right side", false),
    ("fps", "FPS", true),
    ("coords", "Coordinates", true),
    ("facing", "Facing", true),
    ("cps", "Clicks per second", false)
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
pub const COMMAND: &str = "hud";

/// What the box can show.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Reading {
    pub fps: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub clicks: u32,
}

/// Direction.fromYRot: yaw 0 faces south, 90 west.
pub fn facing(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South", "West", "North", "East"][i as usize]
}

/// The box's lines: `show` is fps, coordinates, facing, clicks.
pub fn lines(r: Reading, show: [bool; 4]) -> Vec<String> {
    let mut out = Vec::new();
    if show[0] {
        out.push(format!("FPS {}", r.fps));
    }
    if show[1] {
        out.push(format!("XYZ {:.1} / {:.1} / {:.1}", r.x, r.y, r.z));
    }
    if show[2] {
        out.push(format!("Facing {}", facing(r.yaw)));
    }
    if show[3] {
        out.push(format!("CPS {}", r.clicks));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_picked_lines_show() {
        let r = Reading { fps: 120, x: 1.0, y: 64.0, z: -3.5, yaw: 90.0, clicks: 7 };
        assert_eq!(lines(r, [true, false, false, false]), ["FPS 120"]);
        assert_eq!(
            lines(r, [true, true, true, true]),
            ["FPS 120", "XYZ 1.0 / 64.0 / -3.5", "Facing West", "CPS 7"]
        );
        assert!(lines(r, [false; 4]).is_empty());
    }

    #[test]
    fn facing_is_the_compass_point() {
        assert_eq!(facing(0.0), "South");
        assert_eq!(facing(-90.0), "East");
        assert_eq!(facing(180.0), "North");
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
