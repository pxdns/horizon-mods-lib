// Armor HUD, a native Horizon mod: Minimalist vanilla-like widget to display your armor items and their durability.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_armor_hud`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "armor_hud";
pub const NAME: &str = "Armor HUD";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Minimalist vanilla-like widget to display your armor items and their durability.";

/// The toggle key's name in Controls; unbound by default (the original registers it as unknown).
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.unknown";

/// One setting: its key in the mod's file, its label (the original's), and what its button steps through.
pub enum Kind {
    Flag,
    Cycle(&'static [&'static str]),
}

pub struct Row {
    pub key: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub default: &'static str,
}

const NUMBERS: &[&str] = &["0", "2", "5", "10", "20", "50"];

/// `ArmorHudConfig`'s settings the port applies, in the original's order, with its defaults.
pub const ROWS: &[Row] = &[
    Row { key: "enabled", label: "Enabled", kind: Kind::Flag, default: "true" },
    Row { key: "anchor", label: "Anchor", kind: Kind::Cycle(&["hotbar", "top_center", "top", "vertical_center", "bottom"]), default: "hotbar" },
    Row { key: "side", label: "Side", kind: Kind::Cycle(&["left", "right"]), default: "left" },
    Row { key: "offset_x", label: "X Offset", kind: Kind::Cycle(NUMBERS), default: "0" },
    Row { key: "offset_y", label: "Y Offset", kind: Kind::Cycle(NUMBERS), default: "0" },
    Row { key: "style", label: "Style", kind: Kind::Cycle(&["hotbar", "rounded_corners", "rounded"]), default: "hotbar" },
    Row { key: "orientation", label: "Orientation", kind: Kind::Cycle(&["horizontal", "vertical"]), default: "horizontal" },
    Row { key: "widget_shown", label: "Show Widget", kind: Kind::Cycle(&["not_empty", "always", "if_any_present", "damaged_pieces"]), default: "not_empty" },
    Row { key: "offhand_slot", label: "Offhand Slot", kind: Kind::Cycle(&["adhere", "always_ignore", "always_leave_space"]), default: "adhere" },
    Row { key: "durability", label: "Durability Display", kind: Kind::Cycle(&["bar", "numeric", "percentage"]), default: "bar" },
    Row { key: "mainhand_durability", label: "Show Main Hand Durability", kind: Kind::Flag, default: "false" },
    Row { key: "offhand_durability", label: "Show Offhand Durability", kind: Kind::Flag, default: "true" },
    Row { key: "reversed", label: "Reversed", kind: Kind::Flag, default: "false" },
    Row { key: "icons", label: "Show Icons", kind: Kind::Flag, default: "true" },
    Row { key: "warning", label: "Show Warning", kind: Kind::Flag, default: "true" },
    Row { key: "min_value", label: "Min Durability", kind: Kind::Cycle(&["20", "0", "10", "50", "100"]), default: "20" },
    Row { key: "min_percent", label: "Min Durability (percent)", kind: Kind::Cycle(&["0.1", "0", "0.05", "0.2", "0.3"]), default: "0.1" },
    Row { key: "bob", label: "Icon Bob Intensity", kind: Kind::Cycle(&["3", "0", "1", "2", "4", "5"]), default: "3" },
];

/// A setting's value from the file, or its default when missing or not one of the row's.
pub fn value(get: &impl Fn(&str) -> Option<String>, row: &Row) -> String {
    let v = get(row.key).map(|v| v.trim().to_owned());
    match (&row.kind, v) {
        (Kind::Flag, Some(v)) if v == "true" || v == "false" => v,
        (Kind::Cycle(list), Some(v)) if list.contains(&v.as_str()) => v,
        _ => row.default.to_owned(),
    }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    ROWS.iter()
        .map(|r| {
            let v = value(&get, r);
            let shown = match r.kind {
                Kind::Flag => (if v == "true" { "ON" } else { "OFF" }).to_owned(),
                Kind::Cycle(_) => v,
            };
            format!("{}: {shown}", r.label)
        })
        .collect()
}

/// A press on button `i`: the setting to write and its next value.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let row = ROWS.get(i)?;
    let now = value(&get, row);
    let next = match row.kind {
        Kind::Flag => (now != "true").to_string(),
        Kind::Cycle(list) => list[list.iter().position(|v| *v == now).map_or(0, |p| (p + 1) % list.len())].to_owned(),
    };
    Some((row.key, next))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let get = none;
        let v = |k: &str| value(&get, ROWS.iter().find(|r| r.key == k).unwrap());
        assert_eq!((v("enabled").as_str(), v("anchor").as_str(), v("side").as_str(), v("style").as_str()), ("true", "hotbar", "left", "hotbar"));
        assert_eq!((v("widget_shown").as_str(), v("offhand_slot").as_str(), v("durability").as_str()), ("not_empty", "adhere", "bar"));
        assert_eq!((v("min_value").as_str(), v("min_percent").as_str(), v("bob").as_str()), ("20", "0.1", "3"));
        assert_eq!((v("mainhand_durability").as_str(), v("offhand_durability").as_str()), ("false", "true"));
    }

    #[test]
    fn there_is_a_button_for_each_setting_in_the_originals_order() {
        let l = labels(none);
        assert_eq!(l.len(), 18);
        assert_eq!(l[0], "Enabled: ON");
        assert_eq!(l[1], "Anchor: hotbar");
        assert_eq!(l[9], "Durability Display: bar");
        assert_eq!(l[17], "Icon Bob Intensity: 3");
    }

    #[test]
    fn a_button_flips_or_steps_to_the_next_value_and_wraps() {
        assert_eq!(click(none, 0).unwrap(), ("enabled", "false".to_owned()));
        assert_eq!(click(none, 1).unwrap(), ("anchor", "top_center".to_owned()));
        let last = |k: &str| (k == "anchor").then(|| "bottom".to_owned());
        assert_eq!(click(last, 1).unwrap().1, "hotbar");
        assert!(click(none, 18).is_none());
    }

    #[test]
    fn junk_in_the_file_falls_back_to_the_default() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(labels(junk), labels(none));
    }
}
