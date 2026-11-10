// ShulkerBoxTooltip, a native Horizon mod: View the contents of shulker boxes from your inventory.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_shulker_box_tooltip`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "shulker_box_tooltip";
pub const NAME: &str = "ShulkerBoxTooltip";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "View the contents of shulker boxes from your inventory.";

const KEYS: [&str; 5] = [
    "key.keyboard.left.shift",
    "key.keyboard.left.alt",
    "key.keyboard.left.control",
    "key.keyboard.right.shift",
    "key.keyboard.right.alt",
];
const POSITIONS: [&str; 4] = ["inside", "outside", "outside_top", "outside_bottom"];
const MERGES: [&str; 3] = ["separate", "first_item", "ignore"];
const ORDERS: [&str; 2] = ["stack_size", "preview_order"];
const TYPES: [&str; 3] = ["mod", "vanilla", "none"];

/// `ClientConfiguration`'s settings the port has, with the original's defaults.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub enable: bool,
    pub always_on: bool,
    pub swap_modes: bool,
    pub merge: String,
    pub order: String,
    pub max_row_size: u32,
    pub position: String,
    pub short_item_counts: bool,
    pub generic_container: bool,
    pub colored: bool,
    pub tooltip_type: String,
    pub show_key_hints: bool,
    pub preview_key: String,
    pub full_preview_key: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enable: true,
            always_on: false,
            swap_modes: false,
            merge: "separate".into(),
            order: "stack_size".into(),
            max_row_size: 9,
            position: "inside".into(),
            short_item_counts: true,
            generic_container: true,
            colored: true,
            tooltip_type: "mod".into(),
            show_key_hints: true,
            preview_key: KEYS[0].into(),
            full_preview_key: KEYS[1].into(),
        }
    }
}

fn pick(get: &impl Fn(&str) -> Option<String>, key: &str, allowed: &[&str], d: &str) -> String {
    get(key).map(|v| v.trim().to_owned()).filter(|v| allowed.contains(&v.as_str())).unwrap_or_else(|| d.to_owned())
}

fn next(list: &[&str], now: &str) -> String {
    let at = list.iter().position(|v| *v == now).map_or(0, |p| (p + 1) % list.len());
    list[at].to_owned()
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
            enable: flag("enable", d.enable),
            always_on: flag("always_on", d.always_on),
            swap_modes: flag("swap_modes", d.swap_modes),
            merge: pick(&get, "merge", &MERGES, &d.merge),
            order: pick(&get, "order", &ORDERS, &d.order),
            max_row_size: get("max_row_size").and_then(|v| v.trim().parse().ok()).filter(|v| *v > 0 && *v <= 27).unwrap_or(d.max_row_size),
            position: pick(&get, "position", &POSITIONS, &d.position),
            short_item_counts: flag("short_counts", d.short_item_counts),
            generic_container: flag("generic", d.generic_container),
            colored: flag("colored", d.colored),
            tooltip_type: pick(&get, "tooltip_type", &TYPES, &d.tooltip_type),
            show_key_hints: flag("key_hints", d.show_key_hints),
            preview_key: pick(&get, "preview_key", &KEYS, &d.preview_key),
            full_preview_key: pick(&get, "full_preview_key", &KEYS, &d.full_preview_key),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Enable Preview: {}", on(self.enable)),
            format!("Always On: {}", on(self.always_on)),
            format!("Swap Modes: {}", on(self.swap_modes)),
            format!("Compact Preview Item Components: {}", self.merge),
            format!("Compact Preview Item Order: {}", self.order),
            format!("Maximum Row Size: {}", self.max_row_size),
            format!("Window Position: {}", self.position),
            format!("Abbreviate large item counts: {}", on(self.short_item_counts)),
            format!("Generic Container Preview: {}", on(self.generic_container)),
            format!("Colored Preview Window: {}", on(self.colored)),
            format!("Tooltip Type: {}", self.tooltip_type),
            format!("Show Key Hints: {}", on(self.show_key_hints)),
            format!("Open Stack Preview: {}", self.preview_key),
            format!("Open Stack Full Preview: {}", self.full_preview_key),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let flip = |v: &mut bool, key: &'static str| {
            *v = !*v;
            Some((key, v.to_string()))
        };
        match i {
            0 => flip(&mut self.enable, "enable"),
            1 => flip(&mut self.always_on, "always_on"),
            2 => flip(&mut self.swap_modes, "swap_modes"),
            3 => {
                self.merge = next(&MERGES, &self.merge);
                Some(("merge", self.merge.clone()))
            }
            4 => {
                self.order = next(&ORDERS, &self.order);
                Some(("order", self.order.clone()))
            }
            5 => {
                self.max_row_size = if self.max_row_size >= 9 { 3 } else { self.max_row_size + 3 };
                Some(("max_row_size", self.max_row_size.to_string()))
            }
            6 => {
                self.position = next(&POSITIONS, &self.position);
                Some(("position", self.position.clone()))
            }
            7 => flip(&mut self.short_item_counts, "short_counts"),
            8 => flip(&mut self.generic_container, "generic"),
            9 => flip(&mut self.colored, "colored"),
            10 => {
                self.tooltip_type = next(&TYPES, &self.tooltip_type);
                Some(("tooltip_type", self.tooltip_type.clone()))
            }
            11 => flip(&mut self.show_key_hints, "key_hints"),
            12 => {
                self.preview_key = next(&KEYS, &self.preview_key);
                Some(("preview_key", self.preview_key.clone()))
            }
            13 => {
                self.full_preview_key = next(&KEYS, &self.full_preview_key);
                Some(("full_preview_key", self.full_preview_key.clone()))
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
        assert!(s.enable && !s.always_on && !s.swap_modes && s.short_item_counts && s.generic_container && s.colored && s.show_key_hints);
        assert_eq!((s.merge.as_str(), s.order.as_str(), s.max_row_size, s.position.as_str(), s.tooltip_type.as_str()), ("separate", "stack_size", 9, "inside", "mod"));
        assert_eq!((s.preview_key.as_str(), s.full_preview_key.as_str()), ("key.keyboard.left.shift", "key.keyboard.left.alt"));
    }

    #[test]
    fn there_is_a_button_for_every_setting() {
        assert_eq!(labels(none).len(), 14);
        assert_eq!(click(none, 0).unwrap(), ("enable", "false".to_owned()));
        assert_eq!(click(none, 3).unwrap(), ("merge", "first_item".to_owned()));
        assert_eq!(click(none, 6).unwrap(), ("position", "outside".to_owned()));
        assert_eq!(click(none, 12).unwrap(), ("preview_key", "key.keyboard.left.alt".to_owned()));
        assert!(click(none, 14).is_none());
    }

    #[test]
    fn cycles_wrap_around() {
        let last = |k: &str| (k == "position").then(|| "outside_bottom".to_owned());
        assert_eq!(click(last, 6).unwrap().1, "inside");
        let wide = |k: &str| (k == "max_row_size").then(|| "9".to_owned());
        assert_eq!(click(wide, 5).unwrap().1, "3", "the row size steps 3, 6, 9 and wraps");
    }

    #[test]
    fn junk_falls_back_to_the_defaults() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
    }
}
