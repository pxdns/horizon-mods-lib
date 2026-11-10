// Potion Counter, a native Horizon mod: How many potions, splash potions and lingering potions you carry.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_potion_counter`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "potion_counter";
pub const NAME: &str = "Potion Counter";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "How many potions, splash potions and lingering potions you carry.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("split", "Split by kind", true),
    ("empty", "Show when none", false),
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
pub const COMMAND: &str = "potions";

/// The potion kinds an item can be.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub drink: u32,
    pub splash: u32,
    pub lingering: u32,
}

impl Counts {
    /// Adds a stack of `count` items called `id`.
    pub fn add(&mut self, id: &str, count: u32) {
        match id.strip_prefix("minecraft:").unwrap_or(id) {
            "potion" => self.drink += count,
            "splash_potion" => self.splash += count,
            "lingering_potion" => self.lingering += count,
            _ => {}
        }
    }

    pub fn total(&self) -> u32 {
        self.drink + self.splash + self.lingering
    }
}

/// The box's lines.
pub fn lines(c: Counts, split: bool, show_empty: bool) -> Vec<String> {
    if c.total() == 0 && !show_empty {
        return Vec::new();
    }
    if !split {
        return vec![format!("Potions {}", c.total())];
    }
    let mut out = vec![format!("Potions {}", c.drink)];
    if c.splash > 0 || show_empty {
        out.push(format!("Splash {}", c.splash));
    }
    if c.lingering > 0 {
        out.push(format!("Lingering {}", c.lingering));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_sort_by_kind() {
        let mut c = Counts::default();
        c.add("minecraft:potion", 3);
        c.add("minecraft:splash_potion", 5);
        c.add("lingering_potion", 1);
        c.add("minecraft:glass_bottle", 9);
        assert_eq!((c.drink, c.splash, c.lingering, c.total()), (3, 5, 1, 9));
    }

    #[test]
    fn lines_follow_the_options() {
        let c = Counts {
            drink: 2,
            splash: 4,
            lingering: 0,
        };
        assert_eq!(lines(c, true, false), ["Potions 2", "Splash 4"]);
        assert_eq!(lines(c, false, false), ["Potions 6"]);
        assert!(lines(Counts::default(), true, false).is_empty());
        assert_eq!(
            lines(Counts::default(), true, true),
            ["Potions 0", "Splash 0"]
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
