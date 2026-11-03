// Simple Zoom, a native Horizon mod: Press a key to zoom in and press it again to zoom out; the wheel changes the zoom meanwhile.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_simple_zoom`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "simple_zoom";
pub const NAME: &str = "Simple Zoom";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Press a key to zoom in and press it again to zoom out; the wheel changes the zoom meanwhile.";

use std::cell::RefCell;

/// The key's name in Controls, and its default.
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "key.keyboard.v";
/// Whether the key is held to zoom (otherwise one press turns it on and the next turns it off).
pub const HOLD: bool = false;
/// The zooms the starting zoom cycles through, and the limits of the wheel.
pub const LEVELS: [u8; 7] = [2, 3, 4, 6, 8, 12, 16];
const MIN: f32 = 1.25;
const MAX: f32 = 16.0;
/// Each wheel step multiplies the zoom by this.
const STEP: f32 = 1.25;

/// The settings, one stored file per key.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The zoom the key starts at, one of `LEVELS`.
    pub level: u8,
    /// Ease in and out like the spyglass.
    pub smooth: bool,
    /// Turn slower the more you zoom in.
    pub sensitivity: bool,
    /// The wheel changes the zoom while it is on.
    pub scroll: bool,
}

impl Settings {
    /// From the stored values (`get` reads one key's file).
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let level = get("level")
            .and_then(|v| v.trim().parse::<u8>().ok())
            .filter(|v| LEVELS.contains(v))
            .unwrap_or(3);
        let flag = |key: &str, default: bool| {
            get(key).map_or(default, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => default,
            })
        };
        Self {
            level,
            smooth: flag("smooth", true),
            sensitivity: flag("sensitivity", true),
            scroll: flag("scroll", true),
        }
    }

    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Starting zoom: {}x", self.level),
            format!("Smooth zoom: {}", on(self.smooth)),
            format!("Slower turning: {}", on(self.sensitivity)),
            format!("Wheel zoom: {}", on(self.scroll)),
        ]
    }

    /// A click on button `i` of `labels`; the key and value that changed.
    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                let at = LEVELS.iter().position(|&l| l == self.level).unwrap_or(0);
                self.level = LEVELS[(at + 1) % LEVELS.len()];
                Some(("level", self.level.to_string()))
            }
            1 => {
                self.smooth = !self.smooth;
                Some(("smooth", self.smooth.to_string()))
            }
            2 => {
                self.sensitivity = !self.sensitivity;
                Some(("sensitivity", self.sensitivity.to_string()))
            }
            3 => {
                self.scroll = !self.scroll;
                Some(("scroll", self.scroll.to_string()))
            }
            _ => None,
        }
    }
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    /// The zoom while it is on; None when it is off.
    active: Option<f32>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    settings(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = settings(get);
    let stored = now.click(i);
    STATE.with_borrow_mut(|s| s.settings = Some(now));
    stored
}

/// The zoom key went down or up (`playing`: nothing is over the world).
pub fn key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    let level = f32::from(settings(get).level).clamp(MIN, MAX);
    STATE.with_borrow_mut(|s| {
        if HOLD {
            if pressed && !playing {
                return;
            }
            s.active = pressed.then_some(level);
        } else if pressed {
            s.active = match s.active {
                Some(_) => None,
                None if playing => Some(level),
                None => None,
            };
        }
    });
}

/// Wheel steps (up is positive): true when the zoom is on and they change it.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|s| {
        let scroll = s.settings.is_none_or(|v| v.scroll);
        match &mut s.active {
            Some(z) if scroll => {
                *z = (*z * STEP.powf(steps)).clamp(MIN, MAX);
                true
            }
            _ => false,
        }
    })
}

/// The zoom to ask the game for: factor, smooth, and how much to scale mouse turning.
pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        let z = s.active?;
        let v = s.settings.unwrap_or(Settings::read(|_| None));
        Some((z, v.smooth, if v.sensitivity { 1.0 / z } else { 1.0 }))
    })
}

/// Turns the zoom off and forgets the settings (tests only).
#[cfg(test)]
pub fn forget() {
    STATE.with_borrow_mut(|s| *s = State::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_key_zooms_by_its_mode() {
        forget();
        key(true, true, none);
        assert_eq!(view().map(|v| v.0), Some(f32::from(3u8)));
        key(true, false, none);
        if HOLD {
            assert_eq!(view(), None, "released");
        } else {
            assert!(view().is_some(), "a toggle stays on");
            key(true, true, none);
            assert_eq!(view(), None, "the second press turns it off");
        }
        forget();
    }

    #[test]
    fn a_screen_over_the_world_does_not_start_the_zoom() {
        forget();
        key(false, true, none);
        assert_eq!(view(), None);
        forget();
    }

    #[test]
    fn the_wheel_zooms_only_while_on_and_within_limits() {
        forget();
        assert!(!wheel(1.0), "off: the hotbar keeps it");
        key(true, true, none);
        let start = view().unwrap().0;
        assert!(wheel(2.0));
        assert!(view().unwrap().0 > start);
        assert!(wheel(-100.0));
        assert_eq!(view().unwrap().0, MIN);
        assert!(wheel(100.0));
        assert_eq!(view().unwrap().0, MAX);
        forget();
    }

    #[test]
    fn the_wheel_can_be_turned_off() {
        forget();
        let stored = |k: &str| (k == "scroll").then(|| "false".to_owned());
        key(true, true, stored);
        assert!(!wheel(1.0));
        forget();
    }

    #[test]
    fn settings_read_default_and_click_reports_the_change() {
        forget();
        let s = settings(none);
        assert_eq!(s.level, 3);
        assert!(s.smooth && s.sensitivity && s.scroll);
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("smooth", "false"));
        assert_eq!(labels(none)[1], "Smooth zoom: OFF");
        assert!(click(none, 9).is_none());
        let (_, level) = click(none, 0).unwrap();
        assert!(LEVELS.iter().any(|l| l.to_string() == level));
        forget();
    }

    #[test]
    fn turning_slows_with_the_zoom() {
        forget();
        key(true, true, none);
        let (factor, _, turn) = view().unwrap();
        assert!((turn - 1.0 / factor).abs() < 1e-6);
        forget();
    }

    #[test]
    fn stored_junk_falls_back_to_the_defaults() {
        let junk = |_: &str| Some("banana".to_owned());
        let s = Settings::read(junk);
        assert_eq!(s.level, 3);
        assert!(s.smooth);
    }
}
