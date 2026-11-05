"""Entity Texture Features: random texture variants and emissive layers for mobs, from the textures a resource pack adds. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

ETF = []

ETF.append(dict(
    slug="entity-texture-features", id="entity_texture_features", name="Entity Texture Features", title="Entity Texture Features", raw=True,
    usage="Nothing to press, but it needs a resource pack that adds textures: a mob whose texture is `entity/zombie/zombie` picks, by its entity id, one of `zombie`, `zombie2`, `zombie3`... that the pack provides (the same one every time), and a texture `zombie_e` is drawn over the body at full brightness (glowing eyes). Without such textures it changes nothing.",
    description="Random texture variants and emissive layers for mobs, from the textures a resource pack provides.",
    upstream="Entity Texture Features (etf)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command=None, hooks=["body", "layers", "tick", "list", "settings", "click"],
    options=[("random", "Random texture variants", True), ("emissive", "Emissive layers", True)],
    modpath="horizon_client::mobs::render::lib_entity_texture_features",
    raw_note=("It lives in `crates/horizon-client/src/mobs/render.rs`: cancellable HEAD hooks on `body` (which texture a mob's body uses) and `layers` (its extra layers) that call the original and then change its answer. "
              "They are not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. **I could not look at the result**: it is checked to compile and its logic is tested, but a mob with variants was not seen on screen. "
              "It covers numbered variants (`name2`, `name3`...) and `_e` emissive textures that sit in the resource pack's `textures/entity`; the original's `.properties` rule files (by biome, name, health, time), `optifine/random` folders, player skins and block entity textures are not included."),
    toml_text=toml_text("entity_texture_features", "1.0.0", "crates/horizon-client/src/mobs/render.rs", "lib_entity_texture_features", [
        dict(fields=[("id", '"body"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_lib::mobs::render::body"'), ("helper", '"body.rs"')],
             bindings=[("m", "&Mob", "m")]),
        dict(fields=[("id", '"layers"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_lib::mobs::render::layers"'), ("helper", '"layers.rs"')],
             bindings=[("m", "&Mob", "m"), ("age", "f32", "age")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False), visibility="pub"),
    runtime_extra=r'''
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
pub fn index<'a>(names: impl Iterator<Item = &'a str>) -> (BTreeMap<String, Vec<String>>, BTreeSet<String>) {
    let all: BTreeSet<&str> = names.filter(|n| n.starts_with("entity/")).collect();
    let mut variants: BTreeMap<String, Vec<(u32, String)>> = BTreeMap::new();
    for name in &all {
        if let Some((base, n)) = split_number(name)
            && all.contains(base)
        {
            variants.entry(base.to_owned()).or_default().push((n, (*name).to_owned()));
        }
    }
    let variants = variants
        .into_iter()
        .map(|(base, mut list)| {
            list.sort();
            (base, list.into_iter().map(|(_, name)| name).collect())
        })
        .collect();
    let glow = all.iter().filter_map(|n| n.strip_suffix("_e")).filter(|base| all.contains(base)).map(str::to_owned).collect();
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
            if at == 0 { base.to_owned() } else { list[at - 1].clone() }
        }
        _ => base.to_owned(),
    })
}

/// The emissive twin of a texture, if the pack has one.
pub fn glow_of(texture: &str) -> Option<String> {
    GLOW.with_borrow(|g| g.contains(texture).then(|| format!("{texture}_e")))
}
''',
    tests=r'''
    fn names() -> Vec<&'static str> {
        vec![
            "entity/zombie/zombie", "entity/zombie/zombie2", "entity/zombie/zombie3", "entity/zombie/zombie_e",
            "entity/cow/cow", "entity/cow/cow2", "entity/sign/oak", "entity/sign/oak1", "entity/creeper/creeper", "gui/sprites/x2", "entity/chest/normal_e",
        ]
    }

    #[test]
    fn numbered_names_split_at_two_and_up() {
        assert_eq!(split_number("entity/zombie/zombie2"), Some(("entity/zombie/zombie", 2)));
        assert_eq!(split_number("entity/zombie/zombie12"), Some(("entity/zombie/zombie", 12)));
        assert_eq!(split_number("entity/sign/oak1"), None, "1 is the base itself");
        assert_eq!(split_number("entity/zombie/zombie"), None);
        assert_eq!(split_number("entity/zombie/2"), None, "a bare number is not a variant");
        assert_eq!(split_number("entity/x/y1234"), None);
    }

    #[test]
    fn the_index_keeps_only_entity_textures_whose_base_exists() {
        let (variants, glow) = index(names().into_iter());
        assert_eq!(variants.get("entity/zombie/zombie").unwrap(), &["entity/zombie/zombie2", "entity/zombie/zombie3"]);
        assert_eq!(variants.get("entity/cow/cow").unwrap(), &["entity/cow/cow2"]);
        assert!(!variants.contains_key("entity/sign/oak"), "oak1 is not a variant");
        assert!(!variants.contains_key("gui/sprites/x"));
        assert_eq!(glow.iter().map(String::as_str).collect::<Vec<_>>(), ["entity/zombie/zombie"], "an emissive twin needs its base");
    }

    #[test]
    fn a_mob_keeps_its_variant_and_all_variants_get_used() {
        let (variants, glow) = index(names().into_iter());
        set_index(variants, glow);
        for id in [1, 17, 4000] {
            assert_eq!(pick("entity/zombie/zombie", id), pick("entity/zombie/zombie", id), "stable");
        }
        let picked: std::collections::BTreeSet<String> = (0..200).map(|id| pick("entity/zombie/zombie", id)).collect();
        assert_eq!(picked.len(), 3, "the base and both variants");
        assert!(picked.contains("entity/zombie/zombie"));
        assert_eq!(pick("entity/creeper/creeper", 5), "entity/creeper/creeper", "no variants, no change");
        assert_eq!(pick("entity/unknown", 5), "entity/unknown");
    }

    #[test]
    fn the_emissive_twin_is_named_after_the_texture() {
        let (variants, glow) = index(names().into_iter());
        set_index(variants, glow);
        assert_eq!(glow_of("entity/zombie/zombie"), Some("entity/zombie/zombie_e".to_owned()));
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
        assert!(!enter_layers(), "the layers hook is not entered while it calls the original");
        leave_layers();
    }
''',
    helpers={
        "body": '''// A mob's body model and texture: the texture may be one of the variants a resource pack adds.
fn entity_texture_features_body(m: &Mob) -> std::ops::ControlFlow<(Cow<'static, str>, Cow<'_, str>)> {
    use lib_entity_texture_features as r;
    if !r::random_on() || !r::enter_body() {
        return std::ops::ControlFlow::Continue(());
    }
    let (model, texture) = body(m);
    r::leave_body();
    let texture = r::pick(&texture, m.id);
    std::ops::ControlFlow::Break((model, Cow::Owned(texture)))
}
''',
        "layers": '''// A mob's extra layers, and its emissive twin of the body texture drawn at full brightness.
fn entity_texture_features_layers(m: &Mob, age: f32) -> std::ops::ControlFlow<Vec<Layer>> {
    use lib_entity_texture_features as r;
    if !r::emissive_on() || !r::enter_layers() {
        return std::ops::ControlFlow::Continue(());
    }
    let (model, texture) = body(m);
    let Some(glow) = r::glow_of(&texture) else {
        r::leave_layers();
        return std::ops::ControlFlow::Continue(());
    };
    // The original again: the guard lets this call through the hook.
    let mut layers = layers(m, age);
    r::leave_layers();
    layers.push(Layer { model, texture: Cow::Owned(glow), color: 0xFFFF_FFFF, full_bright: true, scroll: 0.0 });
    std::ops::ControlFlow::Break(layers)
}
''',
        "tick": '''// Once a tick: the switches, and about every five seconds the index of the textures the resource pack provides.
fn entity_texture_features_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::mobs::render::lib_entity_texture_features as r;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(r::ID);
    let (random, emissive) = {
        let get = |k: &str| game.native_setting(r::ID, k);
        (on && r::is(get, "random"), on && r::is(get, "emissive"))
    };
    r::configure(random, emissive);
    if on && game.ticks % 100 == 1 {
        let (variants, glow) = r::index(game.gui.atlas.sprites.keys().map(String::as_str));
        r::set_index(variants, glow);
    }
}
''',
    },
))
