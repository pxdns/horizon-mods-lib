"""Enchantment Descriptions: a line saying what each enchantment on an item does, under its tooltip. Pinned to one Horizon commit."""
from mods_chat import toml_text, common_native

ENCH = []

DESCRIPTIONS = [
    ("protection", "Reduces most damage."), ("fire_protection", "Reduces fire and lava damage and burning time."),
    ("feather_falling", "Reduces fall damage."), ("blast_protection", "Reduces explosion damage and knockback."),
    ("projectile_protection", "Reduces damage from arrows and other projectiles."), ("respiration", "Extends breathing time underwater."),
    ("aqua_affinity", "Mines underwater as fast as on land."), ("thorns", "Hurts attackers who hit you."),
    ("depth_strider", "Moves faster underwater."), ("frost_walker", "Freezes water under you into ice."),
    ("binding_curse", "Cannot be taken off."), ("soul_speed", "Moves faster on soul sand and soul soil."),
    ("swift_sneak", "Sneaks faster."), ("sharpness", "Increases melee damage."), ("smite", "Increases damage to undead."),
    ("bane_of_arthropods", "Increases damage to spiders and silverfish, and slows them."), ("knockback", "Knocks targets further back."),
    ("fire_aspect", "Sets targets on fire."), ("looting", "Mobs drop more loot."), ("sweeping_edge", "Increases sweeping attack damage."),
    ("efficiency", "Mines faster."), ("silk_touch", "Mined blocks drop themselves."), ("unbreaking", "Chance not to lose durability."),
    ("fortune", "Increases drops from blocks."), ("power", "Increases arrow damage."), ("punch", "Knocks targets back further with arrows."),
    ("flame", "Arrows set targets on fire."), ("infinity", "Shooting does not use arrows."), ("luck_of_the_sea", "Better fishing loot."),
    ("lure", "Fish bite faster."), ("loyalty", "A thrown trident returns."), ("impaling", "More damage to aquatic mobs."),
    ("riptide", "Launches you with a thrown trident in water or rain."), ("channeling", "Calls lightning on a thrown trident hit in a storm."),
    ("multishot", "Shoots three projectiles at once."), ("quick_charge", "Reloads a crossbow faster."),
    ("piercing", "Projectiles pass through targets."), ("density", "More smash damage per block fallen."),
    ("breach", "Ignores part of the target's armor."), ("wind_burst", "A smash attack launches you upward."),
    ("mending", "Repairs the item with experience."), ("vanishing_curse", "Disappears when you die."),
    ("lunge", "Launches you forward with a spear attack."),
]

ENCH.append(dict(
    slug="enchantment-descriptions", id="enchantment_descriptions", name="Enchantment Descriptions", title="Enchantment Descriptions", raw=True,
    usage="Nothing to press: hover an enchanted item or book and its tooltip gains a gray line for each enchantment saying what it does, such as `Sharpness: Increases melee damage.`",
    description="Adds a line to enchanted items' tooltips saying what each enchantment does.",
    upstream="Enchantment Descriptions (enchdesc)", upstream_licence="see the upstream project; the descriptions are written for this mod",
    command=None, hooks=["tooltip", "list", "settings", "click"],
    options=[("books", "On enchanted books too", True), ("items", "On enchanted items", True)],
    toml_text=toml_text("enchantment_descriptions", "1.0.0", "crates/horizon-client/src/game.rs", "lib_enchantment_descriptions", [
        dict(fields=[("id", '"tooltip"'), ("kind", '"RETURN"'), ("target", '"horizon_client::game::stations::Game<\'_>::stack_tooltip"'), ("helper", '"tooltip.rs"')],
             bindings=[("lines", "Vec<(horizon_client::text::Text, u32)>", "$value"), ("game", "&crate::game::Game<'_>", "&*self"), ("stack", "&ItemStack", "stack")]),
    ] + common_native(False)),
    raw_note=("It adds a RETURN modifier to `Game::stack_tooltip` and reads the item's enchantments (`ItemStack::data`), neither of which is part of the stable hook-point contract, "
              "so it is **pinned to one Horizon commit**. The lines are added under the tooltip, named, rather than between the enchantment lines, so they never land under the wrong enchantment. "
              "Enchantments from data packs have no description and get none."),
    runtime_extra='''
/// What each vanilla enchantment does, by id without the namespace.
pub const DESCRIPTIONS: &[(&str, &str)] = &[
''' + "\n".join(f'    ("{i}", "{d}"),' for i, d in DESCRIPTIONS) + r'''
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
            c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The description of a vanilla enchantment, or none.
pub fn description(id: &str) -> Option<&'static str> {
    let bare = id.strip_prefix("minecraft:").unwrap_or(id);
    DESCRIPTIONS.iter().find(|(k, _)| *k == bare).map(|(_, d)| *d)
}

/// The lines to add for the enchantments of one stack: `Sharpness: Increases melee damage.`
pub fn lines(enchantments: &[String]) -> Vec<String> {
    enchantments.iter().filter_map(|id| description(id).map(|d| format!("{}: {d}", title(id)))).collect()
}
''',
    tests=r'''
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
        assert_eq!(description("minecraft:mending"), Some("Repairs the item with experience."));
        assert_eq!(description("mending"), description("minecraft:mending"));
        assert_eq!(description("mymod:thing"), None);
    }

    #[test]
    fn lines_skip_unknown_enchantments() {
        let have = vec!["minecraft:sharpness".to_owned(), "mymod:thing".to_owned(), "minecraft:unbreaking".to_owned()];
        assert_eq!(lines(&have), ["Sharpness: Increases melee damage.", "Unbreaking: Chance not to lose durability."]);
        assert!(lines(&[]).is_empty());
    }
''',
    helpers={
        "tooltip": '''// ItemStack.getTooltipLines, then what each enchantment on the item does.
fn enchantment_descriptions_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::lib_enchantment_descriptions as m;
    if !game.native_on(m::ID) {
        return lines;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let Some(data) = stack.data() else {
        return lines;
    };
    let mut found: Vec<String> = Vec::new();
    if m::is(get, "items") {
        found.extend(data.enchantments.iter().map(|(id, _)| id.to_string()));
    }
    if m::is(get, "books") {
        found.extend(data.stored_enchantments.iter().map(|(id, _)| id.to_string()));
    }
    let gray = rgba(170, 170, 170, 255);
    for line in m::lines(&found) {
        lines.push((horizon_client::text::Text::literal(line), gray));
    }
    lines
}
''',
    },
))
