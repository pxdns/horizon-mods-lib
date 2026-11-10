"""The ten native mods: id, text, hooks, runtime.rs and the helper bodies that differ per mod."""

BOX_HEAD = r'''/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;
'''

# A HUD box helper: guard, data (@DATA@), then the box in @CORNER@.
HUD_BOX = '''// The box in @CORNERDOC@; the debug screen takes the top corners.
fn @ID@_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::@MOD@ as m;
    use horizon_client::extras::Corner;
@DATA@
}
'''

DRAW_LINES = '''    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(@CORNER@, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], @INK@, true);
    }'''

COMMAND_TOGGLE = '''// `/@CMD@` hides or shows the box.
fn @ID@_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::@MOD@ as m;
    let name = command.split(' ').next().unwrap_or("");
    if !game.native_on(m::ID) || name != m::COMMAND {
        return;
    }
    *ran = true;
    let stored = {
        let game = &*game;
        m::click(|k| game.native_setting(m::ID, k), 0)
    };
    if let Some((key, value)) = stored {
        game.native_set_setting(m::ID, key, &value);
        game.native_chat(&format!(
            "{} {}",
            m::NAME,
            if value == "true" { "shown" } else { "hidden" }
        ));
    }
}
'''


def opts(rows):
    return ",\n".join(f'    ("{k}", "{label}", {str(d).lower()})' for k, label, d in rows)


def hud_guard(corner, extra=""):
    return f'''    if !game.native_on(m::ID) || !game.native_corner_shown({corner}) {{
        return;
    }}
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {{
        return;
    }}
{extra}'''


MODS = []

# --------------------------------------------------------------------------------------------- fps_display
MODS.append(dict(
    slug="fps-display", id="fps_display", name="FPS Display", title="FPS Display",
    description="Frames per second in a corner of the screen, coloured by how smooth they are.",
    upstream="FPS - Display (fpsdisplay)", upstream_licence="see the upstream project",
    command="fps", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("slowest", "Slowest frame", False), ("low", "1% low", False), ("colour", "Colour by rate", True)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "fps";

/// The box's text colour: green from 60 fps, yellow from 30, red below; white when colouring is off.
pub fn ink(fps: u32, coloured: bool) -> u32 {
    match (coloured, fps) {
        (false, _) => INK,
        (true, 60..) => 0x55_FF55,
        (true, 30..=59) => 0xFF_FF55,
        (true, _) => 0xFF_5555,
    }
}

/// The box's lines: the rate, then the slowest frame and the 1% low when asked for.
pub fn lines(fps: u32, slowest: u32, low: u32, show_slowest: bool, show_low: bool) -> Vec<String> {
    let mut out = vec![format!("{fps} FPS")];
    if show_slowest {
        out.push(format!("Slowest {slowest}"));
    }
    if show_low {
        out.push(format!("1% low {low}"));
    }
    out
}
''',
    tests=r'''
    #[test]
    fn colours_follow_the_rate() {
        assert_eq!(ink(144, true), 0x55_FF55);
        assert_eq!(ink(60, true), 0x55_FF55);
        assert_eq!(ink(59, true), 0xFF_FF55);
        assert_eq!(ink(30, true), 0xFF_FF55);
        assert_eq!(ink(12, true), 0xFF_5555);
        assert_eq!(ink(12, false), INK);
    }

    #[test]
    fn lines_grow_with_the_options() {
        assert_eq!(lines(90, 70, 60, false, false), ["90 FPS"]);
        assert_eq!(lines(90, 70, 60, true, true), ["90 FPS", "Slowest 70", "1% low 60"]);
        assert_eq!(lines(90, 70, 60, false, true), ["90 FPS", "1% low 60"]);
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the top-right corner") .replace("@DATA@", hud_guard("Corner::TopRight") + '''    let (fps, slowest, low) = game.native_frame_rate();
    let get = |k: &str| game.native_setting(m::ID, k);
    let lines = m::lines(fps, slowest, low, m::is(get, "slowest"), m::is(get, "low"));
    let ink = m::ink(fps, m::is(get, "colour"));
''' + DRAW_LINES.replace("@CORNER@", "Corner::TopRight").replace("@INK@", "ink")),
))

# --------------------------------------------------------------------------------------------- sprint_indicator
MODS.append(dict(
    slug="sprint-indicator", id="sprint_indicator", name="Sprint Indicator", title="Sprint Indicator",
    description="Shows Sprinting, Walking or Sneaking in a corner of the screen while you move.",
    upstream="Sprint Indicator (sprintindicator)", upstream_licence="see the upstream project",
    command="sprint", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("walking", "Show walking", True), ("idle", "Show standing still", False)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "sprint";

/// What the player is doing, from the held movement keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Keys {
    pub forward: bool,
    pub left: bool,
    pub back: bool,
    pub right: bool,
    pub sprint: bool,
    pub sneak: bool,
}

impl Keys {
    pub fn moving(&self) -> bool {
        self.forward || self.left || self.back || self.right
    }
}

/// The label to show, or nothing.
pub fn status(keys: Keys, walking: bool, idle: bool) -> Option<&'static str> {
    if keys.sneak {
        Some("Sneaking")
    } else if keys.sprint && keys.moving() {
        Some("Sprinting")
    } else if keys.moving() {
        walking.then_some("Walking")
    } else {
        idle.then_some("Standing")
    }
}
''',
    tests=r'''
    #[test]
    fn sneaking_wins_then_sprinting_then_walking() {
        let k = |forward, sprint, sneak| Keys { forward, sprint, sneak, ..Keys::default() };
        assert_eq!(status(k(true, true, true), true, true), Some("Sneaking"));
        assert_eq!(status(k(true, true, false), true, true), Some("Sprinting"));
        assert_eq!(status(k(true, false, false), true, true), Some("Walking"));
        assert_eq!(status(k(true, false, false), false, true), None);
    }

    #[test]
    fn sprint_without_moving_is_standing() {
        let keys = Keys { sprint: true, ..Keys::default() };
        assert_eq!(status(keys, true, false), None);
        assert_eq!(status(keys, true, true), Some("Standing"));
        assert!(!keys.moving());
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-left corner").replace("@DATA@", '''    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    let mut held = |name: &str| game.native_mapping(name).is_some_and(|(down, _)| down);
    let keys = m::Keys {
        forward: held("key.forward"),
        left: held("key.left"),
        back: held("key.back"),
        right: held("key.right"),
        sprint: held("key.sprint"),
        sneak: held("key.sneak"),
    };
    let game = &*game;
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") {
        return;
    }
    let Some(text) = m::status(keys, m::is(get, "walking"), m::is(get, "idle")) else {
        return;
    };
    let lines = [text.to_owned()];
''' + DRAW_LINES.replace("@CORNER@", "Corner::BottomLeft").replace("@INK@", "m::INK")),
))

# --------------------------------------------------------------------------------------------- key_overlay
MODS.append(dict(
    slug="key-overlay", id="key_overlay", name="Key Overlay", title="Key Overlay",
    description="Shows W, A, S, D, jump and the mouse buttons on screen and lights the ones you hold.",
    upstream="Key Overlay (KeyOverlay)", upstream_licence="see the upstream project",
    command="keys", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("mouse", "Mouse buttons", True), ("space", "Jump bar", True)],
    runtime_extra=r'''
/// Slash command that hides or shows the overlay.
pub const COMMAND: &str = "keys";
/// A held key's box and text; an idle key's.
pub const HELD_BACKGROUND: u32 = 0xB0FF_FFFF;
pub const HELD_INK: u32 = 0x00_0000;
pub const IDLE_BACKGROUND: u32 = 0x7800_0000;
pub const IDLE_INK: u32 = 0xFF_FFFF;

/// One key's box, in GUI pixels from the overlay's top-left corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    /// The vanilla mapping it follows (`key.forward`…).
    pub mapping: &'static str,
    /// The text when the mapping's own label is not wanted.
    pub fixed: Option<&'static str>,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

const KEY: f32 = 16.0;
const GAP: f32 = 2.0;

/// The boxes to draw: W over A S D, the jump bar, then the two mouse buttons.
pub fn cells(mouse: bool, space: bool) -> Vec<Cell> {
    let step = KEY + GAP;
    let mut out = vec![
        Cell { mapping: "key.forward", fixed: None, x: step, y: 0.0, w: KEY, h: KEY },
        Cell { mapping: "key.left", fixed: None, x: 0.0, y: step, w: KEY, h: KEY },
        Cell { mapping: "key.back", fixed: None, x: step, y: step, w: KEY, h: KEY },
        Cell { mapping: "key.right", fixed: None, x: 2.0 * step, y: step, w: KEY, h: KEY },
    ];
    let mut y = 2.0 * step;
    if space {
        out.push(Cell { mapping: "key.jump", fixed: Some("SPACE"), x: 0.0, y, w: 3.0 * KEY + 2.0 * GAP, h: 12.0 });
        y += 12.0 + GAP;
    }
    if mouse {
        let w = (3.0 * KEY + 2.0 * GAP - GAP) / 2.0;
        out.push(Cell { mapping: "key.attack", fixed: Some("LMB"), x: 0.0, y, w, h: 14.0 });
        out.push(Cell { mapping: "key.use", fixed: Some("RMB"), x: w + GAP, y, w, h: 14.0 });
    }
    out
}

/// The overlay's width and height.
pub fn bounds(cells: &[Cell]) -> (f32, f32) {
    cells.iter().fold((0.0, 0.0), |(w, h), c| (f32::max(w, c.x + c.w), f32::max(h, c.y + c.h)))
}

/// A key's label as drawn: at most four characters, upper case.
pub fn short(label: &str) -> String {
    label.chars().take(4).collect::<String>().to_uppercase()
}
''',
    tests=r'''
    #[test]
    fn boxes_do_not_overlap_and_fit_the_bounds() {
        for (mouse, space) in [(true, true), (true, false), (false, true), (false, false)] {
            let cells = cells(mouse, space);
            let (w, h) = bounds(&cells);
            for (i, a) in cells.iter().enumerate() {
                assert!(a.x + a.w <= w && a.y + a.h <= h);
                for b in &cells[i + 1..] {
                    let apart = a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
                    assert!(apart, "{} overlaps {}", a.mapping, b.mapping);
                }
            }
        }
    }

    #[test]
    fn options_choose_the_rows() {
        assert_eq!(cells(false, false).len(), 4);
        assert_eq!(cells(true, true).len(), 7);
        assert_eq!(bounds(&cells(false, false)), (52.0, 34.0));
    }

    #[test]
    fn labels_are_short_and_upper_case() {
        assert_eq!(short("w"), "W");
        assert_eq!(short("Space"), "SPAC");
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-right corner").replace("@DATA@", '''    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    let (shown, mouse, space) = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "shown"), m::is(get, "mouse"), m::is(get, "space"))
    };
    if !shown {
        return;
    }
    let cells = m::cells(mouse, space);
    let mut state = Vec::new();
    for cell in &cells {
        let (down, label) = game.native_mapping(cell.mapping).unwrap_or((false, String::new()));
        let text = cell.fixed.map_or_else(|| m::short(&label), str::to_owned);
        state.push((down, text));
    }
    let game = &*game;
    let (w, h) = m::bounds(&cells);
    let (left, top) = corners.place(Corner::BottomRight, w, h);
    for (cell, (down, text)) in cells.iter().zip(state) {
        let (background, ink) = if down {
            (m::HELD_BACKGROUND, m::HELD_INK)
        } else {
            (m::IDLE_BACKGROUND, m::IDLE_INK)
        };
        let (x, y) = (left + cell.x, top + cell.y);
        game.native_fill(batch, [x, y, cell.w, cell.h], background);
        let tx = x + (cell.w - game.native_text_width(&text)) / 2.0;
        game.native_text(batch, &text, [tx, y + (cell.h - 8.0) / 2.0], ink, !down);
    }'''),
))

# --------------------------------------------------------------------------------------------- cps_counter
MODS.append(dict(
    slug="cps-counter", id="cps_counter", name="CPS Counter", title="CPS Counter",
    description="Your clicks per second in a corner of the screen.",
    upstream="Tempest CPS module (own work)", upstream_licence="MIT",
    command="cps", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("right", "Right clicks", False)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "cps";

/// The box's lines: attack clicks per second, and use clicks when asked for.
pub fn lines(attack: u32, used: u32, right: bool) -> Vec<String> {
    let mut out = vec![format!("{attack} CPS")];
    if right {
        out.push(format!("{used} RMB"));
    }
    out
}
''',
    tests=r'''
    #[test]
    fn right_clicks_are_a_second_line() {
        assert_eq!(lines(8, 3, false), ["8 CPS"]);
        assert_eq!(lines(8, 3, true), ["8 CPS", "3 RMB"]);
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-left corner").replace("@DATA@", hud_guard("Corner::BottomLeft", "") .replace("    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {\n        return;\n    }\n    if !m::is(|k| game.native_setting(m::ID, k), \"shown\") {\n        return;\n    }\n", '''    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    let (shown, right) = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "shown"), m::is(get, "right"))
    };
    if !shown {
        return;
    }
    let (attack, used) = game.native_clicks();
    let game = &*game;
    let lines = m::lines(attack, used, right);
''') + DRAW_LINES.replace("@CORNER@", "Corner::BottomLeft").replace("@INK@", "m::INK")),
))

# --------------------------------------------------------------------------------------------- info_hud
MODS.append(dict(
    slug="info-hud", id="info_hud", name="Info HUD", title="Info HUD",
    description="Your speed, the way you face and your dimension in a corner of the screen, in the spirit of MiniHUD's info lines.",
    upstream="MiniHUD (info lines)", upstream_licence="see the upstream project",
    command="info", hooks=["hud", "tick", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("speed", "Speed", True), ("facing", "Facing", True), ("dimension", "Dimension", False), ("angles", "Yaw and pitch", False)],
    runtime_extra=BOX_HEAD + r'''
use std::cell::Cell as StateCell;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "info";
/// The game ticks 20 times a second.
const TICKS_PER_SECOND: f64 = 20.0;
/// A jump this far in one tick is a teleport, not speed.
const TELEPORT: f64 = 20.0;

thread_local! {
    static LAST: StateCell<Option<(f64, f64)>> = const { StateCell::new(None) };
    static SPEED: StateCell<f64> = const { StateCell::new(0.0) };
}

/// One game tick at `(x, z)`: the horizontal speed in blocks per second, smoothed over about a quarter second.
pub fn tick(x: f64, z: f64) {
    let moved = LAST.with(|last| {
        let moved = last.get().map(|(px, pz)| (x - px).hypot(z - pz));
        last.set(Some((x, z)));
        moved
    });
    let now = match moved {
        Some(d) if d < TELEPORT => d * TICKS_PER_SECOND,
        _ => 0.0,
    };
    SPEED.with(|s| s.set(s.get() * 0.75 + now * 0.25));
}

/// Forgets the last position (no world).
pub fn reset() {
    LAST.with(|l| l.set(None));
    SPEED.with(|s| s.set(0.0));
}

/// The smoothed speed, blocks per second.
pub fn speed() -> f64 {
    SPEED.with(StateCell::get)
}

/// Direction.fromYRot: yaw 0 faces south, 90 west.
pub fn facing(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South (+Z)", "West (-X)", "North (-Z)", "East (+X)"][i as usize]
}

/// A yaw as vanilla shows it: -180 to 180.
pub fn wrap(yaw: f32) -> f32 {
    let y = yaw.rem_euclid(360.0);
    if y > 180.0 { y - 360.0 } else { y }
}

/// The box's lines for the options that are on.
pub fn lines(speed: f64, yaw: f32, pitch: f32, dimension: &str, show: [bool; 4]) -> Vec<String> {
    let mut out = Vec::new();
    if show[0] {
        out.push(format!("Speed {speed:.1} b/s"));
    }
    if show[1] {
        out.push(format!("Facing {}", facing(yaw)));
    }
    if show[2] {
        out.push(dimension.trim_start_matches("minecraft:").replace('_', " "));
    }
    if show[3] {
        out.push(format!("Yaw {:.1} Pitch {:.1}", wrap(yaw), pitch));
    }
    out
}
''',
    tests=r'''
    #[test]
    fn speed_settles_on_the_walking_pace() {
        reset();
        // Walking: 0.2158 blocks a tick = 4.3 b/s.
        for i in 0..40 {
            tick(f64::from(i) * 0.2158, 0.0);
        }
        assert!((speed() - 4.316).abs() < 0.05, "{}", speed());
        // Standing still decays to nothing.
        for _ in 0..60 {
            tick(40.0 * 0.2158, 0.0);
        }
        assert!(speed() < 0.01);
        reset();
        assert_eq!(speed(), 0.0);
    }

    #[test]
    fn a_teleport_is_not_speed() {
        reset();
        tick(0.0, 0.0);
        tick(500.0, 500.0);
        assert_eq!(speed(), 0.0);
    }

    #[test]
    fn facing_and_wrap_follow_vanilla() {
        assert_eq!(facing(0.0), "South (+Z)");
        assert_eq!(facing(90.0), "West (-X)");
        assert_eq!(facing(-180.0), "North (-Z)");
        assert_eq!(facing(-90.0), "East (+X)");
        assert_eq!(wrap(270.0), -90.0);
        assert_eq!(wrap(-190.0), 170.0);
    }

    #[test]
    fn lines_follow_the_toggles() {
        assert_eq!(
            lines(4.32, 180.0, 10.0, "minecraft:the_nether", [true, true, true, true]),
            ["Speed 4.3 b/s", "Facing North (-Z)", "the nether", "Yaw 180.0 Pitch 10.0"]
        );
        assert!(lines(0.0, 0.0, 0.0, "x", [false; 4]).is_empty());
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the top-left corner").replace("@DATA@", hud_guard("Corner::TopLeft", '''    let Some(p) = game.native_player() else {
        return;
    };
    let get = |k: &str| game.native_setting(m::ID, k);
    let show = [m::is(get, "speed"), m::is(get, "facing"), m::is(get, "dimension"), m::is(get, "angles")];
    let lines = m::lines(m::speed(), p.yaw, p.pitch, p.dimension, show);
    if lines.is_empty() {
        return;
    }
''') + DRAW_LINES.replace("@CORNER@", "Corner::TopLeft").replace("@INK@", "m::INK")),
    extra_helpers={"tick": '''// Once a tick: the player's speed, from how far they moved.
fn info_hud_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_info_hud as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    match game.native_player() {
        Some(p) => m::tick(p.x, p.z),
        None => m::reset(),
    }
}
'''},
))

# --------------------------------------------------------------------------------------------- custom_hud
MODS.append(dict(
    slug="custom-hud", id="custom_hud", name="Custom HUD", title="Custom HUD",
    description="One box with the lines you pick: FPS, coordinates, facing and clicks per second, on the side you choose.",
    upstream="CustomHUD (profile idea)", upstream_licence="MIT",
    command="hud", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("right", "Right side", False), ("fps", "FPS", True), ("coords", "Coordinates", True), ("facing", "Facing", True), ("cps", "Clicks per second", False)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "hud";

/// What the box can show.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Reading {
    pub fps: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub clicks: u32,
}

/// Direction.fromYRot: yaw 0 faces south, 90 west.
pub fn facing(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South", "West", "North", "East"][i as usize]
}

/// The box's lines: `show` is fps, coordinates, facing, clicks.
pub fn lines(r: Reading, show: [bool; 4]) -> Vec<String> {
    let mut out = Vec::new();
    if show[0] {
        out.push(format!("FPS {}", r.fps));
    }
    if show[1] {
        out.push(format!("XYZ {:.1} / {:.1} / {:.1}", r.x, r.y, r.z));
    }
    if show[2] {
        out.push(format!("Facing {}", facing(r.yaw)));
    }
    if show[3] {
        out.push(format!("CPS {}", r.clicks));
    }
    out
}
''',
    tests=r'''
    #[test]
    fn only_the_picked_lines_show() {
        let r = Reading { fps: 120, x: 1.0, y: 64.0, z: -3.5, yaw: 90.0, clicks: 7 };
        assert_eq!(lines(r, [true, false, false, false]), ["FPS 120"]);
        assert_eq!(
            lines(r, [true, true, true, true]),
            ["FPS 120", "XYZ 1.0 / 64.0 / -3.5", "Facing West", "CPS 7"]
        );
        assert!(lines(r, [false; 4]).is_empty());
    }

    #[test]
    fn facing_is_the_compass_point() {
        assert_eq!(facing(0.0), "South");
        assert_eq!(facing(-90.0), "East");
        assert_eq!(facing(180.0), "North");
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the corner the player picked").replace("@DATA@", '''    if !game.native_on(m::ID) {
        return;
    }
    let (shown, right, show) = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        (
            m::is(get, "shown"),
            m::is(get, "right"),
            [m::is(get, "fps"), m::is(get, "coords"), m::is(get, "facing"), m::is(get, "cps")],
        )
    };
    let corner = if right { Corner::TopRight } else { Corner::TopLeft };
    if !shown || !game.native_corner_shown(corner) {
        return;
    }
    let clicks = if show[3] { game.native_clicks().0 } else { 0 };
    let game = &*game;
    let (fps, _, _) = game.native_frame_rate();
    let mut reading = m::Reading { fps, clicks, ..m::Reading::default() };
    if let Some(p) = game.native_player() {
        reading.x = p.x;
        reading.y = p.y;
        reading.z = p.z;
        reading.yaw = p.yaw;
    } else {
        // Coordinates and facing mean nothing outside a world.
    }
    let lines = m::lines(reading, show);
    if lines.is_empty() {
        return;
    }
''' + DRAW_LINES.replace("@CORNER@", "corner").replace("@INK@", "m::INK")),
))

# --------------------------------------------------------------------------------------------- zoom pair
ZOOM_RUNTIME = r'''
use std::cell::RefCell;

/// The key's name in Controls, and its default.
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "@DEFAULT_KEY@";
/// Whether the key is held to zoom (otherwise one press turns it on and the next turns it off).
pub const HOLD: bool = @HOLD@;
/// The zooms the starting zoom cycles through, and the limits of the wheel.
pub const LEVELS: [u8; 7] = [2, 3, 4, 6, 8, 12, 16];
const MIN: f32 = 1.25;
const MAX: f32 = 16.0;
/// Each wheel step multiplies the zoom by this.
const STEP: f32 = 1.25;

/// The settings, one stored file per key.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The zoom the key starts at, one of `LEVELS`.
    pub level: u8,
    /// Ease in and out like the spyglass.
    pub smooth: bool,
    /// Turn slower the more you zoom in.
    pub sensitivity: bool,
    /// The wheel changes the zoom while it is on.
    pub scroll: bool,
}

impl Settings {
    /// From the stored values (`get` reads one key's file).
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let level = get("level")
            .and_then(|v| v.trim().parse::<u8>().ok())
            .filter(|v| LEVELS.contains(v))
            .unwrap_or(@LEVEL@);
        let flag = |key: &str, default: bool| {
            get(key).map_or(default, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => default,
            })
        };
        Self {
            level,
            smooth: flag("smooth", true),
            sensitivity: flag("sensitivity", true),
            scroll: flag("scroll", true),
        }
    }

    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Starting zoom: {}x", self.level),
            format!("Smooth zoom: {}", on(self.smooth)),
            format!("Slower turning: {}", on(self.sensitivity)),
            format!("Wheel zoom: {}", on(self.scroll)),
        ]
    }

    /// A click on button `i` of `labels`; the key and value that changed.
    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                let at = LEVELS.iter().position(|&l| l == self.level).unwrap_or(0);
                self.level = LEVELS[(at + 1) % LEVELS.len()];
                Some(("level", self.level.to_string()))
            }
            1 => {
                self.smooth = !self.smooth;
                Some(("smooth", self.smooth.to_string()))
            }
            2 => {
                self.sensitivity = !self.sensitivity;
                Some(("sensitivity", self.sensitivity.to_string()))
            }
            3 => {
                self.scroll = !self.scroll;
                Some(("scroll", self.scroll.to_string()))
            }
            _ => None,
        }
    }
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    /// The zoom while it is on; None when it is off.
    active: Option<f32>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    settings(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = settings(get);
    let stored = now.click(i);
    STATE.with_borrow_mut(|s| s.settings = Some(now));
    stored
}

/// The zoom key went down or up (`playing`: nothing is over the world).
pub fn key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    let level = f32::from(settings(get).level).clamp(MIN, MAX);
    STATE.with_borrow_mut(|s| {
        if HOLD {
            if pressed && !playing {
                return;
            }
            s.active = pressed.then_some(level);
        } else if pressed {
            s.active = match s.active {
                Some(_) => None,
                None if playing => Some(level),
                None => None,
            };
        }
    });
}

/// Wheel steps (up is positive): true when the zoom is on and they change it.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|s| {
        let scroll = s.settings.is_none_or(|v| v.scroll);
        match &mut s.active {
            Some(z) if scroll => {
                *z = (*z * STEP.powf(steps)).clamp(MIN, MAX);
                true
            }
            _ => false,
        }
    })
}

/// The zoom to ask the game for: factor, smooth, and how much to scale mouse turning.
pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        let z = s.active?;
        let v = s.settings.unwrap_or(Settings::read(|_| None));
        Some((z, v.smooth, if v.sensitivity { 1.0 / z } else { 1.0 }))
    })
}

/// Turns the zoom off and forgets the settings (tests only).
#[cfg(test)]
pub fn forget() {
    STATE.with_borrow_mut(|s| *s = State::default());
}
'''

ZOOM_TESTS = r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_key_zooms_by_its_mode() {
        forget();
        key(true, true, none);
        assert_eq!(view().map(|v| v.0), Some(f32::from(@LEVEL@u8)));
        key(true, false, none);
        if HOLD {
            assert_eq!(view(), None, "released");
        } else {
            assert!(view().is_some(), "a toggle stays on");
            key(true, true, none);
            assert_eq!(view(), None, "the second press turns it off");
        }
        forget();
    }

    #[test]
    fn a_screen_over_the_world_does_not_start_the_zoom() {
        forget();
        key(false, true, none);
        assert_eq!(view(), None);
        forget();
    }

    #[test]
    fn the_wheel_zooms_only_while_on_and_within_limits() {
        forget();
        assert!(!wheel(1.0), "off: the hotbar keeps it");
        key(true, true, none);
        let start = view().unwrap().0;
        assert!(wheel(2.0));
        assert!(view().unwrap().0 > start);
        assert!(wheel(-100.0));
        assert_eq!(view().unwrap().0, MIN);
        assert!(wheel(100.0));
        assert_eq!(view().unwrap().0, MAX);
        forget();
    }

    #[test]
    fn the_wheel_can_be_turned_off() {
        forget();
        let stored = |k: &str| (k == "scroll").then(|| "false".to_owned());
        key(true, true, stored);
        assert!(!wheel(1.0));
        forget();
    }

    #[test]
    fn settings_read_default_and_click_reports_the_change() {
        forget();
        let s = settings(none);
        assert_eq!(s.level, @LEVEL@);
        assert!(s.smooth && s.sensitivity && s.scroll);
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("smooth", "false"));
        assert_eq!(labels(none)[1], "Smooth zoom: OFF");
        assert!(click(none, 9).is_none());
        let (_, level) = click(none, 0).unwrap();
        assert!(LEVELS.iter().any(|l| l.to_string() == level));
        forget();
    }

    #[test]
    fn turning_slows_with_the_zoom() {
        forget();
        key(true, true, none);
        let (factor, _, turn) = view().unwrap();
        assert!((turn - 1.0 / factor).abs() < 1e-6);
        forget();
    }

    #[test]
    fn stored_junk_falls_back_to_the_defaults() {
        let junk = |_: &str| Some("banana".to_owned());
        let s = Settings::read(junk);
        assert_eq!(s.level, @LEVEL@);
        assert!(s.smooth);
    }
'''

ZOOM_HELPERS = {
    "key": '''// The zoom key went down or up: the player's own choice of key.
fn @ID@_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::@MOD@ as m;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(m::ID) {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    m::key(playing, pressed, |name| game.native_setting(m::ID, name));
}
''',
    "wheel": '''// While the zoom is on, the wheel changes it instead of moving the hotbar.
fn @ID@_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::@MOD@ as m;
    if game.native_on(m::ID) && m::wheel(steps) {
        *claimed = true;
    }
}
''',
    "zoom": '''// The zoom in effect, unless another mod's came first.
fn @ID@_view(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::@MOD@ as m;
    if zoom.is_none()
        && game.native_on(m::ID)
        && let Some((factor, smooth, turn)) = m::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth, turn });
    }
}
''',
}

for slug, id_, name, desc, upstream, hold, key, level in [
    ("zoomify", "zoomify", "Zoomify", "Hold a key to zoom in, turn the wheel to zoom further, with smooth easing and slower turning.", "Zoomify (zoomify)", True, "key.keyboard.c", 4),
]:
    MODS.append(dict(
        slug=slug, id=id_, name=name, title=name, description=desc,
        upstream=upstream, upstream_licence="see the upstream project",
        command=None, hooks=["key", "wheel", "zoom", "list", "settings", "click"],
        options=None, zoom=True,
        runtime_extra=ZOOM_RUNTIME.replace("@DEFAULT_KEY@", key).replace("@HOLD@", str(hold).lower()).replace("@LEVEL@", str(level)),
        tests=ZOOM_TESTS.replace("@LEVEL@", str(level)),
        keys_expr='vec![[m::KEY.into(), "Zoom".into(), m::DEFAULT_KEY.into()]]',
    ))

# --------------------------------------------------------------------------------------------- waypoints
MODS.append(dict(
    slug="waypoints", id="waypoints", name="Waypoints", title="Waypoints",
    description="Name places with /wp, list them, and track one: its distance and direction show on screen.",
    upstream="Xaero's Minimap and World Map (waypoints idea)", upstream_licence="independent implementation, no upstream code",
    command="wp", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Show tracked waypoint", True)],
    runtime_extra=BOX_HEAD + r'''
/// The command: `/wp add|del|list|track|untrack`.
pub const COMMAND: &str = "wp";
/// The settings file that holds every waypoint.
pub const FILE: &str = "waypoints";

/// A named place.
#[derive(Clone, Debug, PartialEq)]
pub struct Waypoint {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub dimension: String,
}

/// Where the player is: block position and dimension id.
#[derive(Clone, Debug, PartialEq)]
pub struct Here {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dimension: String,
}

/// One line a waypoint is stored as: tab-separated.
pub fn encode(list: &[Waypoint]) -> String {
    list.iter()
        .map(|w| format!("{}\t{}\t{}\t{}\t{}", w.name, w.x, w.y, w.z, w.dimension))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The waypoints of a stored file; lines that do not read are skipped.
pub fn decode(text: &str) -> Vec<Waypoint> {
    text.lines()
        .filter_map(|line| {
            let mut f = line.split('\t');
            Some(Waypoint {
                name: f.next().filter(|n| name_ok(n))?.to_owned(),
                x: f.next()?.parse().ok()?,
                y: f.next()?.parse().ok()?,
                z: f.next()?.parse().ok()?,
                dimension: f.next()?.to_owned(),
            })
        })
        .collect()
}

/// A name is 1 to 24 letters, digits, `_` or `-`.
pub fn name_ok(name: &str) -> bool {
    (1..=24).contains(&name.len())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Blocks from `here` to the waypoint.
pub fn distance(here: &Here, w: &Waypoint) -> f64 {
    let (dx, dy, dz) = (f64::from(w.x) + 0.5 - here.x, f64::from(w.y) - here.y, f64::from(w.z) + 0.5 - here.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// The yaw that faces the waypoint (0 faces south, 90 west), degrees.
pub fn bearing(here: &Here, w: &Waypoint) -> f32 {
    let dx = f64::from(w.x) + 0.5 - here.x;
    let dz = f64::from(w.z) + 0.5 - here.z;
    (-dx).atan2(dz).to_degrees() as f32
}

/// Where the waypoint is relative to the way the player faces.
pub fn relative(yaw: f32, bearing: f32) -> &'static str {
    let mut d = (bearing - yaw).rem_euclid(360.0);
    if d > 180.0 {
        d -= 360.0;
    }
    let names = ["ahead", "ahead right", "right", "behind right", "behind", "behind left", "left", "ahead left"];
    names[((d / 45.0).round() as i32).rem_euclid(8) as usize]
}

/// `home 128 m ahead right`.
pub fn line(here: &Here, yaw: f32, w: &Waypoint) -> String {
    format!("{} {:.0} m {}", w.name, distance(here, w), relative(yaw, bearing(here, w)))
}

#[derive(Default)]
struct State {
    loaded: bool,
    list: Vec<Waypoint>,
    tracked: Option<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// Reads the stored waypoints once.
pub fn load(get: impl FnOnce() -> Option<String>) {
    STATE.with_borrow_mut(|s| {
        if !s.loaded {
            s.loaded = true;
            s.list = decode(&get().unwrap_or_default());
        }
    });
}

/// What a command did: lines for chat, and the file to store when the list changed.
#[derive(Debug, Default, PartialEq)]
pub struct Outcome {
    pub chat: Vec<String>,
    pub save: Option<String>,
}

/// Runs the arguments after `/wp` (`add home`, `track home`, `list`…).
pub fn run(args: &str, here: Option<&Here>, yaw: f32) -> Outcome {
    let mut words = args.split_whitespace();
    let verb = words.next().unwrap_or("");
    let name = words.next().unwrap_or("");
    let say = |text: String| Outcome { chat: vec![text], save: None };
    STATE.with_borrow_mut(|s| match verb {
        "add" | "set" => {
            let Some(here) = here else {
                return say("Not in a world".into());
            };
            if !name_ok(name) {
                return say("Usage: /wp add <name> (1-24 letters, digits, - or _)".into());
            }
            let place = Waypoint {
                name: name.to_owned(),
                x: here.x.floor() as i32,
                y: here.y.floor() as i32,
                z: here.z.floor() as i32,
                dimension: here.dimension.clone(),
            };
            let said = format!("Waypoint {name} set at {} {} {}", place.x, place.y, place.z);
            s.list.retain(|w| w.name != name);
            s.list.push(place);
            Outcome { chat: vec![said], save: Some(encode(&s.list)) }
        }
        "del" | "remove" => {
            let before = s.list.len();
            s.list.retain(|w| w.name != name);
            if s.list.len() == before {
                return say(format!("No waypoint {name}"));
            }
            if s.tracked.as_deref() == Some(name) {
                s.tracked = None;
            }
            Outcome { chat: vec![format!("Waypoint {name} deleted")], save: Some(encode(&s.list)) }
        }
        "list" | "" => {
            if s.list.is_empty() {
                return say("No waypoints. /wp add <name> sets one here.".into());
            }
            let chat = s
                .list
                .iter()
                .map(|w| {
                    let away = here
                        .filter(|h| h.dimension == w.dimension)
                        .map_or(String::new(), |h| format!(", {} m", distance(h, w).round()));
                    format!("{}: {} {} {} ({}{})", w.name, w.x, w.y, w.z, w.dimension.trim_start_matches("minecraft:"), away)
                })
                .collect();
            Outcome { chat, save: None }
        }
        "track" => {
            if !s.list.iter().any(|w| w.name == name) {
                return say(format!("No waypoint {name}"));
            }
            s.tracked = Some(name.to_owned());
            say(match here.and_then(|h| s.list.iter().find(|w| w.name == name).map(|w| line(h, yaw, w))) {
                Some(l) => format!("Tracking {l}"),
                None => format!("Tracking {name}"),
            })
        }
        "untrack" => {
            s.tracked = None;
            say("Not tracking a waypoint".into())
        }
        _ => say("/wp add|del|list|track|untrack <name>".into()),
    })
}

/// The tracked waypoint's line, when the player is in its dimension.
pub fn tracked_line(here: &Here, yaw: f32) -> Option<String> {
    STATE.with_borrow(|s| {
        let name = s.tracked.as_ref()?;
        let w = s.list.iter().find(|w| &w.name == name)?;
        (w.dimension == here.dimension).then(|| line(here, yaw, w))
    })
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget() {
    STATE.with_borrow_mut(|s| *s = State::default());
}
''',
    tests=r'''
    fn here(x: f64, y: f64, z: f64) -> Here {
        Here { x, y, z, dimension: "minecraft:overworld".into() }
    }

    #[test]
    fn waypoints_round_trip_through_the_file() {
        let list = vec![
            Waypoint { name: "home".into(), x: 1, y: 64, z: -3, dimension: "minecraft:overworld".into() },
            Waypoint { name: "end-base".into(), x: -10, y: 70, z: 22, dimension: "minecraft:the_end".into() },
        ];
        assert_eq!(decode(&encode(&list)), list);
        assert_eq!(decode("bad line\nhome\t1\t2"), vec![]);
        assert_eq!(decode("a b\t1\t2\t3\tx"), vec![], "spaces are not allowed in names");
    }

    #[test]
    fn names_are_checked() {
        assert!(name_ok("home_2"));
        assert!(!name_ok(""));
        assert!(!name_ok("has space"));
        assert!(!name_ok(&"x".repeat(25)));
    }

    #[test]
    fn bearing_faces_the_target_in_vanilla_yaw() {
        let p = here(0.5, 64.0, 0.5);
        let at = |x, z| Waypoint { name: "w".into(), x, y: 64, z, dimension: "minecraft:overworld".into() };
        assert!((bearing(&p, &at(0, 10)) - 0.0).abs() < 0.01, "south is +Z");
        assert!((bearing(&p, &at(-10, 0)) - 90.0).abs() < 0.01, "west is -X");
        assert!((bearing(&p, &at(10, 0)) + 90.0).abs() < 0.01, "east is +X");
        assert!((bearing(&p, &at(0, -10)).abs() - 180.0).abs() < 0.01, "north is -Z");
        assert_eq!(distance(&p, &at(3, 4)).round(), 5.0);
    }

    #[test]
    fn direction_is_relative_to_where_you_face() {
        assert_eq!(relative(0.0, 0.0), "ahead");
        assert_eq!(relative(0.0, 90.0), "right", "facing south, west is on the right hand");
        assert_eq!(relative(0.0, -90.0), "left");
        assert_eq!(relative(0.0, 180.0), "behind");
        assert_eq!(relative(160.0, -160.0), "ahead right");
        assert_eq!(relative(-160.0, 160.0), "ahead left");
    }

    #[test]
    fn commands_add_list_track_and_delete() {
        forget();
        let p = here(10.7, 64.0, -3.2);
        let added = run("add home", Some(&p), 0.0);
        assert_eq!(added.chat, ["Waypoint home set at 10 64 -4"]);
        assert!(added.save.as_deref().is_some_and(|s| s.starts_with("home\t10\t64\t-4")));
        assert_eq!(run("add bad!", Some(&p), 0.0).save, None);
        assert_eq!(run("add home", None, 0.0).chat, ["Not in a world"]);
        let listed = run("list", Some(&p), 0.0);
        assert!(listed.chat[0].starts_with("home: 10 64 -4 (overworld, "));
        assert_eq!(tracked_line(&p, 0.0), None);
        assert!(run("track home", Some(&p), 0.0).chat[0].starts_with("Tracking home"));
        assert!(tracked_line(&here(0.0, 64.0, 0.0), 0.0).unwrap().starts_with("home "));
        let nether = Here { dimension: "minecraft:the_nether".into(), ..p.clone() };
        assert_eq!(tracked_line(&nether, 0.0), None, "another dimension");
        assert_eq!(run("del home", Some(&p), 0.0).save.as_deref(), Some(""));
        assert_eq!(tracked_line(&p, 0.0), None, "deleted waypoints are not tracked");
        assert_eq!(run("del home", Some(&p), 0.0).chat, ["No waypoint home"]);
        forget();
    }

    #[test]
    fn the_file_is_read_once() {
        forget();
        load(|| Some("home\t1\t2\t3\tminecraft:overworld".into()));
        load(|| Some("other\t9\t9\t9\tminecraft:overworld".into()));
        assert_eq!(run("list", None, 0.0).chat, ["home: 1 2 3 (overworld)"]);
        forget();
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-right corner").replace("@DATA@", hud_guard("Corner::BottomRight", '''    let Some(p) = game.native_player() else {
        return;
    };
    m::load(|| game.native_setting(m::ID, m::FILE));
    let here = m::Here { x: p.x, y: p.y, z: p.z, dimension: p.dimension.to_owned() };
    let Some(text) = m::tracked_line(&here, p.yaw) else {
        return;
    };
    let lines = [text];
''') + DRAW_LINES.replace("@CORNER@", "Corner::BottomRight").replace("@INK@", "m::INK")),
    command_body='''// /wp: name places, list them, track one.
fn waypoints_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_waypoints as m;
    let mut parts = command.splitn(2, ' ');
    let name = parts.next().unwrap_or("");
    if !game.native_on(m::ID) || name != m::COMMAND {
        return;
    }
    *ran = true;
    let args = parts.next().unwrap_or("");
    {
        let game = &*game;
        m::load(|| game.native_setting(m::ID, m::FILE));
    }
    let player = game.native_player();
    let here = player.as_ref().map(|p| m::Here { x: p.x, y: p.y, z: p.z, dimension: p.dimension.to_owned() });
    let yaw = player.as_ref().map_or(0.0, |p| p.yaw);
    let outcome = m::run(args, here.as_ref(), yaw);
    if let Some(text) = &outcome.save {
        game.native_set_setting(m::ID, m::FILE, text);
    }
    for line in &outcome.chat {
        game.native_chat(line);
    }
}
''',
))
