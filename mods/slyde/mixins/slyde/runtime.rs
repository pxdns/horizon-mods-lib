// Slyde, a native Horizon mod: Allows dragging sliders further than intended.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_slyde`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "slyde";
pub const NAME: &str = "Slyde";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Allows dragging sliders further than intended.";

/// The one setting of the original this port applies (`Cfg.holdKeyToActivate`, off by default).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Settings {
    pub hold_key_to_activate: bool,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        Self { hold_key_to_activate: get("hold_key").is_some_and(|v| v.trim() == "true") }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![format!("Hold Shift/Ctrl key to activate: {}", if self.hold_key_to_activate { "ON" } else { "OFF" })]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        (i == 0).then(|| {
            self.hold_key_to_activate = !self.hold_key_to_activate;
            ("hold_key", self.hold_key_to_activate.to_string())
        })
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
    fn the_extended_drag_is_on_by_default() {
        assert!(!Settings::read(none).hold_key_to_activate);
    }

    #[test]
    fn the_button_turns_the_setting_round() {
        assert_eq!(labels(none), vec!["Hold Shift/Ctrl key to activate: OFF".to_owned()]);
        assert_eq!(click(none, 0).unwrap(), ("hold_key", "true".to_owned()));
        let on = |k: &str| (k == "hold_key").then(|| "true".to_owned());
        assert_eq!(click(on, 0).unwrap().1, "false");
        assert_eq!(labels(on)[0], "Hold Shift/Ctrl key to activate: ON");
    }

    #[test]
    fn two_presses_return_to_the_start() {
        let mut s = Settings::default();
        s.click(0);
        s.click(0);
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn there_is_only_the_one_button_and_junk_means_the_default() {
        assert!(click(none, 1).is_none());
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
    }
}
