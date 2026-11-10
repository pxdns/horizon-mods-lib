// BetterHurtCam, a native Horizon mod: nohurtcam but better: turn the hurt camera off or change its intensity, with a key or the settings page.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_better_hurt_cam`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "better_hurt_cam";
pub const NAME: &str = "BetterHurtCam";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "nohurtcam but better: turn the hurt camera off or change its intensity, with a key or the settings page.";

/// The three keys (`key.betterhurtcam.toggle`, `.plus`, `.minus`) and their default: unbound. The original's bytecode gives
/// key codes 65, 64 and 63 (an `A`, and two codes GLFW has no key for), which cannot be meant as defaults, so none is bound.
pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_PLUS: &str = "plus";
pub const KEY_MINUS: &str = "minus";
pub const DEFAULT_KEY: &str = "key.keyboard.unknown";

/// The settings of `BHCConfig`, with its defaults.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// `Hurtcam Enabled`.
    pub enabled: bool,
    /// `Hurtcam Multiplier`: any number, shown with two decimals.
    pub multiplier: f64,
    /// `Health Bar Blinking`.
    pub heart_blink: bool,
    /// `Hurtcam Type`: true is Old (no direction change), false is Yaw-based.
    pub old_type: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { enabled: true, multiplier: 0.3, heart_blink: true, old_type: false }
    }
}

fn round(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        Self {
            enabled: flag("enabled", d.enabled),
            // `convert` takes a comma as a point.
            multiplier: get("multiplier").and_then(|v| v.trim().replace(',', ".").parse::<f64>().ok()).filter(|v| v.is_finite()).unwrap_or(d.multiplier),
            heart_blink: flag("heart_blink", d.heart_blink),
            old_type: get("type").map_or(d.old_type, |v| match v.trim() {
                "old" => true,
                "yaw_based" => false,
                _ => d.old_type,
            }),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Hurtcam Enabled: {}", if self.enabled { "ON" } else { "OFF" }),
            format!("Hurtcam Multiplier: {:.2}", self.multiplier),
            format!("Health Bar Blinking: {}", if self.heart_blink { "ON" } else { "OFF" }),
            format!("Hurtcam Type: {}", if self.old_type { "Old" } else { "Yaw-based" }),
        ]
    }

    /// `modifyMultiplier`: no limits.
    pub fn modify(&mut self, by: f64) {
        self.multiplier = round(self.multiplier + by);
    }

    /// The multiplier as the key's overlay message prints it (`%.2f`).
    pub fn multiplier_text(&self) -> String {
        format!("{:.2}", self.multiplier)
    }

    /// A click on the settings page: what to store. The multiplier button adds 0.1 and goes back to 0 after 1.0
    /// (the original types any number; the `multiplier` file takes any too).
    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string()))
            }
            1 => {
                self.multiplier = if self.multiplier >= 0.95 || self.multiplier < -0.05 { 0.0 } else { round(self.multiplier + 0.1) };
                Some(("multiplier", self.multiplier.to_string()))
            }
            2 => {
                self.heart_blink = !self.heart_blink;
                Some(("heart_blink", self.heart_blink.to_string()))
            }
            3 => {
                self.old_type = !self.old_type;
                Some(("type", (if self.old_type { "old" } else { "yaw_based" }).to_owned()))
            }
            _ => None,
        }
    }

    /// What a key does to the settings: the key's name, the file to store and its value, and whether to show the multiplier.
    pub fn key(&mut self, name: &str) -> Option<(&'static str, String, bool)> {
        match name {
            KEY_TOGGLE => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string(), false))
            }
            KEY_PLUS => {
                self.modify(0.1);
                Some(("multiplier", self.multiplier.to_string(), true))
            }
            KEY_MINUS => {
                self.modify(-0.1);
                Some(("multiplier", self.multiplier.to_string(), true))
            }
            _ => None,
        }
    }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    Settings::read(get).click(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let s = Settings::read(none);
        assert_eq!((s.enabled, s.multiplier, s.heart_blink, s.old_type), (true, 0.3, true, false));
        assert_eq!(s.multiplier_text(), "0.30");
        assert_eq!(s.labels(), ["Hurtcam Enabled: ON", "Hurtcam Multiplier: 0.30", "Health Bar Blinking: ON", "Hurtcam Type: Yaw-based"]);
    }

    #[test]
    fn the_keys_step_the_multiplier_by_a_tenth_without_limits() {
        let mut s = Settings::default();
        let (file, value, show) = s.key(KEY_PLUS).unwrap();
        assert_eq!((file, value.as_str(), show), ("multiplier", "0.4", true));
        for _ in 0..10 {
            s.key(KEY_PLUS);
        }
        assert_eq!(s.multiplier_text(), "1.40", "past 1");
        for _ in 0..20 {
            s.key(KEY_MINUS);
        }
        assert_eq!(s.multiplier_text(), "-0.60", "below 0");
        let (file, value, show) = s.key(KEY_TOGGLE).unwrap();
        assert_eq!((file, value.as_str(), show), ("enabled", "false", false));
        assert!(s.key("other").is_none());
    }

    #[test]
    fn a_comma_is_a_point_and_junk_falls_back() {
        let comma = |k: &str| (k == "multiplier").then(|| "1,5".to_owned());
        assert_eq!(Settings::read(comma).multiplier, 1.5);
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let nan = |k: &str| (k == "multiplier").then(|| "NaN".to_owned());
        assert_eq!(Settings::read(nan).multiplier, 0.3);
    }

    #[test]
    fn the_buttons_flip_and_the_multiplier_button_cycles_to_zero() {
        let (key, value) = click(none, 3).unwrap();
        assert_eq!((key, value.as_str()), ("type", "old"));
        let (key, value) = click(none, 2).unwrap();
        assert_eq!((key, value.as_str()), ("heart_blink", "false"));
        let (_, value) = click(none, 1).unwrap();
        assert_eq!(value, "0.4");
        let top = |k: &str| (k == "multiplier").then(|| "1".to_owned());
        assert_eq!(click(top, 1).unwrap().1, "0", "after 1.0 it goes back to 0");
        assert!(click(none, 8).is_none());
    }
}
