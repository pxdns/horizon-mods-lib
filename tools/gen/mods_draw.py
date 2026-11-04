"""Crosshair and tooltip drawing ports. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, LIST_B, SET_B, CLK_B, common_native
from mods import opts

DRAW = []

# ------------------------------------------------------------------------------------------------ crosshair_indicator
DRAW.append(dict(
    slug="crosshair-indicator", id="crosshair_indicator", name="Crosshair Indicator", title="Crosshair Indicator", raw=True,
    usage="Nothing to press: when your crosshair is over a living, alive entity, a red mark is drawn over it. The settings page can swap the cross for a dot.",
    description="Turns your crosshair red when it is over a living creature.",
    upstream="Crosshair Indicator (crosshairindicator)", upstream_licence="CC0-1.0",
    command="indicator", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("dot", "Dot instead of cross", False)],
    raw_note=("It reads which entity is under the crosshair from `Game::crosshair_living` and the camera type from the `Game`, and draws over Horizon's crosshair from the HUD hook point. "
              "Those fields are not part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    runtime_extra='''
/// Slash command that hides or shows the indicator.
pub const COMMAND: &str = "indicator";
/// A red mark.
pub const RED: u32 = 0xFFFF_3030;
/// The crosshair sprite is 15 by 15 pixels, centred on the screen.
pub const SPRITE: i32 = 15;

/// A rectangle (x, y, width, height) in pixels, from the crosshair sprite's top-left corner.
pub type Rect = (i32, i32, i32, i32);

/// Whether the indicator shows: over a living, alive entity (`crosshairPickEntity`).
pub fn active(living: Option<bool>) -> bool {
    living == Some(true)
}

/// The mark's rectangles: a cross through the crosshair's centre, or a dot on it.
pub fn rects(dot: bool) -> Vec<Rect> {
    if dot {
        vec![(6, 6, 3, 3)]
    } else {
        vec![(3, 7, 9, 1), (7, 3, 1, 9)]
    }
}

/// The sprite's top-left corner for a screen of `width` by `height` GUI pixels: `Hud.extractCrosshair`'s `(w - 15) / 2`.
pub fn corner(width: i32, height: i32) -> (i32, i32) {
    ((width - SPRITE) / 2, (height - SPRITE) / 2)
}
''',
    tests='''
    #[test]
    fn only_a_living_alive_target_counts() {
        assert!(active(Some(true)));
        assert!(!active(Some(false)), "a dead entity");
        assert!(!active(None), "nothing there");
    }

    #[test]
    fn the_marks_sit_inside_the_sprite_on_its_centre() {
        for dot in [false, true] {
            for (x, y, w, h) in rects(dot) {
                assert!(x >= 0 && y >= 0 && x + w <= SPRITE && y + h <= SPRITE, "{x} {y} {w} {h}");
            }
        }
        // The cross's bars cross on the centre pixel (7, 7).
        let cross = rects(false);
        assert!(cross.iter().all(|&(x, y, w, h)| x <= 7 && 7 < x + w && y <= 7 && 7 < y + h));
    }

    #[test]
    fn the_corner_matches_the_games_integer_division() {
        assert_eq!(corner(1920, 1080), (952, 532));
        assert_eq!(corner(427, 240), (206, 112));
    }
''',
    helpers={
        "hud": '''// A red mark over the crosshair while it is over a living entity.
fn crosshair_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_crosshair_indicator as m;
    let _ = &corners;
    if !game.native_on(m::ID) || !game.native_playing() || !game.camera_type.first_person() {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") || !m::active(game.crosshair_living()) {
        return;
    }
    let (x0, y0) = m::corner(batch.width as i32, batch.height as i32);
    for (x, y, w, h) in m::rects(m::is(get, "dot")) {
        game.native_fill(batch, [(x0 + x) as f32, (y0 + y) as f32, w as f32, h as f32], m::RED);
    }
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
    usage="Nothing to press: hover a shulker box and its tooltip lists what is inside, totalled by item, and optionally as a 3 by 9 grid of two-letter slot codes.",
    description="Shows what is inside a shulker box in its tooltip: totals by item, and a slot grid.",
    upstream="Shulker Box Tooltip (shulkerboxtooltip)", upstream_licence="see the upstream project",
    command=None, hooks=["tooltip", "list", "settings", "click"],
    options=[("summary", "Totals by item", True), ("grid", "Slot grid", False), ("all", "Other containers too", False)],
    toml_text=toml_text("shulker_box_tooltip", "1.0.0", "crates/horizon-client/src/game.rs", "lib_shulker_box_tooltip", [
        dict(fields=[("id", '"tooltip"'), ("kind", '"RETURN"'), ("target", '"horizon_client::game::stations::Game<\'_>::stack_tooltip"'), ("helper", '"tooltip.rs"')],
             bindings=[("lines", "Vec<(horizon_client::text::Text, u32)>", "$value"), ("game", "&crate::game::Game<'_>", "&*self"), ("stack", "&ItemStack", "stack")]),
    ] + common_native(False)),
    raw_note=("It adds a RETURN modifier to `Game::stack_tooltip` and reads the item's container contents (`ItemStack::data`), neither of which is part of the stable hook-point contract, "
              "so it is **pinned to one Horizon commit**. It shows text, not the graphical 9 by 3 grid of the original: drawing inside a tooltip needs a draw hook Horizon does not have."),
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

/// A slot's two-letter code: the first letters of the item's first two words, or `..` for an empty slot.
fn code(slot: Option<&Slot>) -> String {
    match slot {
        None => "..".to_owned(),
        Some(s) => {
            let t = title(&s.item);
            let mut words = t.split(' ');
            let first = words.next().and_then(|w| w.chars().next()).unwrap_or('?');
            let second = words.next().and_then(|w| w.chars().next()).or_else(|| t.chars().nth(1)).unwrap_or(' ');
            format!("{first}{}", second.to_ascii_lowercase())
        }
    }
}

/// The three rows of the box as slot codes.
pub fn grid(slots: &[Slot]) -> Vec<String> {
    (0..SLOTS / COLUMNS)
        .map(|row| {
            (0..COLUMNS)
                .map(|col| {
                    let index = (row * COLUMNS + col) as u8;
                    code(slots.iter().find(|s| s.slot == index))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
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
    fn the_grid_has_three_rows_of_nine_codes() {
        let slots = [slot(0, "minecraft:cobblestone", 1), slot(10, "minecraft:diamond_pickaxe", 1), slot(26, "minecraft:stone", 1)];
        let rows = grid(&slots);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], "Co .. .. .. .. .. .. .. ..");
        assert_eq!(rows[1], ".. Dp .. .. .. .. .. .. ..");
        assert_eq!(rows[2], ".. .. .. .. .. .. .. .. St");
        assert!(rows.iter().all(|r| r.split(' ').count() == COLUMNS));
    }
''',
    helpers={
        "tooltip": '''// ItemStack.getTooltipLines, then what is inside a shulker box.
fn shulker_box_tooltip_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::lib_shulker_box_tooltip as m;
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
        for line in m::grid(&slots) {
            add(line);
        }
    }
    lines
}
''',
    },
))
