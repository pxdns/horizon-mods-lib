// Chat Animation, a native Horizon mod: Makes chat messages appear with a smooth animation.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_chat_animation`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "chat_animation";
pub const NAME: &str = "Chat Animation";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Makes chat messages appear with a smooth animation.";

/// The durations the two Duration buttons step through, in milliseconds (the original takes any number; the `message_ms` and
/// `input_ms` files do too).
pub const STEPS: [i32; 8] = [0, 50, 100, 150, 170, 300, 500, 1000];

/// The settings of `ModConfig`, with its defaults, in the original's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `Animate Messages`.
    pub animate_messages: bool,
    /// `Duration` of the messages' animation.
    pub message_ms: i32,
    /// `Hide Indicator Line`: "Hide the indicator line on the left of messages."
    pub hide_indicator: bool,
    /// `Animate Opacity`.
    pub animate_opacity: bool,
    /// `Animate Input Field`.
    pub animate_input: bool,
    /// `Duration` of the input field's animation.
    pub input_ms: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { animate_messages: true, message_ms: 150, hide_indicator: true, animate_opacity: true, animate_input: true, input_ms: 170 }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        let ms = |key: &str, d: i32| get(key).and_then(|v| v.trim().parse::<i32>().ok()).map_or(d, |v| v.max(0));
        Self {
            animate_messages: flag("animate_messages", d.animate_messages),
            message_ms: ms("message_ms", d.message_ms),
            hide_indicator: flag("hide_indicator", d.hide_indicator),
            animate_opacity: flag("animate_opacity", d.animate_opacity),
            animate_input: flag("animate_input", d.animate_input),
            input_ms: ms("input_ms", d.input_ms),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Animate Messages: {}", on(self.animate_messages)),
            format!("Messages Duration: {} ms", self.message_ms),
            format!("Hide Indicator Line: {}", on(self.hide_indicator)),
            format!("Animate Opacity: {}", on(self.animate_opacity)),
            format!("Animate Input Field: {}", on(self.animate_input)),
            format!("Input Field Duration: {} ms", self.input_ms),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let step = |v: i32| *STEPS.iter().find(|&&s| s > v).unwrap_or(&STEPS[0]);
        match i {
            0 => {
                self.animate_messages = !self.animate_messages;
                Some(("animate_messages", self.animate_messages.to_string()))
            }
            1 => {
                self.message_ms = step(self.message_ms);
                Some(("message_ms", self.message_ms.to_string()))
            }
            2 => {
                self.hide_indicator = !self.hide_indicator;
                Some(("hide_indicator", self.hide_indicator.to_string()))
            }
            3 => {
                self.animate_opacity = !self.animate_opacity;
                Some(("animate_opacity", self.animate_opacity.to_string()))
            }
            4 => {
                self.animate_input = !self.animate_input;
                Some(("animate_input", self.animate_input.to_string()))
            }
            5 => {
                self.input_ms = step(self.input_ms);
                Some(("input_ms", self.input_ms.to_string()))
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
        assert_eq!((s.animate_messages, s.message_ms, s.hide_indicator), (true, 150, true));
        assert_eq!((s.animate_opacity, s.animate_input, s.input_ms), (true, true, 170));
        assert_eq!(s.labels().len(), 6);
    }

    #[test]
    fn stored_values_are_read_and_junk_falls_back() {
        let stored = |k: &str| match k {
            "message_ms" => Some("300".to_owned()),
            "animate_input" => Some("false".to_owned()),
            _ => None,
        };
        let s = Settings::read(stored);
        assert_eq!((s.message_ms, s.animate_input), (300, false));
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let neg = |k: &str| (k == "input_ms").then(|| "-5".to_owned());
        assert_eq!(Settings::read(neg).input_ms, 0);
    }

    #[test]
    fn the_labels_follow_the_originals_order() {
        let l = labels(none);
        assert!(l[0].starts_with("Animate Messages") && l[1].starts_with("Messages Duration") && l[2].starts_with("Hide Indicator Line"));
        assert!(l[3].starts_with("Animate Opacity") && l[4].starts_with("Animate Input Field") && l[5].starts_with("Input Field Duration"));
    }

    #[test]
    fn the_buttons_flip_and_the_durations_step_and_wrap() {
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("animate_messages", "false"));
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("message_ms", "170"), "150 steps to 170");
        let top = |k: &str| (k == "input_ms").then(|| "1000".to_owned());
        assert_eq!(click(top, 5).unwrap().1, "0", "1000 wraps to 0");
        let (key, value) = click(none, 2).unwrap();
        assert_eq!((key, value.as_str()), ("hide_indicator", "false"));
        assert!(click(none, 9).is_none());
    }
}
