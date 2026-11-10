// PotionCounter, a native Horizon mod: Shows how many potions you have left in your inventory.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_potion_counter`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "potion_counter";
pub const NAME: &str = "PotionCounter";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows how many potions you have left in your inventory.";

/// The potions with an effect, by the name each button switches off.
pub const POTIONS: [&str; 18] = [
    "night_vision", "invisibility", "leaping", "fire_resistance", "swiftness", "slowness", "turtle_master", "water_breathing",
    "healing", "harming", "poison", "regeneration", "strength", "weakness", "slow_falling", "wind_charged", "weaving", "oozing",
];

/// `PotionCounterConfig` of the original, with its defaults. `disabled` holds potion names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub enabled: bool,
    pub show_upgrades: bool,
    pub disabled: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { enabled: true, show_upgrades: true, disabled: Vec::new() }
    }
}

pub fn names() -> Vec<&'static str> {
    POTIONS.to_vec()
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
            show_upgrades: flag("upgrades", d.show_upgrades),
            disabled: get("disabled").map(|v| v.split(',').map(str::trim).filter(|p| !p.is_empty()).map(str::to_owned).collect()).unwrap_or_default(),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        let mut v = vec![
            format!("Enabled: {}", on(self.enabled)),
            format!("Show potion upgrades: {}", on(self.show_upgrades)),
        ];
        v.extend(names().into_iter().map(|n| format!("{n}: {}", on(!self.disabled.iter().any(|d| d == n)))));
        v
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string()))
            }
            1 => {
                self.show_upgrades = !self.show_upgrades;
                Some(("upgrades", self.show_upgrades.to_string()))
            }
            n => {
                let name = *names().get(n.checked_sub(2)?)?;
                match self.disabled.iter().position(|d| d == name) {
                    Some(at) => {
                        self.disabled.remove(at);
                    }
                    None => self.disabled.push(name.to_owned()),
                }
                Some(("disabled", self.disabled.join(",")))
            }
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
        assert!(s.enabled && s.show_upgrades && s.disabled.is_empty());
    }

    #[test]
    fn a_button_per_potion_follows_the_two_settings() {
        assert_eq!(labels(none).len(), 2 + 18);
        assert_eq!(click(none, 0).unwrap(), ("enabled", "false".to_owned()));
        assert_eq!(click(none, 1).unwrap(), ("upgrades", "false".to_owned()));
        assert_eq!(click(none, 2).unwrap(), ("disabled", "night_vision".to_owned()));
        assert!(click(none, 20).is_none());
    }

    #[test]
    fn a_disabled_potion_switches_back_on() {
        let off = |k: &str| (k == "disabled").then(|| "night_vision,healing".to_owned());
        assert_eq!(click(off, 2).unwrap().1, "healing");
        assert!(labels(off)[2].ends_with("OFF") && labels(off)[3].ends_with("ON"));
    }

    #[test]
    fn junk_falls_back_to_the_defaults() {
        let junk = |_: &str| Some("banana".to_owned());
        let s = Settings::read(junk);
        assert!(s.enabled && s.show_upgrades);
    }
}
