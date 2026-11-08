// Coords Plus, a native Horizon mod: Your position with the biome you stand in and the matching Nether or Overworld coordinates, in a box on the side you choose.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_coords_plus`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "coords_plus";
pub const NAME: &str = "Coords Plus";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Your position with the biome you stand in and the matching Nether or Overworld coordinates, in a box on the side you choose.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("right", "Right side", false),
    ("coords", "Coordinates", true),
    ("nether", "Nether / Overworld conversion", true),
    ("biome", "Biome", true),
];

thread_local! {
    /// Read once through the game's settings files, then kept.
    static CHOSEN: RefCell<Option<Vec<bool>>> = const { RefCell::new(None) };
}

/// A stored value: `true` or `false`, else the default.
pub fn parse(stored: Option<String>, default: bool) -> bool {
    stored.map_or(default, |v| match v.trim() {
        "true" => true,
        "false" => false,
        _ => default,
    })
}

fn chosen(get: impl Fn(&str) -> Option<String>) -> Vec<bool> {
    CHOSEN.with_borrow_mut(|c| {
        c.get_or_insert_with(|| {
            OPTIONS
                .iter()
                .map(|(key, _, default)| parse(get(key), *default))
                .collect()
        })
        .clone()
    })
}

/// Whether option `key` is on.
pub fn is(get: impl Fn(&str) -> Option<String>, key: &str) -> bool {
    OPTIONS
        .iter()
        .position(|o| o.0 == key)
        .is_some_and(|i| chosen(get)[i])
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let on = chosen(get);
    OPTIONS
        .iter()
        .zip(on)
        .map(|((_, label, _), on)| format!("{label}: {}", if on { "ON" } else { "OFF" }))
        .collect()
}

/// A click on button `i`: flips it and returns what to store (key, value).
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut on = chosen(get);
    let (key, _, _) = *OPTIONS.get(i)?;
    on[i] = !on[i];
    let value = on[i].to_string();
    CHOSEN.with_borrow_mut(|c| *c = Some(on));
    Some((key, value))
}

/// Forgets what was read (tests only).
#[cfg(test)]
pub fn forget_options() {
    CHOSEN.with_borrow_mut(|c| *c = None);
}
/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "coords";

/// What the box can show.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reading {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// `minecraft:overworld`, `minecraft:the_nether` or `minecraft:the_end`.
    pub dimension: String,
    /// The biome id (`minecraft:old_growth_birch_forest`), empty when its chunk is not loaded.
    pub biome: String,
}

/// `minecraft:old_growth_birch_forest` as `Old Growth Birch Forest`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The matching position in the other dimension, as `(label, x, z)`: the Nether is eight times smaller than the
/// Overworld. None in the End, which has no counterpart.
pub fn other_world(dimension: &str, x: f64, z: f64) -> Option<(&'static str, i64, i64)> {
    match dimension {
        "minecraft:overworld" => Some(("Nether", (x / 8.0).floor() as i64, (z / 8.0).floor() as i64)),
        "minecraft:the_nether" => Some(("Overworld", (x * 8.0).floor() as i64, (z * 8.0).floor() as i64)),
        _ => None,
    }
}

/// The box's lines: `show` is coordinates, conversion, biome.
pub fn lines(r: &Reading, show: [bool; 3]) -> Vec<String> {
    let mut out = Vec::new();
    if show[0] {
        out.push(format!("XYZ {:.1} / {:.1} / {:.1}", r.x, r.y, r.z));
    }
    if show[1]
        && let Some((label, x, z)) = other_world(&r.dimension, r.x, r.z)
    {
        out.push(format!("{label} {x} / {z}"));
    }
    if show[2] && !r.biome.is_empty() {
        out.push(format!("Biome {}", title(&r.biome)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(dimension: &str, x: f64, z: f64) -> Reading {
        Reading { x, y: 64.0, z, dimension: dimension.into(), biome: "minecraft:old_growth_birch_forest".into() }
    }

    #[test]
    fn the_nether_is_eight_times_smaller_and_the_overworld_eight_times_larger() {
        assert_eq!(other_world("minecraft:overworld", 800.0, -160.0), Some(("Nether", 100, -20)));
        assert_eq!(other_world("minecraft:the_nether", 100.0, -20.0), Some(("Overworld", 800, -160)));
        assert_eq!(other_world("minecraft:the_end", 1.0, 1.0), None);
    }

    #[test]
    fn negative_positions_round_down_like_block_coordinates() {
        assert_eq!(other_world("minecraft:overworld", -1.0, -9.0), Some(("Nether", -1, -2)));
        assert_eq!(other_world("minecraft:overworld", 7.9, 8.0), Some(("Nether", 0, 1)));
    }

    #[test]
    fn a_biome_id_reads_as_words() {
        assert_eq!(title("minecraft:old_growth_birch_forest"), "Old Growth Birch Forest");
        assert_eq!(title("plains"), "Plains");
        assert_eq!(title(""), "");
    }

    #[test]
    fn only_the_picked_lines_show_and_an_unloaded_biome_has_no_line() {
        let r = at("minecraft:overworld", 801.5, -160.0);
        assert_eq!(
            lines(&r, [true, true, true]),
            ["XYZ 801.5 / 64.0 / -160.0", "Nether 100 / -20", "Biome Old Growth Birch Forest"]
        );
        assert_eq!(lines(&r, [false, true, false]), ["Nether 100 / -20"]);
        assert_eq!(lines(&Reading { biome: String::new(), ..r.clone() }, [false, false, true]), Vec::<String>::new());
        assert_eq!(lines(&at("minecraft:the_end", 0.0, 0.0), [false, true, false]), Vec::<String>::new());
        assert!(lines(&r, [false; 3]).is_empty());
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
        assert_eq!(is(none, OPTIONS[0].0), OPTIONS[0].2);
        forget_options();
        let stored = |k: &str| (k == OPTIONS[0].0).then(|| "false".to_owned());
        assert!(!is(stored, OPTIONS[0].0));
        forget_options();
        assert!(parse(Some("junk".into()), true));
        assert!(!parse(Some(" false\n".into()), true));
    }

    #[test]
    fn a_click_flips_and_reports_what_to_store() {
        forget_options();
        let none = |_: &str| None;
        let (key, value) = click(none, 0).expect("button 0 exists");
        assert_eq!(key, OPTIONS[0].0);
        assert_eq!(value, (!OPTIONS[0].2).to_string());
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
