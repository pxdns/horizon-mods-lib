"""Totem Tweaks (the pop animation) and Tweakeroo (a subset: hand restock, hotbar cycle, fast right click, tool break warning). Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

TW = []

# ------------------------------------------------------------------------------------------------ totem tweaks
TW.append(dict(
    slug="totem-tweaks", id="totem_tweaks", name="Totem Tweaks", title="Totem Tweaks", raw=True,
    usage="Nothing to press: the settings page (Options > Mods > Totem Tweaks) switches the totem pop animation off, picks its speed (0.5 to 3 times), its size (50 to 200 %), and switches the spinning, the rocking and the sideways swing off.",
    description="Customise the totem of undying pop animation: off, speed, size, spin, rocking and position.",
    upstream="Totem Tweaks (totem-tweaks)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command=None, hooks=["pose", "tick", "list", "settings", "click"],
    options=None, custom_settings=True,
    modpath="horizon_client::item_activation::lib_totem_tweaks",
    raw_note=("It replaces `item_activation::animation_pose`, the function that poses the totem for each frame of the pop, with a cancellable HEAD hook that computes the same curve with the player's choices. "
              "It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. Left out from the original: the handheld totem's size and the equip animation, which need hooks in the hand drawing. "
              "I could not look at the animation on screen; with every setting at its default the pose is the same curve as Horizon's, and the logic is tested."),
    toml_text=toml_text("totem_tweaks", "1.0.0", "crates/horizon-client/src/item_activation.rs", "lib_totem_tweaks", [
        dict(fields=[("id", '"pose"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_lib::item_activation::animation_pose"'), ("helper", '"pose.rs"')],
             bindings=[("ticks", "i32", "ticks"), ("partial", "f32", "partial"), ("off_x", "f32", "off_x"), ("off_y", "f32", "off_y"), ("aspect", "f32", "aspect")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False), visibility="pub"),
    runtime_extra=r'''
use std::cell::Cell;

/// The pop lasts 40 ticks.
pub const LENGTH: i32 = 40;
/// Speeds the second button cycles through, as a multiple of vanilla's.
pub const SPEEDS: [f32; 5] = [0.5, 1.0, 1.5, 2.0, 3.0];
/// Sizes the third button cycles through, in percent.
pub const SIZES: [u32; 6] = [50, 75, 100, 125, 150, 200];

/// The player's choices.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub pop: bool,
    pub speed: f32,
    pub size: u32,
    pub spin: bool,
    pub rock: bool,
    pub swing: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { pop: true, speed: 1.0, size: 100, spin: true, rock: true, swing: true }
    }
}

thread_local! {
    static CURRENT: Cell<Settings> = const { Cell::new(Settings { pop: true, speed: 1.0, size: 100, spin: true, rock: true, swing: true }) };
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// Sets what the animation uses and whether the mod is on (the game tells the renderer each tick).
pub fn configure(active: bool, settings: Settings) {
    ACTIVE.with(|a| a.set(active));
    CURRENT.with(|c| c.set(settings));
}

pub fn active() -> bool {
    ACTIVE.with(Cell::get)
}

pub fn current() -> Settings {
    CURRENT.with(Cell::get)
}

fn flag(get: &impl Fn(&str) -> Option<String>, key: &str) -> bool {
    get(key).as_deref().map(str::trim) != Some("false")
}

/// The stored settings; anything that does not read is the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> Settings {
    let speed = get("speed").and_then(|v| v.trim().parse::<f32>().ok()).filter(|v| SPEEDS.contains(v)).unwrap_or(1.0);
    let size = get("size").and_then(|v| v.trim().parse().ok()).filter(|v| SIZES.contains(v)).unwrap_or(100);
    Settings { pop: flag(&get, "pop"), speed, size, spin: flag(&get, "spin"), rock: flag(&get, "rock"), swing: flag(&get, "swing") }
}

fn on_off(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let s = read(get);
    vec![
        format!("Pop animation: {}", on_off(s.pop)),
        format!("Speed: {}x", s.speed),
        format!("Size: {} %", s.size),
        format!("Spin: {}", on_off(s.spin)),
        format!("Rocking: {}", on_off(s.rock)),
        format!("Sideways swing: {}", on_off(s.swing)),
    ]
}

fn next<T: PartialEq + Copy>(list: &[T], current: T) -> T {
    list[(list.iter().position(|v| *v == current).unwrap_or(0) + 1) % list.len()]
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let s = read(get);
    Some(match i {
        0 => ("pop", (!s.pop).to_string()),
        1 => ("speed", next(&SPEEDS, s.speed).to_string()),
        2 => ("size", next(&SIZES, s.size).to_string()),
        3 => ("spin", (!s.spin).to_string()),
        4 => ("rock", (!s.rock).to_string()),
        5 => ("swing", (!s.swing).to_string()),
        _ => return None,
    })
}

/// Where the totem is this frame: position in view space, a uniform scale, and the turns in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub scale: f32,
    pub spin: f32,
    pub rock_x: f32,
    pub rock_z: f32,
}

/// `ScreenEffectRenderer.renderItemActivationAnimation` with the player's settings: the same quintic brings the totem from 10 blocks
/// out to 1 and back; `speed` plays it faster, and past its end (or with the pop off) the totem is not drawn.
pub fn pose(ticks: i32, partial: f32, off_x: f32, off_y: f32, aspect: f32, s: &Settings) -> Pose {
    let hidden = Pose { x: 0.0, y: 0.0, z: -10.0, scale: 0.0, spin: 0.0, rock_x: 0.0, rock_z: 0.0 };
    let t = ((LENGTH - ticks) as f32 + partial) / LENGTH as f32 * s.speed;
    if !s.pop || t >= 1.0 {
        return hidden;
    }
    let (t2, t3) = (t * t, t * t * t);
    let curve = 10.25 * t3 * t2 - 24.95 * t2 * t2 + 25.5 * t3 - 13.8 * t2 + 4.0 * t;
    let angle = curve * std::f32::consts::PI;
    let swing = if s.swing { (angle * 2.0).sin().abs() } else { 0.0 };
    let rock = if s.rock { 6.0 * (t * 8.0).cos() } else { 0.0 };
    Pose {
        x: off_x * 0.3 * aspect * swing,
        y: off_y * 0.3 * swing,
        z: -10.0 + 9.0 * angle.sin(),
        scale: 0.8 * s.size as f32 / 100.0,
        spin: if s.spin { 900.0 * angle.sin().abs() } else { 0.0 },
        rock_x: rock,
        rock_z: rock,
    }
}
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_defaults_are_vanillas_curve() {
        let p = pose(40, 0.0, 0.5, -0.5, 16.0 / 9.0, &Settings::default());
        assert_eq!((p.x, p.y, p.z), (0.0, 0.0, -10.0), "first tick: straight ahead, 10 out");
        assert!((p.scale - 0.8).abs() < 1e-6);
        let mid = pose(20, 0.0, 0.5, -0.5, 16.0 / 9.0, &Settings::default());
        assert!((mid.z + 1.0).abs() < 1e-2, "halfway it is about 1 out: {}", mid.z);
        let a = 0.498_437_5 * std::f32::consts::PI;
        assert!((mid.x - 0.5 * 0.3 * (16.0 / 9.0) * (a * 2.0).sin().abs()).abs() < 1e-5);
    }

    #[test]
    fn off_hides_it_and_speed_ends_it_sooner() {
        let off = Settings { pop: false, ..Settings::default() };
        assert_eq!(pose(30, 0.0, 0.0, 0.0, 1.0, &off).scale, 0.0);
        let fast = Settings { speed: 2.0, ..Settings::default() };
        assert!(pose(30, 0.0, 0.0, 0.0, 1.0, &fast).scale > 0.0, "a quarter of the way in, still showing");
        assert_eq!(pose(20, 0.0, 0.0, 0.0, 1.0, &fast).scale, 0.0, "at double speed it is over at tick 20");
        let slow = Settings { speed: 0.5, ..Settings::default() };
        assert!(pose(2, 0.0, 0.0, 0.0, 1.0, &slow).scale > 0.0, "slow motion lasts to the end");
    }

    #[test]
    fn size_spin_rocking_and_swing_follow_the_settings() {
        let big = Settings { size: 200, ..Settings::default() };
        assert!((pose(25, 0.0, 0.3, 0.3, 1.0, &big).scale - 1.6).abs() < 1e-6);
        let still = Settings { spin: false, rock: false, swing: false, ..Settings::default() };
        let p = pose(25, 0.0, 0.7, 0.7, 1.7, &still);
        assert_eq!((p.spin, p.rock_x, p.rock_z, p.x, p.y), (0.0, 0.0, 0.0, 0.0, 0.0));
        let full = pose(25, 0.0, 0.7, 0.7, 1.7, &Settings::default());
        assert!(full.spin > 0.0 && full.x != 0.0);
        assert_eq!(full.z, p.z, "the flight in and out is not touched");
    }

    #[test]
    fn the_settings_page_cycles_and_reads_back() {
        assert_eq!(read(none), Settings::default());
        assert_eq!(click(none, 0), Some(("pop", "false".to_owned())));
        assert_eq!(click(none, 1), Some(("speed", "1.5".to_owned())));
        assert_eq!(click(none, 2), Some(("size", "125".to_owned())));
        assert_eq!(click(none, 6), None);
        let last = |k: &str| (k == "speed").then(|| "3".to_owned());
        assert_eq!(click(last, 1), Some(("speed", "0.5".to_owned())), "wraps");
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(read(junk), Settings { pop: true, ..Settings::default() });
        assert_eq!(labels(none)[1], "Speed: 1x");
        configure(true, Settings { size: 50, ..Settings::default() });
        assert!(active() && current().size == 50);
        configure(false, Settings::default());
    }
''',
    helpers={
        "pose": '''// The totem's pose for a frame of the pop, with the player's settings.
fn totem_tweaks_pose(ticks: i32, partial: f32, off_x: f32, off_y: f32, aspect: f32) -> std::ops::ControlFlow<Mat4> {
    use lib_totem_tweaks as m;
    if !m::active() {
        return std::ops::ControlFlow::Continue(());
    }
    let p = m::pose(ticks, partial, off_x, off_y, aspect, &m::current());
    std::ops::ControlFlow::Break(
        Mat4::from_translation(Vec3::new(p.x, p.y, p.z))
            * Mat4::from_scale(Vec3::splat(p.scale))
            * Mat4::from_rotation_y(p.spin.to_radians())
            * Mat4::from_rotation_x(p.rock_x.to_radians())
            * Mat4::from_rotation_z(p.rock_z.to_radians()),
    )
}
''',
        "tick": '''// Once a tick: tell the animation what the player chose.
fn totem_tweaks_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::item_activation::lib_totem_tweaks as m;
    if !game.native_window() {
        return;
    }
    let settings = m::read(|k| game.native_setting(m::ID, k));
    m::configure(game.native_on(m::ID), settings);
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ tweakeroo
TW.append(dict(
    slug="tweakeroo", id="tweakeroo", name="Tweakeroo", title="Tweakeroo", raw=True,
    usage="| Tweak | What it does |\n|---|---|\n| Hand restock | When the stack in your hand runs out, a stack of the same item from your inventory takes its place |\n| Hotbar cycle (`;`) | Rotates the hotbar and the three inventory rows: the hotbar goes to row 1, each row down one, the bottom row comes to the hotbar |\n| Fast right click | Placing and using repeat as fast as the game allows while the button is held |\n| Tool break warning | A chat warning when the tool in your hand is about to break |\n\nEach has a switch on the settings page. Restock and the hotbar cycle send slot clicks to the server in a burst, which some servers' anti-cheat flags: ask before using them online.",
    description="Four of Tweakeroo's tweaks: hand restock, hotbar cycle, fast right click and a tool break warning.",
    upstream="Tweakeroo (tweakeroo)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets (the original is LGPL-3.0)",
    command=None, hooks=["key", "tick", "list", "settings", "click"],
    options=[("restock", "Hand restock", True), ("cycle", "Hotbar cycle key", True), ("fast", "Fast right click", False), ("warn", "Tool break warning", True)],
    keys_expr='vec![[m::CYCLE_KEY.into(), "Cycle hotbar".into(), m::CYCLE_DEFAULT.into()]]',
    raw_note=("It reads the inventory and the held item and sets the game's use delay, and clicks slots with `Game::tweak_click`, the call Horizon's own inventory extras use. None of that is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. "
              "Restock and the cycle work when Horizon runs the world (single player, LAN host). **Only four of Tweakeroo's many tweaks are here**: flexible and accurate block placement, free camera (see Freecam), gamma override (built into Horizon), fast left click, the yeets and the rest are not. "
              "It was written from the original's public description, not its source."),
    runtime_extra=r'''
use std::cell::{Cell, RefCell as StateCell};

pub const CYCLE_KEY: &str = "cycle";
pub const CYCLE_DEFAULT: &str = "key.keyboard.semicolon";
/// A tool is about to break at or below this much durability left.
pub const WARN_AT: u32 = 10;

thread_local! {
    /// The selected slot and what it held at the last tick: (slot, item).
    static LAST_HELD: StateCell<Option<(usize, String)>> = const { StateCell::new(None) };
    /// The durability left on the held tool at the last tick, for the warning to fire once when it crosses `WARN_AT`.
    static LAST_LEFT: Cell<Option<u32>> = const { Cell::new(None) };
}

/// Forgets what was held (no world, or the mod is off).
pub fn reset() {
    LAST_HELD.with_borrow_mut(|l| *l = None);
    LAST_LEFT.with(|l| l.set(None));
}

/// Hand restock for one tick. `held` is the selected slot and the item in it now (none when empty); `inventory` is the 36 slots,
/// hotbar first. Returns the inventory slot to move into the selected slot, when the stack in hand has just run out.
pub fn restock(held: (usize, Option<&str>), inventory: &[Option<String>]) -> Option<usize> {
    let (slot, now) = held;
    let before = LAST_HELD.with_borrow_mut(|l| {
        let before = l.clone();
        *l = now.map(|item| (slot, item.to_owned()));
        before
    });
    let (before_slot, item) = before?;
    if now.is_some() || before_slot != slot {
        return None;
    }
    (9..inventory.len()).find(|&i| inventory[i].as_deref() == Some(item.as_str()))
}

/// The (inventory slot, hotbar slot) swaps that rotate the rows: the hotbar swaps with row 1, then row 2, then row 3, which
/// moves the hotbar to row 1, row 1 to row 2, row 2 to row 3 and row 3 to the hotbar.
pub fn cycle_plan() -> Vec<(usize, usize)> {
    (1..=3).flat_map(|row| (0..9).map(move |i| (9 * row + i, i))).collect()
}

/// Forgets what was held, keeping the warning state (the restock is paused: a screen is open, or it is off).
pub fn reset_held() {
    LAST_HELD.with_borrow_mut(|l| *l = None);
}

/// The tool-break warning: true once, when the durability left falls to `WARN_AT` or less from above it.
pub fn warn(left: Option<u32>) -> bool {
    let before = LAST_LEFT.with(|l| l.replace(left));
    matches!((before, left), (Some(b), Some(l)) if b > WARN_AT && l <= WARN_AT)
}
''',
    tests=r'''
    fn inv(items: &[(usize, &str)]) -> Vec<Option<String>> {
        let mut v = vec![None; 36];
        for (i, item) in items {
            v[*i] = Some((*item).to_owned());
        }
        v
    }

    #[test]
    fn a_used_up_stack_is_replaced_from_the_inventory() {
        reset();
        let inventory = inv(&[(3, "stone"), (12, "stone"), (20, "dirt")]);
        assert_eq!(restock((3, Some("stone")), &inventory), None, "still holding it");
        let after = inv(&[(12, "stone"), (20, "dirt")]);
        assert_eq!(restock((3, None), &after), Some(12), "the first matching stack past the hotbar");
        assert_eq!(restock((3, None), &after), None, "once");
    }

    #[test]
    fn nothing_to_restock_with_changes_of_slot_or_no_match() {
        reset();
        let a = inv(&[(3, "stone"), (12, "dirt")]);
        restock((3, Some("stone")), &a);
        assert_eq!(restock((4, None), &a), None, "the player changed slot");
        reset();
        restock((3, Some("stone")), &a);
        assert_eq!(restock((3, None), &inv(&[(12, "dirt")])), None, "no stone left");
        reset();
        assert_eq!(restock((3, None), &a), None, "an empty hand that was never full");
    }

    #[test]
    fn the_cycle_swaps_each_row_through_the_hotbar() {
        let plan = cycle_plan();
        assert_eq!(plan.len(), 27);
        assert_eq!(plan[0], (9, 0));
        assert_eq!(plan[26], (35, 8));
        // Play it on rows named by letters: hotbar a, rows b c d.
        let mut slots: Vec<char> = "aaaaaaaaabbbbbbbbbcccccccccddddddddd".chars().collect();
        for (inventory, hotbar) in plan {
            slots.swap(inventory, hotbar);
        }
        let rows: Vec<String> = slots.chunks(9).map(|r| r.iter().collect()).collect();
        assert_eq!(rows, ["ddddddddd", "aaaaaaaaa", "bbbbbbbbb", "ccccccccc"], "the hotbar goes to row 1, each row down, row 3 comes up");
    }

    #[test]
    fn the_warning_fires_once_on_crossing() {
        reset();
        assert!(!warn(Some(200)));
        assert!(!warn(Some(12)));
        assert!(warn(Some(9)), "crossed 10");
        assert!(!warn(Some(8)), "not again");
        assert!(!warn(None));
        assert!(!warn(Some(9)), "a new tool already below the line does not warn");
        assert!(!warn(Some(300)));
        assert!(warn(Some(10)));
        reset();
    }
''',
    helpers={
        "key": '''// The hotbar cycle key.
fn tweakeroo_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::@MOD@ as m;
    use horizon_client::menu::Input;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() || game.remote.is_some() {
        return;
    }
    if key != game.native_mod_key(m::ID, m::CYCLE_KEY, m::CYCLE_DEFAULT) || !m::is(|k| game.native_setting(m::ID, k), "cycle") {
        return;
    }
    for (slot, hotbar) in m::cycle_plan() {
        game.tweak_click(slot, hotbar as i32, Input::Swap);
    }
}
''',
        "tick": '''// Once a tick: hand restock, fast right click and the tool break warning.
fn tweakeroo_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::@MOD@ as m;
    use horizon_client::menu::Input;
    if !game.native_on(m::ID) || !game.native_window() || game.native_player().is_none() {
        m::reset();
        return;
    }
    let (restock, fast, warn) = {
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "restock"), m::is(get, "fast"), m::is(get, "warn"))
    };
    let playing = game.native_playing();
    if fast && playing && game.right {
        game.use_delay = 0;
    }
    let selected = game.selected;
    let held = game.inventory.get(selected).and_then(|s| s.as_ref()).map(|s| s.item.to_string());
    if restock && playing && game.remote.is_none() && game.carried.is_none() {
        let inventory: Vec<Option<String>> = game.inventory.iter().take(36).map(|s| s.as_ref().map(|s| s.item.to_string())).collect();
        if let Some(from) = m::restock((selected, held.as_deref()), &inventory) {
            game.tweak_click(from, 0, Input::Pickup);
            game.tweak_click(selected, 0, Input::Pickup);
        }
    } else {
        m::reset_held();
    }
    let left = game.inventory.get(selected).and_then(|s| s.as_ref()).and_then(|s| {
        let max = game.items().get(&s.item).map_or(0, |i| i.max_damage);
        (max > 0).then(|| max.saturating_sub(s.damage))
    });
    if m::warn(if warn { left } else { None }) {
        game.native_chat("Your tool is about to break!");
    }
}
''',
    },
))
