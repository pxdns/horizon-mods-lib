// Enchantment Descriptions, a native Horizon mod: Adds a line to enchanted items' tooltips saying what each enchantment does.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_enchantment_descriptions`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "enchantment_descriptions";
pub const NAME: &str = "Enchantment Descriptions";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Adds a line to enchanted items' tooltips saying what each enchantment does.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("books", "On enchanted books too", true),
    ("items", "On enchanted items", true),
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

/// What each vanilla enchantment does, by id without the namespace.
pub const DESCRIPTIONS: &[(&str, &str)] = &[
    ("protection", "Reduces most damage."),
    (
        "fire_protection",
        "Reduces fire and lava damage and burning time.",
    ),
    ("feather_falling", "Reduces fall damage."),
    (
        "blast_protection",
        "Reduces explosion damage and knockback.",
    ),
    (
        "projectile_protection",
        "Reduces damage from arrows and other projectiles.",
    ),
    ("respiration", "Extends breathing time underwater."),
    ("aqua_affinity", "Mines underwater as fast as on land."),
    ("thorns", "Hurts attackers who hit you."),
    ("depth_strider", "Moves faster underwater."),
    ("frost_walker", "Freezes water under you into ice."),
    ("binding_curse", "Cannot be taken off."),
    ("soul_speed", "Moves faster on soul sand and soul soil."),
    ("swift_sneak", "Sneaks faster."),
    ("sharpness", "Increases melee damage."),
    ("smite", "Increases damage to undead."),
    (
        "bane_of_arthropods",
        "Increases damage to spiders and silverfish, and slows them.",
    ),
    ("knockback", "Knocks targets further back."),
    ("fire_aspect", "Sets targets on fire."),
    ("looting", "Mobs drop more loot."),
    ("sweeping_edge", "Increases sweeping attack damage."),
    ("efficiency", "Mines faster."),
    ("silk_touch", "Mined blocks drop themselves."),
    ("unbreaking", "Chance not to lose durability."),
    ("fortune", "Increases drops from blocks."),
    ("power", "Increases arrow damage."),
    ("punch", "Knocks targets back further with arrows."),
    ("flame", "Arrows set targets on fire."),
    ("infinity", "Shooting does not use arrows."),
    ("luck_of_the_sea", "Better fishing loot."),
    ("lure", "Fish bite faster."),
    ("loyalty", "A thrown trident returns."),
    ("impaling", "More damage to aquatic mobs."),
    (
        "riptide",
        "Launches you with a thrown trident in water or rain.",
    ),
    (
        "channeling",
        "Calls lightning on a thrown trident hit in a storm.",
    ),
    ("multishot", "Shoots three projectiles at once."),
    ("quick_charge", "Reloads a crossbow faster."),
    ("piercing", "Projectiles pass through targets."),
    ("density", "More smash damage per block fallen."),
    ("breach", "Ignores part of the target's armor."),
    ("wind_burst", "A smash attack launches you upward."),
    ("mending", "Repairs the item with experience."),
    ("vanishing_curse", "Disappears when you die."),
    ("lunge", "Launches you forward with a spear attack."),
];

/// `minecraft:fire_aspect` as `Fire Aspect`.
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

/// The description of a vanilla enchantment, or none.
pub fn description(id: &str) -> Option<&'static str> {
    let bare = id.strip_prefix("minecraft:").unwrap_or(id);
    DESCRIPTIONS
        .iter()
        .find(|(k, _)| *k == bare)
        .map(|(_, d)| *d)
}

/// The lines to add for the enchantments of one stack: `Sharpness: Increases melee damage.`
pub fn lines(enchantments: &[String]) -> Vec<String> {
    enchantments
        .iter()
        .filter_map(|id| description(id).map(|d| format!("{}: {d}", title(id))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_description_is_unique_and_short() {
        let mut seen = std::collections::BTreeSet::new();
        for (id, text) in DESCRIPTIONS {
            assert!(seen.insert(*id), "{id} twice");
            assert!(!text.is_empty() && text.len() <= 64, "{id}: {text}");
            assert!(text.ends_with('.'));
        }
        assert!(DESCRIPTIONS.len() >= 40);
    }

    #[test]
    fn titles_and_lookups() {
        assert_eq!(title("minecraft:fire_aspect"), "Fire Aspect");
        assert_eq!(
            description("minecraft:mending"),
            Some("Repairs the item with experience.")
        );
        assert_eq!(description("mending"), description("minecraft:mending"));
        assert_eq!(description("mymod:thing"), None);
    }

    #[test]
    fn lines_skip_unknown_enchantments() {
        let have = vec![
            "minecraft:sharpness".to_owned(),
            "mymod:thing".to_owned(),
            "minecraft:unbreaking".to_owned(),
        ];
        assert_eq!(
            lines(&have),
            [
                "Sharpness: Increases melee damage.",
                "Unbreaking: Chance not to lose durability."
            ]
        );
        assert!(lines(&[]).is_empty());
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
