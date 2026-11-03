// Better Hurt Cam, a native Horizon mod: Tone the camera shake down when you are hit, and switch the death tilt off.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_better_hurt_cam`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "better_hurt_cam";
pub const NAME: &str = "Better Hurt Cam";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Tone the camera shake down when you are hit, and switch the death tilt off.";

use std::cell::Cell;

/// The tilt strengths the first button cycles through, as a percentage of Horizon's own.
pub const LEVELS: [u32; 5] = [0, 25, 50, 75, 100];

/// The settings, one stored file per key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// The hurt tilt, a percentage of the game's setting; one of `LEVELS`.
    pub tilt: u32,
    /// Whether the camera still rolls over when you die.
    pub death: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tilt: 50,
            death: false,
        }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        Self {
            tilt: get("tilt")
                .and_then(|v| v.trim().parse().ok())
                .filter(|v| LEVELS.contains(v))
                .unwrap_or(d.tilt),
            death: get("death").map_or(d.death, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => d.death,
            }),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Hit shake: {}%", self.tilt),
            format!("Death tilt: {}", if self.death { "ON" } else { "OFF" }),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                let at = LEVELS.iter().position(|&l| l == self.tilt).unwrap_or(0);
                self.tilt = LEVELS[(at + 1) % LEVELS.len()];
                Some(("tilt", self.tilt.to_string()))
            }
            1 => {
                self.death = !self.death;
                Some(("death", self.death.to_string()))
            }
            _ => None,
        }
    }

    /// The factor to multiply the game's own damage tilt by.
    pub fn factor(&self) -> f64 {
        f64::from(self.tilt) / 100.0
    }

    /// Whether the game's own matrix is exactly what these settings give, so the original can run.
    pub fn is_vanilla(&self) -> bool {
        self.tilt == 100 && self.death
    }
}

thread_local! {
    static SETTINGS: Cell<Option<Settings>> = const { Cell::new(None) };
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|s| {
        let v = s.get().unwrap_or_else(|| Settings::read(get));
        s.set(Some(v));
        v
    })
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    settings(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = settings(get);
    let stored = now.click(i);
    SETTINGS.with(|s| s.set(Some(now)));
    stored
}

/// Forgets the settings (tests only).
#[cfg(test)]
pub fn forget() {
    SETTINGS.with(|s| s.set(None));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_halve_the_shake_and_drop_the_death_tilt() {
        forget();
        let s = settings(none);
        assert_eq!((s.tilt, s.death), (50, false));
        assert_eq!(s.factor(), 0.5);
        assert!(!s.is_vanilla());
        forget();
    }

    #[test]
    fn the_shake_button_cycles_and_reports_what_to_store() {
        forget();
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("tilt", "75"));
        click(none, 0);
        let (_, wrapped) = click(none, 0).unwrap();
        assert_eq!(wrapped, "0", "100 wraps to 0");
        assert_eq!(settings(none).factor(), 0.0);
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("death", "true"));
        assert_eq!(labels(none), ["Hit shake: 0%", "Death tilt: ON"]);
        assert!(click(none, 7).is_none());
        forget();
    }

    #[test]
    fn vanilla_settings_let_the_original_run() {
        assert!(
            Settings {
                tilt: 100,
                death: true
            }
            .is_vanilla()
        );
        assert!(
            !Settings {
                tilt: 100,
                death: false
            }
            .is_vanilla()
        );
    }

    #[test]
    fn stored_junk_falls_back() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let stored = |k: &str| match k {
            "tilt" => Some("25".to_owned()),
            "death" => Some("true".to_owned()),
            _ => None,
        };
        assert_eq!(
            Settings::read(stored),
            Settings {
                tilt: 25,
                death: true
            }
        );
        let off_list = |_: &str| Some("30".to_owned());
        assert_eq!(
            Settings::read(off_list).tilt,
            50,
            "30 is not one of the levels"
        );
    }
}
