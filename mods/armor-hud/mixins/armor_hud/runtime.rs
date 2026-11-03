// Armor HUD, a native Horizon mod: Your armor pieces and how much durability each has left, in a corner of the screen.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_armor_hud`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "armor_hud";
pub const NAME: &str = "Armor HUD";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Your armor pieces and how much durability each has left, in a corner of the screen.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("percent", "Show percent", true),
    ("full", "Hide undamaged pieces", false),
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

/// `minecraft:diamond_helmet` as `Diamond Helmet`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "armor";

/// One worn piece.
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub slot: &'static str,
    pub name: String,
    pub damage: u32,
    pub max: u32,
}

impl Piece {
    pub fn new(slot: &'static str, id: &str, damage: u32, max: u32) -> Self {
        Self {
            slot,
            name: title(id),
            damage,
            max,
        }
    }

    /// Durability left; unbreakable pieces (max 0) read as full.
    pub fn left(&self) -> u32 {
        self.max.saturating_sub(self.damage)
    }

    /// 0 to 100.
    pub fn percent(&self) -> u32 {
        (self.left() * 100).checked_div(self.max).unwrap_or(100)
    }

    /// Green above half, yellow above a quarter, red below.
    pub fn ink(&self) -> u32 {
        match self.percent() {
            51.. => 0x55_FF55,
            26..=50 => 0xFF_FF55,
            _ => 0xFF_5555,
        }
    }

    pub fn is_full(&self) -> bool {
        self.damage == 0
    }
}

/// The box's lines with their colours.
pub fn lines(pieces: &[Piece], as_percent: bool, hide_full: bool) -> Vec<(String, u32)> {
    pieces
        .iter()
        .filter(|p| !(hide_full && p.is_full()))
        .map(|p| {
            let value = match (p.max, as_percent) {
                (0, _) => "unbreakable".to_owned(),
                (_, true) => format!("{}%", p.percent()),
                (_, false) => format!("{}/{}", p.left(), p.max),
            };
            (format!("{}: {} {}", p.slot, p.name, value), p.ink())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_become_titles() {
        assert_eq!(title("minecraft:diamond_helmet"), "Diamond Helmet");
        assert_eq!(title("totem_of_undying"), "Totem Of Undying");
        assert_eq!(title(""), "");
    }

    #[test]
    fn durability_reads_as_left_percent_and_colour() {
        let p = Piece::new("Head", "minecraft:iron_helmet", 55, 165);
        assert_eq!((p.left(), p.percent()), (110, 66));
        assert_eq!(p.ink(), 0x55_FF55);
        assert_eq!(Piece::new("Head", "x", 100, 165).ink(), 0xFF_FF55);
        assert_eq!(Piece::new("Head", "x", 160, 165).ink(), 0xFF_5555);
        assert_eq!(
            Piece::new("Head", "x", 999, 165).left(),
            0,
            "never below zero"
        );
        assert_eq!(Piece::new("Head", "x", 5, 0).percent(), 100, "unbreakable");
    }

    #[test]
    fn lines_follow_the_options() {
        let list = vec![
            Piece::new("Head", "minecraft:iron_helmet", 55, 165),
            Piece::new("Feet", "minecraft:iron_boots", 0, 195),
        ];
        assert_eq!(lines(&list, true, false)[0].0, "Head: Iron Helmet 66%");
        assert_eq!(lines(&list, false, false)[1].0, "Feet: Iron Boots 195/195");
        assert_eq!(lines(&list, true, true).len(), 1, "undamaged pieces hidden");
        assert_eq!(
            lines(&[Piece::new("Chest", "elytra", 0, 0)], false, false)[0].0,
            "Chest: Elytra unbreakable"
        );
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
        assert_eq!(is(none, key), !OPTIONS[0].2);
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
