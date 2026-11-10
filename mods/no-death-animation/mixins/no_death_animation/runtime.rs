// No Death Animation, a native Horizon mod: Removes the death animation tilt on players, and sets how long the red death overlay stays.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_no_death_animation`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "no_death_animation";
pub const NAME: &str = "No Death Animation";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Removes the death animation tilt on players, and sets how long the red death overlay stays.";

/// The values the two time buttons cycle through, in ticks. The original takes any 0 to 100; a button can only step, so
/// it steps through these (22, the default, is one of them). The `death_time` and `overlay_time` files take any 0 to 100.
pub const STEPS: [i32; 9] = [0, 5, 10, 15, 20, 22, 40, 60, 100];
/// The `Affected Entities` choices, as stored in the `affected` file (a comma separated list the player can also edit by hand).
pub const AFFECTED: [&str; 3] = ["minecraft:player", "minecraft:player,minecraft:villager", "minecraft:player,minecraft:zombie,minecraft:skeleton,minecraft:creeper"];

/// The settings of the original's General category, with its defaults.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `Mod Enabled`.
    pub enabled: bool,
    /// `Death Time`: ticks before a dead entity disappears (0 to 100).
    pub death_time: i32,
    /// `Death Overlay Time`: ticks the red tint stays after death (0 to 100).
    pub overlay_time: i32,
    /// `Affected Entities`, normalised the way the original does.
    pub affected: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { enabled: true, death_time: 22, overlay_time: 22, affected: vec!["minecraft:player".to_owned()] }
    }
}

/// `getEntityTypeFromString`'s first steps: trim, lower case, spaces to underscores, `minecraft:` when no namespace.
/// None for a blank entry.
pub fn normalise(entry: &str) -> Option<String> {
    let e = entry.trim();
    if e.is_empty() {
        return None;
    }
    let e = e.to_lowercase().replace(' ', "_");
    Some(if e.contains(':') { e } else { format!("minecraft:{e}") })
}

fn clamp(v: i32) -> i32 {
    v.clamp(0, 100)
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let num = |key: &str, d: i32| get(key).and_then(|v| v.trim().parse::<i32>().ok()).map_or(d, clamp);
        let affected: Vec<String> = get("affected").map_or_else(|| d.affected.clone(), |v| v.split(',').filter_map(normalise).collect());
        Self {
            enabled: get("enabled").map_or(d.enabled, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => d.enabled,
            }),
            death_time: num("death_time", d.death_time),
            overlay_time: num("overlay_time", d.overlay_time),
            affected,
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Mod Enabled: {}", if self.enabled { "ON" } else { "OFF" }),
            format!("Death Time: {} ticks", self.death_time),
            format!("Death Overlay Time: {} ticks", self.overlay_time),
            format!("Affected Entities: {}", self.affected.join(", ")),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let step = |v: i32| *STEPS.iter().find(|&&s| s > v).unwrap_or(&STEPS[0]);
        match i {
            0 => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string()))
            }
            1 => {
                self.death_time = step(self.death_time);
                Some(("death_time", self.death_time.to_string()))
            }
            2 => {
                self.overlay_time = step(self.overlay_time);
                Some(("overlay_time", self.overlay_time.to_string()))
            }
            3 => {
                let now = self.affected.join(",");
                let at = AFFECTED.iter().position(|&a| a == now).map_or(0, |p| p + 1);
                let next = AFFECTED[at % AFFECTED.len()];
                self.affected = next.split(',').map(str::to_owned).collect();
                Some(("affected", next.to_owned()))
            }
            _ => None,
        }
    }

    /// `NoDeathAnimation.shouldOverride`: whether entities of `kind` are affected.
    pub fn should_override(&self, kind: &str) -> bool {
        self.enabled && self.affected.iter().any(|a| a == kind)
    }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = Settings::read(get);
    now.click(i)
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
        assert!(s.enabled);
        assert_eq!((s.death_time, s.overlay_time), (22, 22));
        assert_eq!(s.affected, ["minecraft:player"]);
        assert!(s.should_override("minecraft:player"));
        assert!(!s.should_override("minecraft:zombie"));
    }

    #[test]
    fn disabled_overrides_nothing() {
        let off = |k: &str| (k == "enabled").then(|| "false".to_owned());
        assert!(!Settings::read(off).should_override("minecraft:player"));
    }

    #[test]
    fn entries_are_normalised_like_the_original() {
        assert_eq!(normalise("  Player "), Some("minecraft:player".to_owned()));
        assert_eq!(normalise("Iron Golem"), Some("minecraft:iron_golem".to_owned()));
        assert_eq!(normalise("Mod:Thing"), Some("mod:thing".to_owned()));
        assert_eq!(normalise("   "), None);
        let list = |k: &str| (k == "affected").then(|| "Zombie, ,minecraft:player".to_owned());
        assert_eq!(Settings::read(list).affected, ["minecraft:zombie", "minecraft:player"]);
    }

    #[test]
    fn times_are_clamped_to_zero_to_a_hundred() {
        let big = |k: &str| (k == "death_time" || k == "overlay_time").then(|| "500".to_owned());
        let s = Settings::read(big);
        assert_eq!((s.death_time, s.overlay_time), (100, 100));
        let neg = |k: &str| (k == "death_time").then(|| "-4".to_owned());
        assert_eq!(Settings::read(neg).death_time, 0);
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk).death_time, 22);
    }

    #[test]
    fn the_buttons_step_and_report_what_to_store() {
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("death_time", "40"), "22 steps to 40");
        let at100 = |k: &str| (k == "overlay_time").then(|| "100".to_owned());
        let (_, wrapped) = click(at100, 2).unwrap();
        assert_eq!(wrapped, "0", "100 wraps to 0");
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("enabled", "false"));
        let (key, value) = click(none, 3).unwrap();
        assert_eq!((key, value.as_str()), ("affected", "minecraft:player,minecraft:villager"));
        assert!(click(none, 9).is_none());
        assert_eq!(labels(none)[0], "Mod Enabled: ON");
    }
}
