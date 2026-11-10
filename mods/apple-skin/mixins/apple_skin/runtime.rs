// AppleSkin, a native Horizon mod: Food/hunger-related HUD improvements.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_apple_skin`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "apple_skin";
pub const NAME: &str = "AppleSkin";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Food/hunger-related HUD improvements.";

/// The flash alphas the button steps through (the file takes any 0 to 1).
pub const ALPHAS: [f32; 5] = [0.25, 0.5, 0.65, 0.75, 1.0];

/// The HUD settings of `ModConfig`, in the order of its screen, with the defaults.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub show_food_values_hud_overlay: bool,
    pub show_food_values_hud_overlay_when_offhand: bool,
    pub show_food_health_hud_overlay: bool,
    pub show_food_exhaustion_hud_underlay: bool,
    pub show_saturation_hud_overlay: bool,
    pub show_vanilla_animations_overlay: bool,
    pub max_hud_overlay_flash_alpha: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_food_values_hud_overlay: true,
            show_food_values_hud_overlay_when_offhand: true,
            show_food_health_hud_overlay: true,
            show_food_exhaustion_hud_underlay: true,
            show_saturation_hud_overlay: true,
            show_vanilla_animations_overlay: true,
            max_hud_overlay_flash_alpha: 0.65,
        }
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
        Self {
            show_food_values_hud_overlay: flag("held_food", d.show_food_values_hud_overlay),
            show_food_values_hud_overlay_when_offhand: flag("offhand", d.show_food_values_hud_overlay_when_offhand),
            show_food_health_hud_overlay: flag("health", d.show_food_health_hud_overlay),
            show_food_exhaustion_hud_underlay: flag("exhaustion", d.show_food_exhaustion_hud_underlay),
            show_saturation_hud_overlay: flag("saturation", d.show_saturation_hud_overlay),
            show_vanilla_animations_overlay: flag("animations", d.show_vanilla_animations_overlay),
            max_hud_overlay_flash_alpha: get("flash_alpha")
                .and_then(|v| v.trim().parse::<f32>().ok())
                .filter(|v| v.is_finite())
                .unwrap_or(d.max_hud_overlay_flash_alpha),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Show hunger restored from held food: {}", on(self.show_food_values_hud_overlay)),
            format!("Show overlays for food in off-hand: {}", on(self.show_food_values_hud_overlay_when_offhand)),
            format!("Show estimated health overlay: {}", on(self.show_food_health_hud_overlay)),
            format!("Show exhaustion underlay: {}", on(self.show_food_exhaustion_hud_underlay)),
            format!("Show saturation overlay: {}", on(self.show_saturation_hud_overlay)),
            format!("Animate HUD icons to match Minecraft: {}", on(self.show_vanilla_animations_overlay)),
            format!("Max alpha of flashing HUD icons: {}", self.max_hud_overlay_flash_alpha),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let flip = |v: &mut bool, key: &'static str| {
            *v = !*v;
            Some((key, v.to_string()))
        };
        match i {
            0 => flip(&mut self.show_food_values_hud_overlay, "held_food"),
            1 => flip(&mut self.show_food_values_hud_overlay_when_offhand, "offhand"),
            2 => flip(&mut self.show_food_health_hud_overlay, "health"),
            3 => flip(&mut self.show_food_exhaustion_hud_underlay, "exhaustion"),
            4 => flip(&mut self.show_saturation_hud_overlay, "saturation"),
            5 => flip(&mut self.show_vanilla_animations_overlay, "animations"),
            6 => {
                let at = ALPHAS.iter().position(|a| (*a - self.max_hud_overlay_flash_alpha).abs() < 1e-6).map_or(0, |p| (p + 1) % ALPHAS.len());
                self.max_hud_overlay_flash_alpha = ALPHAS[at];
                Some(("flash_alpha", self.max_hud_overlay_flash_alpha.to_string()))
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
        assert!(s.show_food_values_hud_overlay && s.show_food_values_hud_overlay_when_offhand && s.show_food_health_hud_overlay);
        assert!(s.show_food_exhaustion_hud_underlay && s.show_saturation_hud_overlay && s.show_vanilla_animations_overlay);
        assert_eq!(s.max_hud_overlay_flash_alpha, 0.65);
    }

    #[test]
    fn the_buttons_flip_in_the_screens_order() {
        let l = labels(none);
        assert_eq!(l.len(), 7);
        assert!(l[0].starts_with("Show hunger restored") && l[3].starts_with("Show exhaustion underlay") && l[6].starts_with("Max alpha"));
        assert_eq!(click(none, 0).unwrap(), ("held_food", "false".to_owned()));
        assert_eq!(click(none, 4).unwrap(), ("saturation", "false".to_owned()));
        assert!(click(none, 9).is_none());
    }

    #[test]
    fn the_alpha_button_walks_its_steps_and_wraps() {
        assert_eq!(click(none, 6).unwrap(), ("flash_alpha", "0.75".to_owned()), "0.65 is third, 0.75 follows");
        let top = |k: &str| (k == "flash_alpha").then(|| "1".to_owned());
        assert_eq!(click(top, 6).unwrap().1, "0.25");
    }

    #[test]
    fn junk_falls_back_to_the_defaults() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
    }
}
