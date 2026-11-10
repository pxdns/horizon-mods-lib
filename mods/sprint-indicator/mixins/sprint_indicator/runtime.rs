// Sprint Indicator, a native Horizon mod: Simple UI element showing whether you're currently sprinting, and whether you're crouching too.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_sprint_indicator`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "sprint_indicator";
pub const NAME: &str = "Sprint Indicator";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Simple UI element showing whether you're currently sprinting, and whether you're crouching too.";

/// The offsets the two Icon offset buttons step through, in GUI pixels (the original takes any whole numbers as `x,y`; the `offset_x`
/// and `offset_y` files do too).
pub const STEPS: [i32; 9] = [-20, -10, -5, -2, 0, 2, 5, 10, 20];
/// `adjustAttackIndicatorX`: how far the hotbar's attack indicator moves out for the icon.
pub const ATTACK_SHIFT: i32 = 24;
/// The icon's size and the alpha of the "armed" sprint icon (`ARMED_ALPHA`).
pub const SIZE: i32 = 18;
pub const ARMED_ALPHA: f32 = 0.7;

/// The settings of `SprintIndicatorConfig`, with its defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `sprintEnabled`: show an icon when sprinting.
    pub sprint: bool,
    /// `sprintShowWhenActive`: also show the sprint icon, dimmed, while the sprint key is active but you are not sprinting.
    pub show_when_active: bool,
    /// `crouchEnabled`: show an icon when crouching.
    pub crouch: bool,
    /// `offset`: x and y.
    pub offset: (i32, i32),
    /// `adjustAttackIndicator`: shift the hotbar's attack indicator out of the icon's way.
    pub adjust_attack_indicator: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { sprint: true, show_when_active: false, crouch: true, offset: (0, 0), adjust_attack_indicator: true }
    }
}

/// What to draw, from `HudMixin.afterRenderHotbar`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Icon {
    /// True for the slowness icon (crouching), false for the speed icon.
    pub crouch: bool,
    pub alpha: f32,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        let num = |key: &str, d: i32| get(key).and_then(|v| v.trim().parse::<i32>().ok()).unwrap_or(d);
        Self {
            sprint: flag("sprint", d.sprint),
            show_when_active: flag("show_when_active", d.show_when_active),
            crouch: flag("crouch", d.crouch),
            offset: (num("offset_x", d.offset.0), num("offset_y", d.offset.1)),
            adjust_attack_indicator: flag("adjust_attack_indicator", d.adjust_attack_indicator),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Show sprint indicator: {}", on(self.sprint)),
            format!("Show when sprint key is active: {}", on(self.show_when_active)),
            format!("Show crouch indicator: {}", on(self.crouch)),
            format!("Icon offset X: {}", self.offset.0),
            format!("Icon offset Y: {}", self.offset.1),
            format!("Shift attack indicator: {}", on(self.adjust_attack_indicator)),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let step = |v: i32| *STEPS.iter().find(|&&s| s > v).unwrap_or(&STEPS[0]);
        match i {
            0 => {
                self.sprint = !self.sprint;
                Some(("sprint", self.sprint.to_string()))
            }
            1 => {
                self.show_when_active = !self.show_when_active;
                Some(("show_when_active", self.show_when_active.to_string()))
            }
            2 => {
                self.crouch = !self.crouch;
                Some(("crouch", self.crouch.to_string()))
            }
            3 => {
                self.offset.0 = step(self.offset.0);
                Some(("offset_x", self.offset.0.to_string()))
            }
            4 => {
                self.offset.1 = step(self.offset.1);
                Some(("offset_y", self.offset.1.to_string()))
            }
            5 => {
                self.adjust_attack_indicator = !self.adjust_attack_indicator;
                Some(("adjust_attack_indicator", self.adjust_attack_indicator.to_string()))
            }
            _ => None,
        }
    }

    /// The icon to draw, if any: `sprinting` is `isSprinting`, `crouching` is `isCrouching`, `sprint_key` is the sprint key being active.
    /// Crouching wins and is drawn in full; sprinting is full; an armed sprint key is dimmed.
    pub fn icon(&self, sprinting: bool, crouching: bool, sprint_key: bool) -> Option<Icon> {
        let sprint = self.sprint && sprinting;
        let armed = !sprint && self.sprint && self.show_when_active && sprint_key;
        let crouch = self.crouch && crouching;
        if !sprint && !armed && !crouch {
            return None;
        }
        Some(Icon { crouch, alpha: if crouch || sprint { 1.0 } else { ARMED_ALPHA } })
    }

    /// Where the icon goes, from the screen's size in GUI pixels: beside the hotbar on the off hand's side, 6 pixels out.
    pub fn position(&self, width: i32, height: i32, main_hand_left: bool) -> (i32, i32) {
        let x = if main_hand_left { (width - 182) / 2 - 18 - 6 } else { (width + 182) / 2 + 6 };
        (x + self.offset.0, height - 20 + self.offset.1)
    }

    /// How far to push the hotbar's attack indicator out (0 when not adjusting).
    pub fn attack_shift(&self) -> i32 {
        if self.adjust_attack_indicator && (self.sprint || self.crouch) { ATTACK_SHIFT } else { 0 }
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
        assert_eq!((s.sprint, s.show_when_active, s.crouch), (true, false, true));
        assert_eq!((s.offset, s.adjust_attack_indicator), ((0, 0), true));
    }

    #[test]
    fn sprinting_and_crouching_show_in_full_and_crouching_wins() {
        let s = Settings::default();
        assert_eq!(s.icon(true, false, false), Some(Icon { crouch: false, alpha: 1.0 }));
        assert_eq!(s.icon(false, true, false), Some(Icon { crouch: true, alpha: 1.0 }));
        assert_eq!(s.icon(true, true, false), Some(Icon { crouch: true, alpha: 1.0 }), "the slowness icon");
        assert_eq!(s.icon(false, false, true), None, "a held sprint key alone shows nothing by default");
    }

    #[test]
    fn an_armed_sprint_key_is_dimmed_when_asked() {
        let s = Settings { show_when_active: true, ..Settings::default() };
        assert_eq!(s.icon(false, false, true), Some(Icon { crouch: false, alpha: 0.7 }));
        assert_eq!(s.icon(true, false, true).unwrap().alpha, 1.0, "sprinting is not dimmed");
        let off = Settings { sprint: false, show_when_active: true, ..Settings::default() };
        assert_eq!(off.icon(false, false, true), None);
        assert_eq!(off.icon(true, false, false), None);
        let no_crouch = Settings { crouch: false, ..Settings::default() };
        assert_eq!(no_crouch.icon(false, true, false), None);
    }

    #[test]
    fn the_icon_sits_beside_the_hotbar_on_the_off_hand_side_plus_the_offset() {
        let s = Settings::default();
        assert_eq!(s.position(320, 240, false), (251 + 6, 220), "right handed: right of the hotbar");
        assert_eq!(s.position(320, 240, true), (69 - 18 - 6, 220), "left handed: left of it");
        let moved = Settings { offset: (3, -4), ..s };
        assert_eq!(moved.position(320, 240, false), (260, 216));
    }

    #[test]
    fn the_attack_indicator_moves_out_only_when_adjusting_and_an_icon_is_on() {
        assert_eq!(Settings::default().attack_shift(), 24);
        assert_eq!(Settings { adjust_attack_indicator: false, ..Settings::default() }.attack_shift(), 0);
        assert_eq!(Settings { sprint: false, crouch: false, ..Settings::default() }.attack_shift(), 0);
    }

    #[test]
    fn the_buttons_flip_and_the_offsets_step_and_wrap() {
        assert_eq!(click(none, 0).unwrap(), ("sprint", "false".to_owned()));
        assert_eq!(click(none, 3).unwrap(), ("offset_x", "2".to_owned()));
        let top = |k: &str| (k == "offset_y").then(|| "20".to_owned());
        assert_eq!(click(top, 4).unwrap().1, "-20");
        assert!(click(none, 9).is_none());
        assert_eq!(labels(none).len(), 6);
    }
}
