// Minimap Plus, a native Horizon mod: Turns Horizon's minimap on and shapes it: corner, size, zoom keys, a big-map key, turning and round or square.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_minimap_plus`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "minimap_plus";
pub const NAME: &str = "Minimap Plus";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Turns Horizon's minimap on and shapes it: corner, size, zoom keys, a big-map key, turning and round or square.";

use std::cell::Cell;

/// The command that hides or shows the map.
pub const COMMAND: &str = "minimap";
pub const BIG_KEY: &str = "big";
pub const BIG_DEFAULT: &str = "key.keyboard.m";
pub const IN_KEY: &str = "zoom_in";
pub const IN_DEFAULT: &str = "key.keyboard.equal";
pub const OUT_KEY: &str = "zoom_out";
pub const OUT_DEFAULT: &str = "key.keyboard.minus";

/// Side lengths in GUI pixels the second button cycles through.
pub const SIZES: [u32; 5] = [64, 96, 128, 160, 192];
/// Blocks per map pixel: smaller is closer. Horizon keeps it between 0.5 and 4.
pub const ZOOMS: [f32; 5] = [0.5, 1.0, 2.0, 3.0, 4.0];
/// The corners, in the order the fourth button cycles them (the debug screen takes the top left).
pub const CORNERS: [&str; 4] = ["Top right", "Bottom right", "Bottom left", "Top left"];
/// The big map: the largest side Horizon allows, at the widest view.
pub const BIG_SIZE: f32 = 256.0;
pub const BIG_ZOOM: f32 = 4.0;

thread_local! {
    static BIG: Cell<bool> = const { Cell::new(false) };
}

pub fn big() -> bool {
    BIG.with(Cell::get)
}

pub fn toggle_big() {
    BIG.with(|b| b.set(!b.get()));
}

/// The stored settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub shown: bool,
    pub size: u32,
    pub zoom: f32,
    pub corner: usize,
    pub rotate: bool,
    pub round: bool,
}

fn flag(get: &impl Fn(&str) -> Option<String>, key: &str, default: bool) -> bool {
    match get(key).as_deref().map(str::trim) {
        Some("true") => true,
        Some("false") => false,
        _ => default,
    }
}

/// The stored settings; anything that does not read falls back to the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> Config {
    let size = get("size").and_then(|v| v.trim().parse().ok()).filter(|v| SIZES.contains(v)).unwrap_or(128);
    let zoom = get("zoom").and_then(|v| v.trim().parse::<f32>().ok()).filter(|v| ZOOMS.contains(v)).unwrap_or(1.0);
    let corner = get("corner").and_then(|v| v.trim().parse().ok()).filter(|v| *v < CORNERS.len()).unwrap_or(0);
    Config { shown: flag(&get, "shown", true), size, zoom, corner, rotate: flag(&get, "rotate", true), round: flag(&get, "round", false) }
}

/// What Horizon's minimap should look like: corner index, side, blocks per pixel, turning, round; none when hidden.
pub fn view(config: &Config, big: bool) -> Option<(usize, f32, f32, bool, bool)> {
    if !config.shown {
        return None;
    }
    let (size, zoom) = if big { (BIG_SIZE, BIG_ZOOM) } else { (config.size as f32, config.zoom) };
    Some((config.corner, size, zoom, config.rotate, config.round))
}

/// The next zoom step: closer (fewer blocks per pixel) or further; the ends stay.
pub fn step_zoom(current: f32, closer: bool) -> f32 {
    let at = ZOOMS.iter().position(|z| (*z - current).abs() < 1e-6).unwrap_or(1);
    let next = if closer { at.saturating_sub(1) } else { (at + 1).min(ZOOMS.len() - 1) };
    ZOOMS[next]
}

fn cycle<T: PartialEq + Copy>(list: &[T], current: T) -> T {
    let at = list.iter().position(|v| *v == current).unwrap_or(0);
    list[(at + 1) % list.len()]
}

fn on_off(value: bool) -> &'static str {
    if value { "on" } else { "off" }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let c = read(get);
    vec![
        format!("Minimap: {}", on_off(c.shown)),
        format!("Size: {} px", c.size),
        format!("Zoom: {} blocks per pixel", c.zoom),
        format!("Corner: {}", CORNERS[c.corner]),
        format!("Turns with you: {}", on_off(c.rotate)),
        format!("Round: {}", on_off(c.round)),
    ]
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let c = read(get);
    Some(match i {
        0 => ("shown", (!c.shown).to_string()),
        1 => ("size", cycle(&SIZES, c.size).to_string()),
        2 => ("zoom", cycle(&ZOOMS, c.zoom).to_string()),
        3 => ("corner", ((c.corner + 1) % CORNERS.len()).to_string()),
        4 => ("rotate", (!c.rotate).to_string()),
        5 => ("round", (!c.round).to_string()),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_defaults_are_a_turning_square_map_top_right() {
        let c = read(none);
        assert_eq!((c.shown, c.size, c.zoom, c.corner, c.rotate, c.round), (true, 128, 1.0, 0, true, false));
        assert_eq!(view(&c, false), Some((0, 128.0, 1.0, true, false)));
    }

    #[test]
    fn the_big_map_is_the_largest_and_widest() {
        let c = read(none);
        assert_eq!(view(&c, true), Some((0, 256.0, 4.0, true, false)));
        toggle_big();
        assert!(big());
        toggle_big();
        assert!(!big());
    }

    #[test]
    fn hidden_means_nothing_to_show() {
        let off = |k: &str| (k == "shown").then(|| "false".to_owned());
        assert_eq!(view(&read(off), false), None);
        assert_eq!(view(&read(off), true), None);
    }

    #[test]
    fn zoom_steps_stop_at_the_ends() {
        assert_eq!(step_zoom(1.0, true), 0.5);
        assert_eq!(step_zoom(0.5, true), 0.5);
        assert_eq!(step_zoom(4.0, false), 4.0);
        assert_eq!(step_zoom(2.0, false), 3.0);
        assert_eq!(step_zoom(7.0, false), 2.0, "an unknown value counts as the default step");
    }

    #[test]
    fn buttons_cycle_each_setting() {
        assert_eq!(click(none, 0), Some(("shown", "false".to_owned())));
        assert_eq!(click(none, 1), Some(("size", "160".to_owned())));
        assert_eq!(click(none, 2), Some(("zoom", "2".to_owned())));
        assert_eq!(click(none, 3), Some(("corner", "1".to_owned())));
        assert_eq!(click(none, 4), Some(("rotate", "false".to_owned())));
        assert_eq!(click(none, 5), Some(("round", "true".to_owned())));
        assert_eq!(click(none, 6), None);
        let last = |k: &str| (k == "corner").then(|| "3".to_owned());
        assert_eq!(click(last, 3), Some(("corner", "0".to_owned())), "the corners wrap");
    }

    #[test]
    fn junk_settings_fall_back() {
        let junk = |k: &str| Some(if k == "zoom" { "9".to_owned() } else { "banana".to_owned() });
        assert_eq!(read(junk), read(none));
        assert_eq!(labels(none)[2], "Zoom: 1 blocks per pixel");
    }
}
