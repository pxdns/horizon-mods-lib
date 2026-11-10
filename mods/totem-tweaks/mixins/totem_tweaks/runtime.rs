// Totem Tweaks, a native Horizon mod: Customise the totem of undying pop animation: off, speed, size, spin, rocking and position.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_totem_tweaks`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "totem_tweaks";
pub const NAME: &str = "Totem Tweaks";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Customise the totem of undying pop animation: off, speed, size, spin, rocking and position.";

use std::cell::Cell;

/// The pop lasts 40 ticks.
pub const LENGTH: i32 = 40;
/// Speeds the second button cycles through, as a multiple of vanilla's.
pub const SPEEDS: [f32; 5] = [0.5, 1.0, 1.5, 2.0, 3.0];
/// Sizes the third button cycles through, in percent.
pub const SIZES: [u32; 6] = [50, 75, 100, 125, 150, 200];

/// The player's choices.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub pop: bool,
    pub speed: f32,
    pub size: u32,
    pub spin: bool,
    pub rock: bool,
    pub swing: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { pop: true, speed: 1.0, size: 100, spin: true, rock: true, swing: true }
    }
}

thread_local! {
    static CURRENT: Cell<Settings> = const { Cell::new(Settings { pop: true, speed: 1.0, size: 100, spin: true, rock: true, swing: true }) };
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// Sets what the animation uses and whether the mod is on (the game tells the renderer each tick).
pub fn configure(active: bool, settings: Settings) {
    ACTIVE.with(|a| a.set(active));
    CURRENT.with(|c| c.set(settings));
}

pub fn active() -> bool {
    ACTIVE.with(Cell::get)
}

pub fn current() -> Settings {
    CURRENT.with(Cell::get)
}

fn flag(get: &impl Fn(&str) -> Option<String>, key: &str) -> bool {
    get(key).as_deref().map(str::trim) != Some("false")
}

/// The stored settings; anything that does not read is the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> Settings {
    let speed = get("speed").and_then(|v| v.trim().parse::<f32>().ok()).filter(|v| SPEEDS.contains(v)).unwrap_or(1.0);
    let size = get("size").and_then(|v| v.trim().parse().ok()).filter(|v| SIZES.contains(v)).unwrap_or(100);
    Settings { pop: flag(&get, "pop"), speed, size, spin: flag(&get, "spin"), rock: flag(&get, "rock"), swing: flag(&get, "swing") }
}

fn on_off(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let s = read(get);
    vec![
        format!("Pop animation: {}", on_off(s.pop)),
        format!("Speed: {}x", s.speed),
        format!("Size: {} %", s.size),
        format!("Spin: {}", on_off(s.spin)),
        format!("Rocking: {}", on_off(s.rock)),
        format!("Sideways swing: {}", on_off(s.swing)),
    ]
}

fn next<T: PartialEq + Copy>(list: &[T], current: T) -> T {
    list[(list.iter().position(|v| *v == current).unwrap_or(0) + 1) % list.len()]
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let s = read(get);
    Some(match i {
        0 => ("pop", (!s.pop).to_string()),
        1 => ("speed", next(&SPEEDS, s.speed).to_string()),
        2 => ("size", next(&SIZES, s.size).to_string()),
        3 => ("spin", (!s.spin).to_string()),
        4 => ("rock", (!s.rock).to_string()),
        5 => ("swing", (!s.swing).to_string()),
        _ => return None,
    })
}

/// Where the totem is this frame: position in view space, a uniform scale, and the turns in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub scale: f32,
    pub spin: f32,
    pub rock_x: f32,
    pub rock_z: f32,
}

/// `ScreenEffectRenderer.renderItemActivationAnimation` with the player's settings: the same quintic brings the totem from 10 blocks
/// out to 1 and back; `speed` plays it faster, and past its end (or with the pop off) the totem is not drawn.
pub fn pose(ticks: i32, partial: f32, off_x: f32, off_y: f32, aspect: f32, s: &Settings) -> Pose {
    let hidden = Pose { x: 0.0, y: 0.0, z: -10.0, scale: 0.0, spin: 0.0, rock_x: 0.0, rock_z: 0.0 };
    let t = ((LENGTH - ticks) as f32 + partial) / LENGTH as f32 * s.speed;
    if !s.pop || t >= 1.0 {
        return hidden;
    }
    let (t2, t3) = (t * t, t * t * t);
    let curve = 10.25 * t3 * t2 - 24.95 * t2 * t2 + 25.5 * t3 - 13.8 * t2 + 4.0 * t;
    let angle = curve * std::f32::consts::PI;
    let swing = if s.swing { (angle * 2.0).sin().abs() } else { 0.0 };
    let rock = if s.rock { 6.0 * (t * 8.0).cos() } else { 0.0 };
    Pose {
        x: off_x * 0.3 * aspect * swing,
        y: off_y * 0.3 * swing,
        z: -10.0 + 9.0 * angle.sin(),
        scale: 0.8 * s.size as f32 / 100.0,
        spin: if s.spin { 900.0 * angle.sin().abs() } else { 0.0 },
        rock_x: rock,
        rock_z: rock,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_defaults_are_vanillas_curve() {
        let p = pose(40, 0.0, 0.5, -0.5, 16.0 / 9.0, &Settings::default());
        assert_eq!((p.x, p.y, p.z), (0.0, 0.0, -10.0), "first tick: straight ahead, 10 out");
        assert!((p.scale - 0.8).abs() < 1e-6);
        let mid = pose(20, 0.0, 0.5, -0.5, 16.0 / 9.0, &Settings::default());
        assert!((mid.z + 1.0).abs() < 1e-2, "halfway it is about 1 out: {}", mid.z);
        let a = 0.498_437_5 * std::f32::consts::PI;
        assert!((mid.x - 0.5 * 0.3 * (16.0 / 9.0) * (a * 2.0).sin().abs()).abs() < 1e-5);
    }

    #[test]
    fn off_hides_it_and_speed_ends_it_sooner() {
        let off = Settings { pop: false, ..Settings::default() };
        assert_eq!(pose(30, 0.0, 0.0, 0.0, 1.0, &off).scale, 0.0);
        let fast = Settings { speed: 2.0, ..Settings::default() };
        assert!(pose(30, 0.0, 0.0, 0.0, 1.0, &fast).scale > 0.0, "a quarter of the way in, still showing");
        assert_eq!(pose(20, 0.0, 0.0, 0.0, 1.0, &fast).scale, 0.0, "at double speed it is over at tick 20");
        let slow = Settings { speed: 0.5, ..Settings::default() };
        assert!(pose(2, 0.0, 0.0, 0.0, 1.0, &slow).scale > 0.0, "slow motion lasts to the end");
    }

    #[test]
    fn size_spin_rocking_and_swing_follow_the_settings() {
        let big = Settings { size: 200, ..Settings::default() };
        assert!((pose(25, 0.0, 0.3, 0.3, 1.0, &big).scale - 1.6).abs() < 1e-6);
        let still = Settings { spin: false, rock: false, swing: false, ..Settings::default() };
        let p = pose(25, 0.0, 0.7, 0.7, 1.7, &still);
        assert_eq!((p.spin, p.rock_x, p.rock_z, p.x, p.y), (0.0, 0.0, 0.0, 0.0, 0.0));
        let full = pose(25, 0.0, 0.7, 0.7, 1.7, &Settings::default());
        assert!(full.spin > 0.0 && full.x != 0.0);
        assert_eq!(full.z, p.z, "the flight in and out is not touched");
    }

    #[test]
    fn the_settings_page_cycles_and_reads_back() {
        assert_eq!(read(none), Settings::default());
        assert_eq!(click(none, 0), Some(("pop", "false".to_owned())));
        assert_eq!(click(none, 1), Some(("speed", "1.5".to_owned())));
        assert_eq!(click(none, 2), Some(("size", "125".to_owned())));
        assert_eq!(click(none, 6), None);
        let last = |k: &str| (k == "speed").then(|| "3".to_owned());
        assert_eq!(click(last, 1), Some(("speed", "0.5".to_owned())), "wraps");
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(read(junk), Settings { pop: true, ..Settings::default() });
        assert_eq!(labels(none)[1], "Speed: 1x");
        configure(true, Settings { size: 50, ..Settings::default() });
        assert!(active() && current().size == 50);
        configure(false, Settings::default());
    }
}
