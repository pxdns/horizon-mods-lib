// Entity Texture Features, a native Horizon mod: Random texture variants and emissive layers for mobs, from the textures a resource pack provides.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_entity_texture_features`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "entity_texture_features";
pub const NAME: &str = "Entity Texture Features";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Random texture variants and emissive layers for mobs, from the textures a resource pack provides.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("random", "Random texture variants", true),
    ("emissive", "Emissive layers", true),
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

use std::cell::{Cell, RefCell as StateCell};
use std::collections::{BTreeMap, BTreeSet};

thread_local! {
    static RANDOM: Cell<bool> = const { Cell::new(true) };
    static EMISSIVE: Cell<bool> = const { Cell::new(true) };
    /// Base texture name to its variants (the base is not in the list), in number order.
    static VARIANTS: StateCell<BTreeMap<String, Vec<String>>> = const { StateCell::new(BTreeMap::new()) };
    /// Textures that have an `_e` emissive twin.
    static GLOW: StateCell<BTreeSet<String>> = const { StateCell::new(BTreeSet::new()) };
    static INSIDE_BODY: Cell<bool> = const { Cell::new(false) };
    static INSIDE_LAYERS: Cell<bool> = const { Cell::new(false) };
}

/// Sets what the mod does (the game tells the renderer each tick).
pub fn configure(random: bool, emissive: bool) {
    RANDOM.with(|v| v.set(random));
    EMISSIVE.with(|v| v.set(emissive));
}

pub fn random_on() -> bool {
    RANDOM.with(Cell::get)
}

pub fn emissive_on() -> bool {
    EMISSIVE.with(Cell::get)
}

/// Marks the nested call of the body hook; false when this is the nested one.
pub fn enter_body() -> bool {
    if INSIDE_BODY.with(Cell::get) {
        return false;
    }
    INSIDE_BODY.with(|i| i.set(true));
    true
}

pub fn leave_body() {
    INSIDE_BODY.with(|i| i.set(false));
}

/// The same for the layers hook, which calls the original `layers` again.
pub fn enter_layers() -> bool {
    if INSIDE_LAYERS.with(Cell::get) {
        return false;
    }
    INSIDE_LAYERS.with(|i| i.set(true));
    true
}

pub fn leave_layers() {
    INSIDE_LAYERS.with(|i| i.set(false));
}

/// Splits `entity/zombie/zombie12` into (`entity/zombie/zombie`, 12): a name ending in a number of 2 or more.
pub fn split_number(name: &str) -> Option<(&str, u32)> {
    let digits = name.len() - name.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 || digits > 3 {
        return None;
    }
    let (base, number) = name.split_at(name.len() - digits);
    let n: u32 = number.parse().ok()?;
    (n >= 2 && !base.is_empty() && !base.ends_with('/')).then_some((base, n))
}

/// Builds the index from the names of the textures that exist: variants by base, and which textures have an emissive twin.
pub fn index<'a>(
    names: impl Iterator<Item = &'a str>,
) -> (BTreeMap<String, Vec<String>>, BTreeSet<String>) {
    let all: BTreeSet<&str> = names.filter(|n| n.starts_with("entity/")).collect();
    let mut variants: BTreeMap<String, Vec<(u32, String)>> = BTreeMap::new();
    for name in &all {
        if let Some((base, n)) = split_number(name)
            && all.contains(base)
        {
            variants
                .entry(base.to_owned())
                .or_default()
                .push((n, (*name).to_owned()));
        }
    }
    let variants = variants
        .into_iter()
        .map(|(base, mut list)| {
            list.sort();
            (base, list.into_iter().map(|(_, name)| name).collect())
        })
        .collect();
    let glow = all
        .iter()
        .filter_map(|n| n.strip_suffix("_e"))
        .filter(|base| all.contains(base))
        .map(str::to_owned)
        .collect();
    (variants, glow)
}

/// Stores a new index.
pub fn set_index(variants: BTreeMap<String, Vec<String>>, glow: BTreeSet<String>) {
    VARIANTS.with_borrow_mut(|v| *v = variants);
    GLOW.with_borrow_mut(|g| *g = glow);
}

/// The texture a mob with entity id `id` uses in place of `base`: the base itself or one of its variants, the same every time.
pub fn pick(base: &str, id: u32) -> String {
    VARIANTS.with_borrow(|v| match v.get(base) {
        Some(list) if !list.is_empty() => {
            // Mix the id so neighbouring ids do not take neighbouring variants.
            let h = id.wrapping_mul(0x9E37_79B1) >> 7;
            let at = h as usize % (list.len() + 1);
            if at == 0 {
                base.to_owned()
            } else {
                list[at - 1].clone()
            }
        }
        _ => base.to_owned(),
    })
}

/// The emissive twin of a texture, if the pack has one.
pub fn glow_of(texture: &str) -> Option<String> {
    GLOW.with_borrow(|g| g.contains(texture).then(|| format!("{texture}_e")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names() -> Vec<&'static str> {
        vec![
            "entity/zombie/zombie",
            "entity/zombie/zombie2",
            "entity/zombie/zombie3",
            "entity/zombie/zombie_e",
            "entity/cow/cow",
            "entity/cow/cow2",
            "entity/sign/oak",
            "entity/sign/oak1",
            "entity/creeper/creeper",
            "gui/sprites/x2",
            "entity/chest/normal_e",
        ]
    }

    #[test]
    fn numbered_names_split_at_two_and_up() {
        assert_eq!(
            split_number("entity/zombie/zombie2"),
            Some(("entity/zombie/zombie", 2))
        );
        assert_eq!(
            split_number("entity/zombie/zombie12"),
            Some(("entity/zombie/zombie", 12))
        );
        assert_eq!(
            split_number("entity/sign/oak1"),
            None,
            "1 is the base itself"
        );
        assert_eq!(split_number("entity/zombie/zombie"), None);
        assert_eq!(
            split_number("entity/zombie/2"),
            None,
            "a bare number is not a variant"
        );
        assert_eq!(split_number("entity/x/y1234"), None);
    }

    #[test]
    fn the_index_keeps_only_entity_textures_whose_base_exists() {
        let (variants, glow) = index(names().into_iter());
        assert_eq!(
            variants.get("entity/zombie/zombie").unwrap(),
            &["entity/zombie/zombie2", "entity/zombie/zombie3"]
        );
        assert_eq!(
            variants.get("entity/cow/cow").unwrap(),
            &["entity/cow/cow2"]
        );
        assert!(
            !variants.contains_key("entity/sign/oak"),
            "oak1 is not a variant"
        );
        assert!(!variants.contains_key("gui/sprites/x"));
        assert_eq!(
            glow.iter().map(String::as_str).collect::<Vec<_>>(),
            ["entity/zombie/zombie"],
            "an emissive twin needs its base"
        );
    }

    #[test]
    fn a_mob_keeps_its_variant_and_all_variants_get_used() {
        let (variants, glow) = index(names().into_iter());
        set_index(variants, glow);
        for id in [1, 17, 4000] {
            assert_eq!(
                pick("entity/zombie/zombie", id),
                pick("entity/zombie/zombie", id),
                "stable"
            );
        }
        let picked: std::collections::BTreeSet<String> = (0..200)
            .map(|id| pick("entity/zombie/zombie", id))
            .collect();
        assert_eq!(picked.len(), 3, "the base and both variants");
        assert!(picked.contains("entity/zombie/zombie"));
        assert_eq!(
            pick("entity/creeper/creeper", 5),
            "entity/creeper/creeper",
            "no variants, no change"
        );
        assert_eq!(pick("entity/unknown", 5), "entity/unknown");
    }

    #[test]
    fn the_emissive_twin_is_named_after_the_texture() {
        let (variants, glow) = index(names().into_iter());
        set_index(variants, glow);
        assert_eq!(
            glow_of("entity/zombie/zombie"),
            Some("entity/zombie/zombie_e".to_owned())
        );
        assert_eq!(glow_of("entity/cow/cow"), None);
    }

    #[test]
    fn the_switches_and_the_nested_call() {
        configure(false, true);
        assert!(!random_on() && emissive_on());
        configure(true, false);
        assert!(random_on() && !emissive_on());
        leave_body();
        assert!(enter_body());
        assert!(!enter_body());
        leave_body();
        assert!(enter_layers());
        assert!(
            !enter_layers(),
            "the layers hook is not entered while it calls the original"
        );
        leave_layers();
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
