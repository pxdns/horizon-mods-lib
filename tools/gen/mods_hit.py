"""Combat Hitboxes: wraps Horizon's ray-vs-box test so entity boxes are a little bigger for aiming. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, LIST_B, SET_B, CLK_B

HIT = []

HIT.append(dict(
    slug="combat-hitboxes", id="combat_hitboxes", name="Combat Hitboxes", title="Combat Hitboxes", raw=True,
    usage="Nothing to press: entities are a little easier to hit, because the box your crosshair is tested against is bigger. The first button picks how much (0, 0.05, 0.1, 0.2 or 0.3 blocks).",
    description="Makes entities slightly easier to hit by growing the box your crosshair is tested against.",
    upstream="Combat Hitboxes (combat-hitbox)", upstream_licence="see the upstream project",
    command=None, hooks=["ray", "frame", "list", "settings", "click"],
    options=None, custom_settings=True,
    raw_note=("It puts a cancellable HEAD hook on `ray_box`, the ray-versus-box test in Horizon's `game.rs`, which runs the same test again on a larger box. That function is not part of the stable "
              "hook-point contract, so the mod is **pinned to one Horizon commit**. It grows the boxes of everything the crosshair is tested against, and only changes where you aim: it does not change who the server says you hit."),
    toml_text=toml_text("combat_hitboxes", "1.0.0", "crates/horizon-client/src/game.rs", "lib_combat_hitboxes", [
        dict(fields=[("id", '"ray"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client::game::ray_box"'), ("helper", '"ray.rs"')],
             bindings=[("eye", "[f64; 3]", "eye"), ("dir", "[f64; 3]", "dir"), ("reach", "f64", "reach"), ("b", "&Aabb", "&b")]),
        native("frame", "native_frame", [G, ("elapsed", "std::time::Duration", "elapsed")]),
        native("list", "native_mods", LIST_B), native("settings", "native_settings", SET_B), native("click", "native_settings_click", CLK_B),
    ]),
    runtime_extra=r'''
use std::cell::Cell;

/// How much the boxes grow, in hundredths of a block: the first button cycles through these.
pub const LEVELS: [u32; 5] = [0, 5, 10, 20, 30];
/// The default, a tenth of a block.
pub const DEFAULT: u32 = 10;

thread_local! {
    static GROW: Cell<u32> = const { Cell::new(DEFAULT) };
    /// True while the mod runs the test again on the larger box.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// Sets the growth from the stored value (the game tells the mod each frame).
pub fn configure(hundredths: u32) {
    GROW.with(|g| g.set(hundredths));
}

/// The stored `grow`, one of `LEVELS`, else the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> u32 {
    get("grow").and_then(|v| v.trim().parse().ok()).filter(|v| LEVELS.contains(v)).unwrap_or(DEFAULT)
}

/// How much to grow the box by in blocks, and marks the nested test; None when there is nothing to grow
/// or the test is already the nested one.
pub fn enter() -> Option<f64> {
    if INSIDE.with(Cell::get) {
        return None;
    }
    let grow = GROW.with(Cell::get);
    if grow == 0 {
        return None;
    }
    INSIDE.with(|i| i.set(true));
    Some(f64::from(grow) / 100.0)
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    vec![format!("Hitbox growth: +{:.2} blocks", f64::from(read(get)) / 100.0)]
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    if i != 0 {
        return None;
    }
    let current = read(get);
    let at = LEVELS.iter().position(|&l| l == current).unwrap_or(0);
    let next = LEVELS[(at + 1) % LEVELS.len()];
    configure(next);
    Some(("grow", next.to_string()))
}
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_default_is_a_tenth_of_a_block() {
        assert_eq!(read(none), 10);
        assert_eq!(labels(none), ["Hitbox growth: +0.10 blocks"]);
    }

    #[test]
    fn the_button_cycles_and_sets_the_growth() {
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("grow", "20"));
        assert_eq!(enter(), Some(0.2));
        leave();
        let at_end = |k: &str| (k == "grow").then(|| "30".to_owned());
        assert_eq!(click(at_end, 0).unwrap().1, "0", "30 wraps to 0");
        assert_eq!(enter(), None, "no growth, nothing to run again");
        assert!(click(none, 4).is_none());
        configure(DEFAULT);
    }

    #[test]
    fn the_nested_test_runs_the_original() {
        configure(10);
        assert_eq!(enter(), Some(0.1));
        assert_eq!(enter(), None, "nested");
        leave();
        assert_eq!(enter(), Some(0.1));
        leave();
    }

    #[test]
    fn stored_junk_falls_back() {
        assert_eq!(read(|_| Some("7".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some("banana".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some(" 5\n".to_owned())), 5);
    }
''',
    helpers={
        "ray": '''// The crosshair's ray against an entity's box, run again on a box that has grown.
fn combat_hitboxes_ray(eye: [f64; 3], dir: [f64; 3], reach: f64, b: &Aabb) -> std::ops::ControlFlow<Option<f64>> {
    use crate::game::lib_combat_hitboxes as m;
    let Some(grow) = m::enter() else {
        return std::ops::ControlFlow::Continue(());
    };
    let bigger = b.clone().inflate(Vec3::new(grow, grow, grow));
    let hit = ray_box(eye, dir, reach, bigger);
    m::leave();
    std::ops::ControlFlow::Break(hit)
}
''',
        "frame": '''// Each frame: tell the mod how much the player wants the boxes to grow.
fn combat_hitboxes_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::lib_combat_hitboxes as m;
    let _ = elapsed;
    m::configure(if game.native_on(m::ID) { m::read(|k| game.native_setting(m::ID, k)) } else { 0 });
}
''',
    },
))
