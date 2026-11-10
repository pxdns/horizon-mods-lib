"""Crosshair and tooltip drawing ports. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, LIST_B, SET_B, CLK_B, common_native
from mods import opts

DRAW = []

# ------------------------------------------------------------------------------------------------ crosshair_indicator
# The real Crosshair Indicator (Modrinth zLAdIOH7, SwimmingDog1 and MGC8, CC0-1.0), read from its jar: one mixin,
# `@Inject(method = "extractCrosshair", at = @At("TAIL"))`, that blits a 15 x 15 sprite at the screen's centre through
# `RenderPipelines.CROSSHAIR` when `minecraft.crosshairPickEntity instanceof Player`. No settings, no keys, no commands.
DRAW.append(dict(
    slug="crosshair-indicator", id="crosshair_indicator", name="Crosshair Indicator", title="Crosshair Indicator",
    usage="Nothing to press and nothing to set: switch it on in Options > Mods, and corner brackets show round your crosshair whenever you aim at another player.",
    description="Shows corner brackets around your crosshair when you're aiming at a player.",
    upstream="Crosshair Indicator (crosshairindicator) by SwimmingDog1 and MGC8", upstream_licence="CC0-1.0",
    command=None, hooks=["hud", "list"], options=None, needs_api=400,
    hook_note=("It asks the game what the crosshair is on with `native_crosshair_target` (**`NATIVE_HOOKS_API` 400**, added on the client-modpack-port branch of the Horizon fork) "
               "and draws in the HUD hook point, through the crosshair's own inverting blend. It is not part of upstream Horizon's hook points yet, so it needs a Horizon that has API 400."),
    runtime_extra='''
/// The crosshair sprite is 15 by 15 pixels, centred on the screen.
pub const SPRITE: i32 = 15;
/// The 12 white pixels of the original's `crosshair.png`: four corner brackets round a 7 by 7 square at (4, 4).
pub const PIXELS: [(i32, i32); 12] = [
    (4, 4), (5, 4), (9, 4), (10, 4),
    (4, 5), (10, 5),
    (4, 9), (10, 9),
    (4, 10), (5, 10), (9, 10), (10, 10),
];

/// Whether the brackets show: the crosshair is on a player who is not in spectator mode. A mob, an armor stand or
/// anything else does not count, and neither does a block or nothing (the game then gives no target).
pub fn active(kind: &str, spectator: bool) -> bool {
    kind == "minecraft:player" && !spectator
}

/// The sprite's top-left corner for a screen of `width` by `height` GUI pixels: `Gui.extractCrosshair`'s `(w - 15) / 2`.
pub fn corner(width: i32, height: i32) -> (i32, i32) {
    ((width - SPRITE) / 2, (height - SPRITE) / 2)
}
''',
    tests='''
    #[test]
    fn a_player_shows_the_brackets() {
        assert!(active("minecraft:player", false));
    }

    #[test]
    fn a_player_in_spectator_mode_does_not() {
        assert!(!active("minecraft:player", true));
    }

    #[test]
    fn a_mob_or_another_entity_does_not() {
        for kind in ["minecraft:cow", "minecraft:zombie", "minecraft:armor_stand", "minecraft:item_frame", "minecraft:mannequin"] {
            assert!(!active(kind, false), "{kind}");
        }
    }

    #[test]
    fn no_target_and_a_block_give_nothing_to_ask() {
        // The game answers `None` for a block and for nothing, so the hook never reaches `active`; an empty kind is no player.
        assert!(!active("", false));
    }

    #[test]
    fn the_sprite_is_the_originals_twelve_pixels_inside_a_15_square() {
        assert_eq!(PIXELS.len(), 12);
        assert!(PIXELS.iter().all(|&(x, y)| (0..SPRITE).contains(&x) && (0..SPRITE).contains(&y)));
        // Four corner brackets: each is an L of three pixels round a 7 by 7 square from (4, 4) to (10, 10), none inside it.
        assert!(PIXELS.iter().all(|&(x, y)| (4..=10).contains(&x) && (4..=10).contains(&y)));
        assert!(PIXELS.iter().all(|&(x, y)| x == 4 || x == 10 || y == 4 || y == 10), "only the square's edge");
        let mut sorted = PIXELS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 12, "no pixel twice");
        // The middle of each side is empty: the brackets leave a gap.
        assert!(!PIXELS.contains(&(7, 4)) && !PIXELS.contains(&(4, 7)) && !PIXELS.contains(&(10, 7)) && !PIXELS.contains(&(7, 10)));
    }

    #[test]
    fn the_corner_matches_the_games_integer_division() {
        assert_eq!(corner(1920, 1080), (952, 532));
        assert_eq!(corner(427, 240), (206, 112));
        assert_eq!(corner(854, 480), (419, 232));
    }
''',
    helpers={
        "hud": '''// The brackets round the crosshair, drawn after it through its inverting blend.
fn crosshair_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_crosshair_indicator as m;
    let _ = &corners;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let Some(target) = game.native_crosshair_target() else {
        return;
    };
    if !m::active(&target.kind, target.spectator) {
        return;
    }
    let (x0, y0) = m::corner(batch.width as i32, batch.height as i32);
    let start = batch.vertices.len();
    for (x, y) in m::PIXELS {
        game.native_fill(batch, [(x0 + x) as f32, (y0 + y) as f32, 1.0, 1.0], 0xFFFF_FFFF);
    }
    batch.invert_since(start);
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ entity_crosshair
DRAW.append(dict(
    slug="entity-crosshair", id="entity_crosshair", name="Entity Crosshair", title="Entity Crosshair", raw=True,
    usage="Nothing to press: the settings page picks the crosshair's style (plus, gap plus, small plus, dot, ring) and colour, and whether it turns red over a living entity.",
    description="Replaces the crosshair with a style and colour of your choice, red over living entities.",
    upstream="Entity Crosshair (EntityCrosshair)", upstream_licence="see the upstream project",
    command=None, hooks=["frame", "crosshair", "list", "settings", "click"],
    options=None, custom_settings=True,
    modpath="horizon_client::hud_layout::lib_entity_crosshair",
    raw_note=("Its module lives in Horizon's `hud_layout.rs` (the render crate) and a RETURN modifier on `hud_layout::crosshair` swaps the crosshair sprite for the mod's own shape, keeping the attack indicator; "
              "the game tells it each frame what is under the crosshair. None of that is part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    toml_text=toml_text("entity_crosshair", "1.0.0", "crates/horizon-client-render/src/hud_layout.rs", "lib_entity_crosshair", [
        dict(fields=[("id", '"crosshair"'), ("kind", '"RETURN"'), ("target", '"horizon_client_render::hud_layout::crosshair"'), ("helper", '"crosshair.rs"')],
             bindings=[("draws", "Vec<GuiDraw>", "$value"), ("w", "i32", "w"), ("h", "i32", "h")]),
        native("frame", "native_frame", [G, ("elapsed", "std::time::Duration", "elapsed")]),
        native("list", "native_mods", LIST_B), native("settings", "native_settings", SET_B), native("click", "native_settings_click", CLK_B),
    ], visibility="pub"),
    runtime_extra='''
use std::cell::Cell;

/// The shapes the first button cycles through, and their names.
pub const STYLES: [&str; 5] = ["Plus", "Gap plus", "Small plus", "Dot", "Ring"];
/// The colours the second button cycles through: name and ARGB.
pub const COLOURS: [(&str, u32); 5] = [
    ("White", 0xFFFF_FFFF),
    ("Green", 0xFF55_FF55),
    ("Cyan", 0xFF55_FFFF),
    ("Yellow", 0xFFFF_FF55),
    ("Magenta", 0xFFFF_55FF),
];
/// What the crosshair turns over a living entity.
pub const RED: u32 = 0xFFFF_5555;
/// The crosshair sprite is 15 by 15 pixels; shapes are rectangles (x, y, width, height) inside it.
pub type Rect = (i32, i32, i32, i32);

/// The settings, one stored file per key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub style: usize,
    pub colour: usize,
    /// Turn red over a living, alive entity.
    pub red: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { style: 0, colour: 0, red: true }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let index = |key: &str, len: usize, default: usize| {
            get(key).and_then(|v| v.trim().parse::<usize>().ok()).filter(|&i| i < len).unwrap_or(default)
        };
        Self {
            style: index("style", STYLES.len(), d.style),
            colour: index("colour", COLOURS.len(), d.colour),
            red: get("red").map_or(d.red, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => d.red,
            }),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Style: {}", STYLES[self.style]),
            format!("Colour: {}", COLOURS[self.colour].0),
            format!("Red over mobs: {}", if self.red { "ON" } else { "OFF" }),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                self.style = (self.style + 1) % STYLES.len();
                Some(("style", self.style.to_string()))
            }
            1 => {
                self.colour = (self.colour + 1) % COLOURS.len();
                Some(("colour", self.colour.to_string()))
            }
            2 => {
                self.red = !self.red;
                Some(("red", self.red.to_string()))
            }
            _ => None,
        }
    }
}

/// The rectangles of a style, from the sprite's top-left corner; the crosshair's centre pixel is (7, 7).
pub fn shape(style: usize) -> Vec<Rect> {
    match style {
        0 => vec![(3, 7, 9, 1), (7, 3, 1, 9)],
        1 => vec![(2, 7, 4, 1), (9, 7, 4, 1), (7, 2, 1, 4), (7, 9, 1, 4)],
        2 => vec![(5, 7, 5, 1), (7, 5, 1, 5)],
        3 => vec![(6, 6, 3, 3)],
        _ => vec![(4, 3, 7, 1), (4, 11, 7, 1), (3, 4, 1, 7), (11, 4, 1, 7)],
    }
}

/// The colour to draw in.
pub fn colour(s: Settings, over_living: bool) -> u32 {
    if s.red && over_living { RED } else { COLOURS[s.colour].1 }
}

thread_local! {
    static STATE: Cell<Option<(Settings, bool)>> = const { Cell::new(None) };
}

/// The game tells the render code each frame: the settings (None when the mod is off) and whether a living entity is under the crosshair.
pub fn configure(settings: Option<Settings>, over_living: bool) {
    STATE.with(|s| s.set(settings.map(|v| (v, over_living))));
}

/// What to draw this frame: the shape and its colour; None when the mod is off.
pub fn current() -> Option<(Vec<Rect>, u32)> {
    let (s, living) = STATE.with(Cell::get)?;
    Some((shape(s.style), colour(s, living)))
}

/// Whether a living, alive entity is under the crosshair.
pub fn active_target(living: Option<bool>) -> bool {
    living == Some(true)
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = Settings::read(get);
    now.click(i)
}
''',
    tests='''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn every_shape_fits_in_the_sprite_and_stays_on_its_centre() {
        for style in 0..STYLES.len() {
            for (x, y, w, h) in shape(style) {
                assert!(x >= 0 && y >= 0 && x + w <= 15 && y + h <= 15, "style {style}: {x} {y} {w} {h}");
            }
        }
        // The plus, small plus and dot are symmetric around the centre pixel.
        for style in [0, 2, 3] {
            let rects = shape(style);
            let left = rects.iter().map(|r| r.0).min().unwrap();
            let right = rects.iter().map(|r| r.0 + r.2 - 1).max().unwrap();
            assert_eq!(left + right, 14, "style {style} is centred on pixel 7");
        }
    }

    #[test]
    fn red_over_a_living_entity_unless_switched_off() {
        let s = Settings::default();
        assert_eq!(colour(s, false), 0xFFFF_FFFF);
        assert_eq!(colour(s, true), RED);
        assert_eq!(colour(Settings { red: false, ..s }, true), 0xFFFF_FFFF);
        assert_eq!(colour(Settings { colour: 1, ..s }, false), 0xFF55_FF55);
    }

    #[test]
    fn the_buttons_cycle_and_report_what_to_store() {
        let mut s = Settings::default();
        assert_eq!(s.click(0), Some(("style", "1".to_owned())));
        for _ in 0..4 {
            s.click(0);
        }
        assert_eq!(s.style, 0, "style wraps");
        assert_eq!(s.click(2), Some(("red", "false".to_owned())));
        assert!(s.click(5).is_none());
        assert_eq!(labels(none)[0], "Style: Plus");
    }

    #[test]
    fn stored_junk_and_out_of_range_fall_back() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let far = |k: &str| (k == "style").then(|| "9".to_owned());
        assert_eq!(Settings::read(far).style, 0);
    }

    #[test]
    fn the_frame_state_decides_what_is_drawn() {
        configure(None, true);
        assert!(current().is_none(), "off: Horizon's own crosshair");
        configure(Some(Settings::default()), false);
        assert_eq!(current().unwrap().1, 0xFFFF_FFFF);
        configure(Some(Settings::default()), true);
        assert_eq!(current().unwrap().1, RED);
        configure(None, false);
        assert!(active_target(Some(true)) && !active_target(Some(false)) && !active_target(None));
    }
''',
    helpers={
        "crosshair": '''// hud_layout::crosshair: the sprite Horizon draws is replaced by the mod's shape (the attack indicator after it stays).
fn entity_crosshair_crosshair(draws: Vec<GuiDraw>, w: i32, h: i32) -> Vec<GuiDraw> {
    let Some((shape, color)) = lib_entity_crosshair::current() else {
        return draws;
    };
    let (x0, y0) = ((w - 15) / 2, (h - 15) / 2);
    let mut out: Vec<GuiDraw> = draws.into_iter().skip(1).collect();
    for (x, y, rw, rh) in shape {
        out.push(fill(x0 + x, y0 + y, x0 + x + rw, y0 + y + rh, color));
    }
    out
}
''',
        "frame": '''// Each frame: tell the render code the mod's settings and whether a living entity is under the crosshair.
fn entity_crosshair_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use horizon_client::hud_layout::lib_entity_crosshair as m;
    let _ = elapsed;
    if !game.native_on(m::ID) {
        m::configure(None, false);
        return;
    }
    let settings = m::Settings::read(|k| game.native_setting(m::ID, k));
    m::configure(Some(settings), m::active_target(game.crosshair_living()));
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ shulker_box_tooltip
DRAW.append(dict(
    slug="shulker-box-tooltip", id="shulker_box_tooltip", name="Shulker Box Tooltip", title="Shulker Box Tooltip", raw=True,
    usage="Nothing to press: hover a shulker box and its tooltip lists what is inside, totalled by item, with a 3 by 9 grid of the real item icons and counts under it.",
    description="Shows what is inside a shulker box in its tooltip: totals by item, and a graphical 9 by 3 grid of the items.",
    upstream="Shulker Box Tooltip (shulkerboxtooltip)", upstream_licence="see the upstream project",
    command=None, hooks=["tooltip", "bundle", "list", "settings", "click"],
    modpath="horizon_items::bundle::lib_shulker_box_tooltip",
    options=[("summary", "Totals by item", True), ("grid", "Slot grid with item icons", True), ("all", "Other containers too", False)],
    toml_text=toml_text("shulker_box_tooltip", "1.0.0", "crates/horizon-items/src/bundle.rs", "lib_shulker_box_tooltip", [
        dict(fields=[("id", '"tooltip"'), ("kind", '"RETURN"'), ("target", '"horizon_client::game::stations::Game<\'_>::stack_tooltip"'), ("helper", '"tooltip.rs"')],
             bindings=[("lines", "Vec<(horizon_client::text::Text, u32)>", "$value"), ("game", "&crate::game::Game<'_>", "&*self"), ("stack", "&ItemStack", "stack")]),
        dict(fields=[("id", '"bundle"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_items::bundle::tooltip_layout"'), ("helper", '"bundle.rs"')],
             bindings=[("items", "&Items", "items"), ("stack", "&ItemStack", "stack"), ("x", "f32", "x"), ("y", "f32", "y"), ("box_width", "f32", "box_width")]),
    ] + common_native(False), visibility="pub"),
    raw_note=("It adds a RETURN modifier to `Game::stack_tooltip` (the totals) and a cancellable HEAD hook on `bundle::tooltip_layout`, the function Horizon uses to lay out a bundle's item grid in its tooltip, "
              "which it answers for a shulker box with a 9 by 3 grid; Horizon then draws the real item icons and counts itself. It reads the item's container contents (`ItemStack::data`). None of this is part of the stable hook-point contract, "
              "so it is **pinned to one Horizon commit**."),
    runtime_extra='''
use std::collections::BTreeMap;

/// How many item lines the totals show before `and N more kinds`.
pub const MAX_LINES: usize = 8;
/// A shulker box holds 27 slots, three rows of nine.
pub const COLUMNS: usize = 9;
pub const SLOTS: usize = 27;

/// One filled slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub slot: u8,
    pub item: String,
    pub count: u32,
}

/// `minecraft:purple_shulker_box` as `Purple Shulker Box`.
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

/// Whether the item is a shulker box of any colour.
pub fn is_shulker(id: &str) -> bool {
    id.ends_with("shulker_box")
}

/// The contents totalled by item, most first: `64 x Cobblestone`, then `and N more kinds` past `MAX_LINES`, then the slots used.
pub fn summary(slots: &[Slot]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for s in slots {
        *totals.entry(s.item.as_str()).or_insert(0) += s.count;
    }
    let mut rows: Vec<(&str, u32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let mut out: Vec<String> = rows.iter().take(MAX_LINES).map(|(id, n)| format!("{n} x {}", title(id))).collect();
    if rows.len() > MAX_LINES {
        out.push(format!("and {} more kinds", rows.len() - MAX_LINES));
    }
    out.push(format!("{} of {SLOTS} slots used", slots.len()));
    out
}

thread_local! {
    /// Whether the tooltip being built should get the item grid (set while its lines are made).
    static GRID: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn set_grid(on: bool) {
    GRID.with(|g| g.set(on));
}

pub fn grid_on() -> bool {
    GRID.with(std::cell::Cell::get)
}

/// A slot's cell in the grid is this many pixels square, like a bundle's.
pub const CELL: f32 = 24.0;
/// The grid's size: nine cells across, three down.
pub const GRID_WIDTH: f32 = COLUMNS as f32 * CELL;
pub const GRID_HEIGHT: f32 = (SLOTS / COLUMNS) as f32 * CELL;
/// Spaces on a line of their own that make the tooltip wide enough for the grid.
pub const PAD: usize = 60;

/// The top-left of slot `n` for a tooltip whose box starts at `(x, y)` and is `box_width` wide, the grid centred in it.
pub fn slot_at(n: u8, x: f32, y: f32, box_width: f32) -> Option<[f32; 2]> {
    let n = usize::from(n);
    if n >= SLOTS {
        return None;
    }
    let left = x + ((box_width - GRID_WIDTH) / 2.0).floor().max(0.0);
    Some([left + (n % COLUMNS) as f32 * CELL, y + (n / COLUMNS) as f32 * CELL])
}
''',
    tests='''
    fn slot(slot: u8, item: &str, count: u32) -> Slot {
        Slot { slot, item: item.to_owned(), count }
    }

    #[test]
    fn ids_become_titles_and_shulkers_are_found() {
        assert_eq!(title("minecraft:purple_shulker_box"), "Purple Shulker Box");
        assert!(is_shulker("minecraft:shulker_box"));
        assert!(is_shulker("minecraft:red_shulker_box"));
        assert!(!is_shulker("minecraft:chest"));
    }

    #[test]
    fn the_summary_totals_by_item_most_first() {
        let slots = [slot(0, "minecraft:cobblestone", 64), slot(1, "minecraft:cobblestone", 20), slot(5, "minecraft:diamond", 3)];
        assert_eq!(summary(&slots), ["84 x Cobblestone", "3 x Diamond", "3 of 27 slots used"]);
        assert_eq!(summary(&[]), ["0 of 27 slots used"]);
    }

    #[test]
    fn many_kinds_are_cut_off_with_a_count() {
        let slots: Vec<Slot> = (0..12).map(|i| slot(i, &format!("minecraft:item_{i:02}"), 1)).collect();
        let lines = summary(&slots);
        assert_eq!(lines.len(), MAX_LINES + 2);
        assert_eq!(lines[MAX_LINES], "and 4 more kinds");
    }

    #[test]
    fn slots_sit_on_a_nine_by_three_grid_centred_in_the_box() {
        assert_eq!(slot_at(0, 10.0, 20.0, 216.0), Some([10.0, 20.0]));
        assert_eq!(slot_at(10, 10.0, 20.0, 216.0), Some([34.0, 44.0]));
        assert_eq!(slot_at(26, 10.0, 20.0, 216.0), Some([202.0, 68.0]));
        assert_eq!(slot_at(0, 0.0, 0.0, 240.0), Some([12.0, 0.0]), "a wider box centres it");
        assert_eq!(slot_at(0, 0.0, 0.0, 100.0), Some([0.0, 0.0]), "a narrow box never pushes it left");
        assert_eq!(slot_at(27, 0.0, 0.0, 216.0), None);
        assert_eq!((GRID_WIDTH, GRID_HEIGHT), (216.0, 72.0));
    }

    #[test]
    fn the_grid_flag_is_per_tooltip() {
        set_grid(true);
        assert!(grid_on());
        set_grid(false);
        assert!(!grid_on());
    }
''',
    helpers={
        "tooltip": '''// ItemStack.getTooltipLines, then what is inside a shulker box.
fn shulker_box_tooltip_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::@MOD@ as m;
    m::set_grid(false);
    if !game.native_on(m::ID) {
        return lines;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !(m::is(get, "all") || m::is_shulker(&stack.item)) {
        return lines;
    }
    let Some(data) = stack.data() else {
        return lines;
    };
    if data.container.is_empty() {
        return lines;
    }
    let slots: Vec<m::Slot> = data
        .container
        .iter()
        .map(|(slot, s)| m::Slot { slot: *slot, item: s.item.to_string(), count: s.count })
        .collect();
    let gray = rgba(170, 170, 170, 255);
    let mut add = |text: String| lines.push((horizon_client::text::Text::literal(text), gray));
    if m::is(get, "summary") {
        for line in m::summary(&slots) {
            add(line);
        }
    }
    if m::is(get, "grid") {
        // The tooltip is as wide as its widest line: a line of spaces makes room for the grid, which Horizon draws under the text.
        add(" ".repeat(m::PAD));
        m::set_grid(true);
    }
    lines
}
''',
        "bundle": '''// A bundle's item grid in a tooltip: answered for a shulker box with its 27 slots, nine across.
fn shulker_box_tooltip_bundle(
    items: &Items,
    stack: &ItemStack,
    x: f32,
    y: f32,
    box_width: f32,
) -> std::ops::ControlFlow<Option<TooltipLayout>> {
    use lib_shulker_box_tooltip as m;
    let _ = items;
    if !m::grid_on() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(data) = stack.data() else {
        return std::ops::ControlFlow::Continue(());
    };
    if data.container.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let slots = data
        .container
        .iter()
        .filter_map(|(slot, s)| m::slot_at(*slot, x, y, box_width).map(|at| (at, s.clone())))
        .collect();
    std::ops::ControlFlow::Break(Some(TooltipLayout {
        width: m::GRID_WIDTH,
        height: m::GRID_HEIGHT,
        slots,
        selected: None,
        overflow: None,
        // The progress bar a bundle has is drawn far off screen.
        bar: [-1000.0, -1000.0],
        fill: 0.0,
        full: false,
        label: None,
        empty_description: None,
    }))
}
''',
    },
))
