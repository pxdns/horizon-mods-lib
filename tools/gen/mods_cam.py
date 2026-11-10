"""Camera and input ports: they hook Horizon's mouse look (`turn_player`), render camera (`camera_view`), hurt camera (`bob_matrix`)
and the held-key set, with cancellable HEAD hooks. Pinned to one Horizon commit."""
from mods import opts

CAM = []
CF = "std::ops::ControlFlow"
MOVE_KEYS = "[K::KeyW, K::KeyA, K::KeyS, K::KeyD, K::Space, K::ShiftLeft, K::ControlLeft]"

KEY_HELPER = '''// The key went down or up: the player's own choice of key.
fn @ID@_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::@MOD@ as m;
    if !game.native_on(m::ID) {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let playing = game.native_playing();
@BODY@
}
'''

TURN_HELPER = '''// Mouse look: while the mod owns the view, the mouse turns that and not the player.
fn @ID@_turn(game: &mut crate::game::Game<'_>, xo: f64, yo: f64) -> std::ops::ControlFlow<()> {
    use crate::game::@MOD@ as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
@PRE@
    if m::turn(xo, yo@ARG@) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
'''

CAMERA_HELPER = '''// The camera the frame renders from: the original run again with the mod's own angles, then adjusted.
fn @ID@_camera(
    game: &mut crate::game::Game<'_>,
    partial: f32,
) -> std::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    use crate::game::@MOD@ as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    // `enter` is None when the mod is off or this is the nested call: the original body runs.
    let Some((yaw, pitch)) = m::enter() else {
        return std::ops::ControlFlow::Continue(());
    };
    let (real_yaw, real_pitch) = (game.yaw, game.pitch);
    game.yaw = yaw;
    game.pitch = pitch;
    let view = game.camera_view(partial);
    game.yaw = real_yaw;
    game.pitch = real_pitch;
    m::leave();
@FINISH@
}
'''

# ------------------------------------------------------------------------------------------------ freelook
CAM.append(dict(
    usage="| Key | Default | What it does |\n|---|---|---|\n| Freelook | `Left Alt` | Hold (or press, with *Toggle*) and move the mouse to look around; your player keeps facing the same way |", slug="freelook", id="freelook", name="Freelook", title="Freelook", raw=True,
    description="Hold a key and move the mouse to look around without turning your player.",
    upstream="Freelook (freelook)", upstream_licence="see the upstream project",
    command=None, hooks=["key", "turn", "camera", "list", "settings", "click"],
    options=[("toggle", "Toggle instead of hold", False), ("invert", "Invert pitch", False)],
    keys_expr='vec![[m::KEY.into(), "Freelook".into(), m::DEFAULT_KEY.into()]]',
    raw_note=("It hooks Horizon's mouse-look function (`turn_player`) and render camera (`camera_view`) with cancellable HEAD hooks, which are not part of the stable "
              "hook-point contract, so it is **pinned to one Horizon commit** and has to be re-checked when Horizon changes them."),
    runtime_extra=r'''
use std::cell::Cell;

/// The key's name in Controls, and its default.
pub const KEY: &str = "freelook";
pub const DEFAULT_KEY: &str = "key.keyboard.left.alt";

/// The view while freelook is on: yaw and pitch of the camera, in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
}

thread_local! {
    static VIEW: Cell<Option<View>> = const { Cell::new(None) };
    /// True while the camera function runs for the mod, so its nested call is the original.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// The freelook key went down or up; `yaw`/`pitch` are the player's now.
pub fn key(playing: bool, pressed: bool, toggle: bool, yaw: f32, pitch: f32) {
    let on = VIEW.with(Cell::get).is_some();
    let want = if toggle {
        if !pressed {
            return;
        }
        !on
    } else {
        pressed
    };
    if want && !on {
        if playing {
            VIEW.with(|v| v.set(Some(View { yaw, pitch })));
        }
    } else if !want && on {
        VIEW.with(|v| v.set(None));
    }
}

/// Whether freelook is on.
#[allow(dead_code)]
pub fn active() -> bool {
    VIEW.with(Cell::get).is_some()
}

/// Mouse deltas as `Entity.turn` takes them: 0.15 degrees per unit, pitch kept within 90. True when freelook took them.
pub fn turn(xo: f64, yo: f64, invert: bool) -> bool {
    VIEW.with(|v| match v.get() {
        Some(mut view) => {
            let pitch_delta = (yo as f32) * 0.15 * if invert { -1.0 } else { 1.0 };
            view.yaw += (xo as f32) * 0.15;
            view.pitch = (view.pitch + pitch_delta).clamp(-90.0, 90.0);
            v.set(Some(view));
            true
        }
        None => false,
    })
}

/// The angles to render from, and marks the nested call; None when freelook is off or the call is already nested.
pub fn enter() -> Option<(f32, f32)> {
    if INSIDE.with(Cell::get) {
        return None;
    }
    let view = VIEW.with(Cell::get)?;
    INSIDE.with(|i| i.set(true));
    Some((view.yaw, view.pitch))
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// Turns freelook off (tests only).
#[cfg(test)]
pub fn forget() {
    VIEW.with(|v| v.set(None));
    INSIDE.with(|i| i.set(false));
}
''',
    tests=r'''
    #[test]
    fn hold_starts_on_press_and_stops_on_release() {
        forget();
        key(true, true, false, 30.0, 10.0);
        assert!(active());
        assert_eq!(enter(), Some((30.0, 10.0)));
        leave();
        key(false, false, false, 0.0, 0.0);
        assert!(!active(), "released on any screen");
        forget();
    }

    #[test]
    fn a_screen_cannot_start_it() {
        forget();
        key(false, true, false, 0.0, 0.0);
        assert!(!active());
        forget();
    }

    #[test]
    fn toggle_flips_on_each_press_and_ignores_release() {
        forget();
        key(true, true, true, 0.0, 0.0);
        key(true, false, true, 0.0, 0.0);
        assert!(active());
        key(true, true, true, 0.0, 0.0);
        assert!(!active());
        forget();
    }

    #[test]
    fn the_mouse_turns_the_view_not_the_player() {
        forget();
        assert!(!turn(10.0, 10.0, false), "off: the player turns");
        key(true, true, false, 0.0, 0.0);
        assert!(turn(100.0, 100.0, false));
        let (yaw, pitch) = enter().unwrap();
        assert!((yaw - 15.0).abs() < 1e-4 && (pitch - 15.0).abs() < 1e-4);
        leave();
        assert!(turn(0.0, 1000.0, false));
        assert_eq!(enter().unwrap().1, 90.0, "pitch stops at 90");
        leave();
        assert!(turn(0.0, 10_000.0, true));
        assert_eq!(enter().unwrap().1, -90.0, "inverted pitch goes the other way");
        leave();
        forget();
    }

    #[test]
    fn the_nested_camera_call_runs_the_original() {
        forget();
        key(true, true, false, 5.0, 6.0);
        assert!(enter().is_some());
        assert_eq!(enter(), None, "nested");
        leave();
        assert!(enter().is_some());
        leave();
        forget();
    }
''',
    helpers={
        "key": KEY_HELPER.replace("@BODY@", '''    let toggle = m::is(|k| game.native_setting(m::ID, k), "toggle");
    m::key(playing, pressed, toggle, game.yaw, game.pitch);'''),
        "turn": TURN_HELPER.replace("@PRE@", '''    let invert = m::is(|k| game.native_setting(m::ID, k), "invert");''').replace("@ARG@", ", invert"),
        "camera": CAMERA_HELPER.replace("@FINISH@", "    std::ops::ControlFlow::Break(view)"),
    },
))

# ------------------------------------------------------------------------------------------------ freecam
CAM.append(dict(
    usage="| Key | Default | What it does |\n|---|---|---|\n| Freecam | `F8` | Press to detach the camera, press again to return. W A S D fly in the direction you look, Space goes up, Shift down, Ctrl faster |", slug="freecam", id="freecam", name="Freecam", title="Freecam", raw=True,
    description="Detach the camera and fly it around while your player stands still.",
    upstream="Freecam (freecam)", upstream_licence="see the upstream project",
    command=None, hooks=["key", "frame", "tick", "turn", "camera", "list", "settings", "click"],
    options=[("fast", "Fast flight (3x)", False)],
    keys_expr='vec![[m::KEY.into(), "Freecam".into(), m::DEFAULT_KEY.into()]]',
    raw_note=("It hooks Horizon's mouse-look function (`turn_player`) and render camera (`camera_view`) with cancellable HEAD hooks, and reads and edits the held-key sets of the "
              "`Game`, none of which is part of the stable hook-point contract, so it is **pinned to one Horizon commit**. It assumes the default W A S D, Space and Shift movement keys. "
              "Your clicks still act as your player's while the camera is detached."),
    runtime_extra=r'''
use std::cell::Cell;

/// The key's name in Controls, and its default.
pub const KEY: &str = "freecam";
pub const DEFAULT_KEY: &str = "key.keyboard.f8";
/// Blocks per second; three times that when fast.
pub const SPEED: f64 = 10.0;

/// The movement keys held.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Input {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fast: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct State {
    yaw: f32,
    pitch: f32,
    /// Where the camera is; filled from the player's own camera the first frame.
    at: Option<[f64; 3]>,
}

thread_local! {
    static STATE: Cell<Option<State>> = const { Cell::new(None) };
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// The key went down: freecam turns on (only while playing) or off (anywhere).
pub fn key(playing: bool, pressed: bool, yaw: f32, pitch: f32) {
    if !pressed {
        return;
    }
    if STATE.with(Cell::get).is_some() {
        STATE.with(|s| s.set(None));
    } else if playing {
        STATE.with(|s| s.set(Some(State { yaw, pitch, at: None })));
    }
}

pub fn active() -> bool {
    STATE.with(Cell::get).is_some()
}

/// Mouse deltas as `Entity.turn` takes them. True when freecam took them.
pub fn turn(xo: f64, yo: f64) -> bool {
    STATE.with(|s| match s.get() {
        Some(mut st) => {
            st.yaw += (xo as f32) * 0.15;
            st.pitch = (st.pitch + (yo as f32) * 0.15).clamp(-90.0, 90.0);
            s.set(Some(st));
            true
        }
        None => false,
    })
}

/// The velocity in blocks per second for a view and the keys held: forward goes where the camera looks.
pub fn velocity(yaw: f32, pitch: f32, input: Input, fast_option: bool) -> [f64; 3] {
    let (y, p) = (f64::from(yaw).to_radians(), f64::from(pitch).to_radians());
    let forward = [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()];
    let right = [-y.cos(), 0.0, -y.sin()];
    let axis = |plus: bool, minus: bool| f64::from(i32::from(plus) - i32::from(minus));
    let (f, r, u) = (axis(input.forward, input.back), axis(input.right, input.left), axis(input.up, input.down));
    let speed = if input.fast || fast_option { SPEED * 3.0 } else { SPEED };
    let v = [forward[0] * f + right[0] * r, forward[1] * f + u, forward[2] * f + right[2] * r];
    let len = v.iter().map(|c| c * c).sum::<f64>().sqrt();
    if len < 1e-9 {
        return [0.0; 3];
    }
    [v[0] / len * speed, v[1] / len * speed, v[2] / len * speed]
}

/// One frame of `seconds`: moves the camera.
pub fn step(seconds: f32, input: Input, fast_option: bool) {
    STATE.with(|s| {
        if let Some(mut st) = s.get() {
            if let Some(at) = &mut st.at {
                let v = velocity(st.yaw, st.pitch, input, fast_option);
                for (a, d) in at.iter_mut().zip(v) {
                    *a += d * f64::from(seconds);
                }
            }
            s.set(Some(st));
        }
    });
}

/// The angles to render from, and marks the nested call; None when freecam is off or the call is nested.
pub fn enter() -> Option<(f32, f32)> {
    if INSIDE.with(Cell::get) {
        return None;
    }
    let st = STATE.with(Cell::get)?;
    INSIDE.with(|i| i.set(true));
    Some((st.yaw, st.pitch))
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// The camera to render from: the player's own camera the first frame (so freecam starts where you are), then where it was flown.
pub fn view(real: Option<([f64; 3], f32, f32)>) -> Option<([f64; 3], f32, f32)> {
    let (position, yaw, pitch) = real?;
    STATE.with(|s| {
        let mut st = s.get()?;
        let at = *st.at.get_or_insert(position);
        s.set(Some(st));
        Some((at, yaw, pitch))
    })
}

/// Turns freecam off (tests only).
#[cfg(test)]
pub fn forget() {
    STATE.with(|s| s.set(None));
    INSIDE.with(|i| i.set(false));
}
''',
    tests=r'''
    fn walking(forward: bool) -> Input {
        Input { forward, ..Input::default() }
    }

    #[test]
    fn the_key_toggles_and_only_starts_while_playing() {
        forget();
        key(false, true, 0.0, 0.0);
        assert!(!active());
        key(true, false, 0.0, 0.0);
        assert!(!active(), "release does nothing");
        key(true, true, 0.0, 0.0);
        assert!(active());
        key(false, true, 0.0, 0.0);
        assert!(!active(), "off from any screen");
        forget();
    }

    #[test]
    fn forward_goes_where_the_camera_looks() {
        let v = velocity(0.0, 0.0, walking(true), false);
        assert!((v[2] - SPEED).abs() < 1e-9 && v[0].abs() < 1e-9 && v[1].abs() < 1e-9, "yaw 0 faces +Z: {v:?}");
        let west = velocity(90.0, 0.0, walking(true), false);
        assert!((west[0] + SPEED).abs() < 1e-9, "yaw 90 faces -X: {west:?}");
        let down = velocity(0.0, 90.0, walking(true), false);
        assert!((down[1] + SPEED).abs() < 1e-9, "looking down flies down: {down:?}");
    }

    #[test]
    fn strafing_up_diagonals_and_speed() {
        let right = velocity(0.0, 0.0, Input { right: true, ..Input::default() }, false);
        assert!((right[0] + SPEED).abs() < 1e-9, "facing south, right is -X: {right:?}");
        let up = velocity(0.0, 0.0, Input { up: true, ..Input::default() }, false);
        assert_eq!(up, [0.0, SPEED, 0.0]);
        let both = velocity(0.0, 0.0, Input { forward: true, right: true, ..Input::default() }, false);
        let len = both.iter().map(|c| c * c).sum::<f64>().sqrt();
        assert!((len - SPEED).abs() < 1e-9, "diagonals are not faster");
        let fast = velocity(0.0, 0.0, walking(true), true);
        assert!((fast[2] - SPEED * 3.0).abs() < 1e-9);
        assert_eq!(velocity(0.0, 0.0, Input::default(), false), [0.0; 3]);
        assert_eq!(velocity(0.0, 0.0, Input { forward: true, back: true, ..Input::default() }, false), [0.0; 3]);
    }

    #[test]
    fn the_camera_starts_at_the_players_and_then_flies() {
        forget();
        assert_eq!(view(Some(([1.0, 2.0, 3.0], 0.0, 0.0))), None, "off: the game's own camera");
        key(true, true, 0.0, 0.0);
        assert_eq!(view(Some(([1.0, 2.0, 3.0], 5.0, 6.0))).unwrap().0, [1.0, 2.0, 3.0]);
        step(0.5, walking(true), false);
        let moved = view(Some(([9.0, 9.0, 9.0], 5.0, 6.0))).unwrap().0;
        assert!((moved[2] - 8.0).abs() < 1e-9 && moved[0] == 1.0, "half a second at 10 b/s: {moved:?}");
        forget();
    }

    #[test]
    fn the_mouse_turns_the_camera_and_the_nested_call_is_the_original() {
        forget();
        assert!(!turn(1.0, 1.0));
        key(true, true, 0.0, 0.0);
        assert!(turn(200.0, -2000.0));
        let (yaw, pitch) = enter().unwrap();
        assert!((yaw - 30.0).abs() < 1e-4 && pitch == -90.0);
        assert_eq!(enter(), None);
        leave();
        forget();
    }
''',
    helpers={
        "key": KEY_HELPER.replace("@BODY@", '''    m::key(playing, pressed, game.yaw, game.pitch);
    if !m::active() {
        // Give the movement keys back that are still held.
        use winit::keyboard::KeyCode as K;
        for k in @KEYS@ {
            if game.physical_keys.contains(&k) {
                game.keys.insert(k);
            }
        }
    }'''.replace("@KEYS@", MOVE_KEYS)),
        "frame": '''// Each frame: fly the camera with the movement keys and keep them from moving the player.
fn freecam_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::lib_freecam as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) || !m::active() {
        return;
    }
    let fast = m::is(|k| game.native_setting(m::ID, k), "fast");
    let held = |k: K| game.physical_keys.contains(&k);
    let input = m::Input {
        forward: held(K::KeyW),
        back: held(K::KeyS),
        left: held(K::KeyA),
        right: held(K::KeyD),
        up: held(K::Space),
        down: held(K::ShiftLeft),
        fast: held(K::ControlLeft),
    };
    if game.native_playing() {
        m::step(elapsed.as_secs_f32(), input, fast);
    }
    for k in @KEYS@ {
        game.keys.remove(&k);
    }
}
'''.replace("@KEYS@", MOVE_KEYS),
        "tick": '''// Once a tick, before the player's movement is read: the movement keys belong to the camera.
fn freecam_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_freecam as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) || !m::active() {
        return;
    }
    for k in @KEYS@ {
        game.keys.remove(&k);
    }
}
'''.replace("@KEYS@", MOVE_KEYS),
        "turn": TURN_HELPER.replace("@PRE@", "").replace("@ARG@", ""),
        "camera": CAMERA_HELPER.replace("@FINISH@", '''    let real = view.map(|(p, yaw, pitch)| ([p.x, p.y, p.z], yaw, pitch));
    std::ops::ControlFlow::Break(m::view(real).map(|(p, yaw, pitch)| (glam::DVec3::new(p[0], p[1], p[2]), yaw, pitch)))'''),
    },
))

# ------------------------------------------------------------------------------------------------ snappy_tappy
CAM.append(dict(
    usage="| Key | Default | What it does |\n|---|---|---|\n| Toggle Snappy Tappy | `5` | Turns the mod on or off and says `Snappy Tappy: ON` or `OFF` over the hotbar |\n\nHold your left and right keys (or forward and back) together and the one pressed last wins; let go of it and the other carries on. It uses the keys you have bound. The settings page has Enabled.",
    slug="snappy-tappy", id="snappy_tappy", name="SnappyTappy", title="SnappyTappy", raw=True,
    description="Last-input priority for movement keys: pressing D while holding A moves you right at once, then back to A when you let go.",
    upstream="SnappyTappy (snappytappy) by cutebow", upstream_licence="MIT",
    command=None, hooks=["key", "tick", "list", "settings", "click"],
    options=[("enabled", "Enabled", True)],
    keys_expr='vec![[m::KEY.into(), "Toggle Snappy Tappy".into(), m::DEFAULT_KEY.into()]]',
    raw_note=("It edits the `Game`'s held-key set before movement is read, and says ON or OFF through the game's action bar (`Game.titles`), which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    runtime_extra=r'''
use std::cell::Cell;
use std::collections::HashMap;

/// `key.snappy_tappy.toggle`, default `5` (GLFW 53).
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.5";

thread_local! {
    /// The tick each key last went down, by its name (`key.keyboard.a`), as `pressTick`.
    static PRESSED: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static TICK: Cell<u64> = const { Cell::new(0) };
}

/// One game tick passed (`InputInterceptor.tick`).
pub fn next_tick() {
    TICK.with(|t| t.set(t.get() + 1));
}

/// A key went down (`down`) or up: a press records the tick.
pub fn pressed(key: &str, down: bool) {
    let tick = TICK.with(Cell::get);
    PRESSED.with_borrow_mut(|p| {
        if down {
            p.entry(key.to_owned()).or_insert(tick);
        } else {
            p.remove(key);
        }
    });
}

/// When `key` went down, 0 when it is not known.
pub fn press_tick(key: &str) -> u64 {
    PRESSED.with_borrow(|p| p.get(key).copied().unwrap_or(0))
}

/// `resolvePair`: of two opposite keys held together, the one pressed later counts; at the same tick the first (left, forward) does.
/// Returns (first counts as held, second counts as held). One key held is just held.
pub fn resolve(first_held: bool, second_held: bool, first_tick: u64, second_tick: u64) -> (bool, bool) {
    if first_held && second_held {
        if first_tick >= second_tick { (true, false) } else { (false, true) }
    } else {
        (first_held, second_held)
    }
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget() {
    PRESSED.with_borrow_mut(HashMap::clear);
    TICK.with(|t| t.set(0));
}
''',
    tests=r'''
    #[test]
    fn the_later_press_of_two_held_keys_wins() {
        assert_eq!(resolve(true, true, 5, 9), (false, true));
        assert_eq!(resolve(true, true, 9, 5), (true, false));
        assert_eq!(resolve(true, true, 7, 7), (true, false), "the same tick: the first key");
        assert_eq!(resolve(true, false, 9, 1), (true, false), "one key held is just held");
        assert_eq!(resolve(false, false, 0, 0), (false, false));
    }

    #[test]
    fn presses_are_timed_and_a_release_forgets_them() {
        forget();
        next_tick();
        pressed("key.keyboard.a", true);
        next_tick();
        next_tick();
        pressed("key.keyboard.d", true);
        assert_eq!((press_tick("key.keyboard.a"), press_tick("key.keyboard.d")), (1, 3));
        pressed("key.keyboard.d", false);
        assert_eq!(press_tick("key.keyboard.d"), 0);
        pressed("key.keyboard.a", true);
        assert_eq!(press_tick("key.keyboard.a"), 1, "a key repeat does not move its press");
        forget();
    }

    #[test]
    fn a_pair_switches_to_the_newer_key_and_back() {
        forget();
        next_tick();
        pressed("key.keyboard.a", true);
        next_tick();
        pressed("key.keyboard.d", true);
        let now = || resolve(true, true, press_tick("key.keyboard.a"), press_tick("key.keyboard.d"));
        assert_eq!(now(), (false, true), "D pressed last: right");
        pressed("key.keyboard.d", false);
        assert_eq!(resolve(true, false, press_tick("key.keyboard.a"), 0), (true, false), "back to A");
        forget();
    }

    #[test]
    fn the_toggle_key_is_five() {
        assert_eq!(DEFAULT_KEY, "key.keyboard.5");
    }
''',
    helpers={
        "key": '''// A key went down or up: time it, and the toggle key turns the mod on or off.
fn snappy_tappy_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_snappy_tappy as m;
    m::pressed(key, pressed);
    if pressed && game.native_playing() && key == game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        let now = !m::is(|k| game.native_setting(m::ID, k), "enabled");
        game.native_set_setting(m::ID, "enabled", if now { "true" } else { "false" });
        let line = format!("Snappy Tappy: {}", if now { "ON" } else { "OFF" });
        game.titles.set_overlay(horizon_client::text::Text::literal(line), false);
    }
}
''',
        "tick": '''// Once a tick, before movement is read: of two opposite bound keys held together, only the last pressed counts.
fn snappy_tappy_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_snappy_tappy as m;
    m::next_tick();
    if !game.native_on(m::ID) || !game.native_window() || !game.native_playing() {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "enabled") {
        return;
    }
    // The keys the player bound, as the original reads Options.keyLeft and the rest.
    let bound = |mapping: &str| {
        let name = game.settings.keys.get(mapping).to_owned();
        horizon_client::keybinds::key_code(&name).map(|code| (name, code))
    };
    for (first, second) in [("key.left", "key.right"), ("key.forward", "key.back")] {
        let (Some((first_name, first_code)), Some((second_name, second_code))) = (bound(first), bound(second)) else {
            continue;
        };
        let held = |k| game.physical_keys.contains(&k);
        let (a, b) = m::resolve(held(first_code), held(second_code), m::press_tick(&first_name), m::press_tick(&second_name));
        for (key, want) in [(first_code, a), (second_code, b)] {
            if want {
                game.keys.insert(key);
            } else {
                game.keys.remove(&key);
            }
        }
    }
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ better_hurt_cam
CAM.append(dict(
    usage="| Key | Default | What it does |\n|---|---|---|\n| Toggle HurtCam | unbound | Turns the hurt camera on or off |\n| Increase HurtCam Strength | unbound | Multiplier +0.1, shown over the hotbar |\n| Decrease HurtCam Strength | unbound | Multiplier -0.1, shown over the hotbar |\n\nThe settings page has Hurtcam Enabled, Hurtcam Multiplier, Health Bar Blinking and Hurtcam Type (Old or Yaw-based).",
    slug="better-hurt-cam", id="better_hurt_cam", name="BetterHurtCam", title="BetterHurtCam", raw=False, needs_api=402,
    description="nohurtcam but better: turn the hurt camera off or change its intensity, with a key or the settings page.",
    upstream="BetterHurtCam (betterhurtcam) by uku", upstream_licence="MIT",
    command=None, hooks=["key", "list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    keys_expr='vec![[m::KEY_TOGGLE.into(), "Toggle HurtCam".into(), m::DEFAULT_KEY.into()], [m::KEY_PLUS.into(), "Increase HurtCam Strength".into(), m::DEFAULT_KEY.into()], [m::KEY_MINUS.into(), "Decrease HurtCam Strength".into(), m::DEFAULT_KEY.into()]]',
    runtime_extra=r'''
/// The three keys (`key.betterhurtcam.toggle`, `.plus`, `.minus`) and their default: unbound. The original's bytecode gives
/// key codes 65, 64 and 63 (an `A`, and two codes GLFW has no key for), which cannot be meant as defaults, so none is bound.
pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_PLUS: &str = "plus";
pub const KEY_MINUS: &str = "minus";
pub const DEFAULT_KEY: &str = "key.keyboard.unknown";

/// The settings of `BHCConfig`, with its defaults.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// `Hurtcam Enabled`.
    pub enabled: bool,
    /// `Hurtcam Multiplier`: any number, shown with two decimals.
    pub multiplier: f64,
    /// `Health Bar Blinking`.
    pub heart_blink: bool,
    /// `Hurtcam Type`: true is Old (no direction change), false is Yaw-based.
    pub old_type: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { enabled: true, multiplier: 0.3, heart_blink: true, old_type: false }
    }
}

fn round(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        Self {
            enabled: flag("enabled", d.enabled),
            // `convert` takes a comma as a point.
            multiplier: get("multiplier").and_then(|v| v.trim().replace(',', ".").parse::<f64>().ok()).filter(|v| v.is_finite()).unwrap_or(d.multiplier),
            heart_blink: flag("heart_blink", d.heart_blink),
            old_type: get("type").map_or(d.old_type, |v| match v.trim() {
                "old" => true,
                "yaw_based" => false,
                _ => d.old_type,
            }),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Hurtcam Enabled: {}", if self.enabled { "ON" } else { "OFF" }),
            format!("Hurtcam Multiplier: {:.2}", self.multiplier),
            format!("Health Bar Blinking: {}", if self.heart_blink { "ON" } else { "OFF" }),
            format!("Hurtcam Type: {}", if self.old_type { "Old" } else { "Yaw-based" }),
        ]
    }

    /// `modifyMultiplier`: no limits.
    pub fn modify(&mut self, by: f64) {
        self.multiplier = round(self.multiplier + by);
    }

    /// The multiplier as the key's overlay message prints it (`%.2f`).
    pub fn multiplier_text(&self) -> String {
        format!("{:.2}", self.multiplier)
    }

    /// A click on the settings page: what to store. The multiplier button adds 0.1 and goes back to 0 after 1.0
    /// (the original types any number; the `multiplier` file takes any too).
    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string()))
            }
            1 => {
                self.multiplier = if self.multiplier >= 0.95 || self.multiplier < -0.05 { 0.0 } else { round(self.multiplier + 0.1) };
                Some(("multiplier", self.multiplier.to_string()))
            }
            2 => {
                self.heart_blink = !self.heart_blink;
                Some(("heart_blink", self.heart_blink.to_string()))
            }
            3 => {
                self.old_type = !self.old_type;
                Some(("type", (if self.old_type { "old" } else { "yaw_based" }).to_owned()))
            }
            _ => None,
        }
    }

    /// What a key does to the settings: the key's name, the file to store and its value, and whether to show the multiplier.
    pub fn key(&mut self, name: &str) -> Option<(&'static str, String, bool)> {
        match name {
            KEY_TOGGLE => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string(), false))
            }
            KEY_PLUS => {
                self.modify(0.1);
                Some(("multiplier", self.multiplier.to_string(), true))
            }
            KEY_MINUS => {
                self.modify(-0.1);
                Some(("multiplier", self.multiplier.to_string(), true))
            }
            _ => None,
        }
    }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    Settings::read(get).click(i)
}
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let s = Settings::read(none);
        assert_eq!((s.enabled, s.multiplier, s.heart_blink, s.old_type), (true, 0.3, true, false));
        assert_eq!(s.multiplier_text(), "0.30");
        assert_eq!(s.labels(), ["Hurtcam Enabled: ON", "Hurtcam Multiplier: 0.30", "Health Bar Blinking: ON", "Hurtcam Type: Yaw-based"]);
    }

    #[test]
    fn the_keys_step_the_multiplier_by_a_tenth_without_limits() {
        let mut s = Settings::default();
        let (file, value, show) = s.key(KEY_PLUS).unwrap();
        assert_eq!((file, value.as_str(), show), ("multiplier", "0.4", true));
        for _ in 0..10 {
            s.key(KEY_PLUS);
        }
        assert_eq!(s.multiplier_text(), "1.40", "past 1");
        for _ in 0..20 {
            s.key(KEY_MINUS);
        }
        assert_eq!(s.multiplier_text(), "-0.60", "below 0");
        let (file, value, show) = s.key(KEY_TOGGLE).unwrap();
        assert_eq!((file, value.as_str(), show), ("enabled", "false", false));
        assert!(s.key("other").is_none());
    }

    #[test]
    fn a_comma_is_a_point_and_junk_falls_back() {
        let comma = |k: &str| (k == "multiplier").then(|| "1,5".to_owned());
        assert_eq!(Settings::read(comma).multiplier, 1.5);
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let nan = |k: &str| (k == "multiplier").then(|| "NaN".to_owned());
        assert_eq!(Settings::read(nan).multiplier, 0.3);
    }

    #[test]
    fn the_buttons_flip_and_the_multiplier_button_cycles_to_zero() {
        let (key, value) = click(none, 3).unwrap();
        assert_eq!((key, value.as_str()), ("type", "old"));
        let (key, value) = click(none, 2).unwrap();
        assert_eq!((key, value.as_str()), ("heart_blink", "false"));
        let (_, value) = click(none, 1).unwrap();
        assert_eq!(value, "0.4");
        let top = |k: &str| (k == "multiplier").then(|| "1".to_owned());
        assert_eq!(click(top, 1).unwrap().1, "0", "after 1.0 it goes back to 0");
        assert!(click(none, 8).is_none());
    }
''',
    helpers={
        "key": '''// One of the three keys went down: the toggle, or the multiplier up or down by a tenth (shown over the hotbar).
fn better_hurt_cam_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_better_hurt_cam as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    for name in [m::KEY_TOGGLE, m::KEY_PLUS, m::KEY_MINUS] {
        if key != game.native_mod_key(m::ID, name, m::DEFAULT_KEY) {
            continue;
        }
        let mut settings = m::Settings::read(|k| game.native_setting(m::ID, k));
        if let Some((file, value, show)) = settings.key(name) {
            game.native_set_setting(m::ID, file, &value);
            if show {
                game.native_overlay_message("Hurtcam multiplier modified to ", &settings.multiplier_text());
            }
        }
        return;
    }
}
''',
        "tick": '''// Once a tick: hand the game the hurt camera these settings give (vanilla when the mod is off).
fn better_hurt_cam_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_better_hurt_cam as m;
    if !game.native_on(m::ID) {
        game.native_hurt_cam_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_hurt_cam(s.enabled, s.multiplier, s.old_type, s.heart_blink);
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ no_death_animation
CAM.append(dict(
    usage="Nothing to press: dead players no longer tip over, they vanish after 22 ticks and the red tint stays 22 ticks. The settings page switches the mod, sets both times, and picks who is affected.",
    slug="no-death-animation", id="no_death_animation", name="No Death Animation", title="No Death Animation", raw=False, needs_api=401,
    description="Removes the death animation tilt on players, and sets how long the red death overlay stays.",
    upstream="No Death Animation (no-death-animation) by Walksy", upstream_licence="MIT",
    command=None, hooks=["list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    runtime_extra=r'''
/// The values the two time buttons cycle through, in ticks. The original takes any 0 to 100; a button can only step, so
/// it steps through these (22, the default, is one of them). The `death_time` and `overlay_time` files take any 0 to 100.
pub const STEPS: [i32; 9] = [0, 5, 10, 15, 20, 22, 40, 60, 100];
/// The `Affected Entities` choices, as stored in the `affected` file (a comma separated list the player can also edit by hand).
pub const AFFECTED: [&str; 3] = ["minecraft:player", "minecraft:player,minecraft:villager", "minecraft:player,minecraft:zombie,minecraft:skeleton,minecraft:creeper"];

/// The settings of the original's General category, with its defaults.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `Mod Enabled`.
    pub enabled: bool,
    /// `Death Time`: ticks before a dead entity disappears (0 to 100).
    pub death_time: i32,
    /// `Death Overlay Time`: ticks the red tint stays after death (0 to 100).
    pub overlay_time: i32,
    /// `Affected Entities`, normalised the way the original does.
    pub affected: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { enabled: true, death_time: 22, overlay_time: 22, affected: vec!["minecraft:player".to_owned()] }
    }
}

/// `getEntityTypeFromString`'s first steps: trim, lower case, spaces to underscores, `minecraft:` when no namespace.
/// None for a blank entry.
pub fn normalise(entry: &str) -> Option<String> {
    let e = entry.trim();
    if e.is_empty() {
        return None;
    }
    let e = e.to_lowercase().replace(' ', "_");
    Some(if e.contains(':') { e } else { format!("minecraft:{e}") })
}

fn clamp(v: i32) -> i32 {
    v.clamp(0, 100)
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let num = |key: &str, d: i32| get(key).and_then(|v| v.trim().parse::<i32>().ok()).map_or(d, clamp);
        let affected: Vec<String> = get("affected").map_or_else(|| d.affected.clone(), |v| v.split(',').filter_map(normalise).collect());
        Self {
            enabled: get("enabled").map_or(d.enabled, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => d.enabled,
            }),
            death_time: num("death_time", d.death_time),
            overlay_time: num("overlay_time", d.overlay_time),
            affected,
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Mod Enabled: {}", if self.enabled { "ON" } else { "OFF" }),
            format!("Death Time: {} ticks", self.death_time),
            format!("Death Overlay Time: {} ticks", self.overlay_time),
            format!("Affected Entities: {}", self.affected.join(", ")),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let step = |v: i32| *STEPS.iter().find(|&&s| s > v).unwrap_or(&STEPS[0]);
        match i {
            0 => {
                self.enabled = !self.enabled;
                Some(("enabled", self.enabled.to_string()))
            }
            1 => {
                self.death_time = step(self.death_time);
                Some(("death_time", self.death_time.to_string()))
            }
            2 => {
                self.overlay_time = step(self.overlay_time);
                Some(("overlay_time", self.overlay_time.to_string()))
            }
            3 => {
                let now = self.affected.join(",");
                let at = AFFECTED.iter().position(|&a| a == now).map_or(0, |p| p + 1);
                let next = AFFECTED[at % AFFECTED.len()];
                self.affected = next.split(',').map(str::to_owned).collect();
                Some(("affected", next.to_owned()))
            }
            _ => None,
        }
    }

    /// `NoDeathAnimation.shouldOverride`: whether entities of `kind` are affected.
    pub fn should_override(&self, kind: &str) -> bool {
        self.enabled && self.affected.iter().any(|a| a == kind)
    }
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
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let s = Settings::read(none);
        assert!(s.enabled);
        assert_eq!((s.death_time, s.overlay_time), (22, 22));
        assert_eq!(s.affected, ["minecraft:player"]);
        assert!(s.should_override("minecraft:player"));
        assert!(!s.should_override("minecraft:zombie"));
    }

    #[test]
    fn disabled_overrides_nothing() {
        let off = |k: &str| (k == "enabled").then(|| "false".to_owned());
        assert!(!Settings::read(off).should_override("minecraft:player"));
    }

    #[test]
    fn entries_are_normalised_like_the_original() {
        assert_eq!(normalise("  Player "), Some("minecraft:player".to_owned()));
        assert_eq!(normalise("Iron Golem"), Some("minecraft:iron_golem".to_owned()));
        assert_eq!(normalise("Mod:Thing"), Some("mod:thing".to_owned()));
        assert_eq!(normalise("   "), None);
        let list = |k: &str| (k == "affected").then(|| "Zombie, ,minecraft:player".to_owned());
        assert_eq!(Settings::read(list).affected, ["minecraft:zombie", "minecraft:player"]);
    }

    #[test]
    fn times_are_clamped_to_zero_to_a_hundred() {
        let big = |k: &str| (k == "death_time" || k == "overlay_time").then(|| "500".to_owned());
        let s = Settings::read(big);
        assert_eq!((s.death_time, s.overlay_time), (100, 100));
        let neg = |k: &str| (k == "death_time").then(|| "-4".to_owned());
        assert_eq!(Settings::read(neg).death_time, 0);
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk).death_time, 22);
    }

    #[test]
    fn the_buttons_step_and_report_what_to_store() {
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("death_time", "40"), "22 steps to 40");
        let at100 = |k: &str| (k == "overlay_time").then(|| "100".to_owned());
        let (_, wrapped) = click(at100, 2).unwrap();
        assert_eq!(wrapped, "0", "100 wraps to 0");
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("enabled", "false"));
        let (key, value) = click(none, 3).unwrap();
        assert_eq!((key, value.as_str()), ("affected", "minecraft:player,minecraft:villager"));
        assert!(click(none, 9).is_none());
        assert_eq!(labels(none)[0], "Mod Enabled: ON");
    }
''',
    helpers={
        "tick": '''// Once a tick: hand the game the death style these settings give (nothing when the mod is off).
fn no_death_animation_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_no_death_animation as m;
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_death_style(game.native_on(m::ID) && s.enabled, &s.affected, s.death_time, s.overlay_time);
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ simple_zoom
CAM.append(dict(
    usage="| Key | Default | What it does |\n|---|---|---|\n| Zoom | `C` | Hold to zoom in (3x). While held, the mouse wheel zooms further (up) or less (down), 1x to 50x; letting go resets it to 3x |\n\nMouse turning is divided by the zoom while it is held, and the wheel does not move the hotbar meanwhile. No settings.",
    slug="simple-zoom", id="simple_zoom", name="Simple Zoom", title="Simple Zoom", raw=False, needs_api=403,
    description="A simple zoom: hold a key, scroll to change the magnification. No unnecessary stuff.",
    upstream="Simple Zoom (zoom-simple) by MrRockis", upstream_licence="MIT",
    command=None, hooks=["key", "wheel", "zoom", "list"],
    options=None,
    keys_expr='vec![[m::KEY.into(), "Zoom".into(), m::DEFAULT_KEY.into()]]',
    runtime_extra=r'''
use std::cell::Cell;

/// `key.simplezoom.toggle`, "Zoom" in the category "Simple Zoom"; the original's default is `C` (its page says so).
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "key.keyboard.c";
/// `getDefaultZoomLevel`.
pub const DEFAULT_ZOOM: f64 = 3.0;
/// `handleMouseScroll`: each wheel event multiplies the zoom by this up or down, between 1 and 50.
pub const UP: f64 = 1.1;
pub const DOWN: f64 = 0.9;
pub const MIN: f64 = 1.0;
pub const MAX: f64 = 50.0;

thread_local! {
    static HELD: Cell<bool> = const { Cell::new(false) };
    static ZOOM: Cell<f64> = const { Cell::new(DEFAULT_ZOOM) };
}

/// The zoom key went down or up. Letting go resets the zoom (`resetZoomLevel`, which the original does whenever the field of view is
/// asked for with the key up).
pub fn key(playing: bool, pressed: bool) {
    if pressed && !playing {
        return;
    }
    HELD.with(|h| h.set(pressed));
    if !pressed {
        ZOOM.with(|z| z.set(DEFAULT_ZOOM));
    }
}

/// A wheel event (up is positive): true while the key is held, when the hotbar must not scroll (`canScrollInventory`).
pub fn wheel(steps: f32) -> bool {
    if !HELD.with(Cell::get) {
        return false;
    }
    ZOOM.with(|z| {
        let mut v = z.get();
        if steps > 0.0 {
            v *= UP;
        } else if steps < 0.0 {
            v *= DOWN;
        }
        z.set(v.clamp(MIN, MAX));
    });
    true
}

/// The zoom to ask the game for while the key is held: the factor (the field of view is divided by it, at once), and the turning scale
/// (mouse movement is divided by the zoom).
pub fn view() -> Option<(f32, f32)> {
    if !HELD.with(Cell::get) {
        return None;
    }
    let z = ZOOM.with(Cell::get);
    Some((z as f32, (1.0 / z) as f32))
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget() {
    HELD.with(|h| h.set(false));
    ZOOM.with(|z| z.set(DEFAULT_ZOOM));
}
''',
    tests=r'''
    #[test]
    fn holding_the_key_zooms_three_times_and_letting_go_stops() {
        forget();
        assert_eq!(view(), None);
        key(true, true);
        let (factor, turn) = view().unwrap();
        assert_eq!(factor, 3.0);
        assert!((turn - 1.0 / 3.0).abs() < 1e-6, "mouse turning is divided by the zoom");
        key(true, false);
        assert_eq!(view(), None);
    }

    #[test]
    fn the_wheel_multiplies_by_1_1_and_0_9_between_1_and_50() {
        forget();
        key(true, true);
        wheel(1.0);
        assert!((view().unwrap().0 - 3.3).abs() < 1e-5);
        wheel(-2.0);
        assert!((view().unwrap().0 - 2.97).abs() < 1e-5, "a step is a step whatever its size");
        for _ in 0..200 {
            wheel(1.0);
        }
        assert_eq!(view().unwrap().0, 50.0);
        for _ in 0..200 {
            wheel(-1.0);
        }
        assert_eq!(view().unwrap().0, 1.0);
        forget();
    }

    #[test]
    fn the_zoom_resets_when_the_key_is_let_go() {
        forget();
        key(true, true);
        wheel(1.0);
        wheel(1.0);
        key(true, false);
        key(true, true);
        assert_eq!(view().unwrap().0, 3.0);
        forget();
    }

    #[test]
    fn the_wheel_only_belongs_to_the_mod_while_the_key_is_held() {
        forget();
        assert!(!wheel(1.0), "the hotbar scrolls as usual");
        key(true, true);
        assert!(wheel(1.0), "the hotbar does not");
        key(true, false);
        assert!(!wheel(1.0));
        forget();
    }

    #[test]
    fn a_press_outside_the_game_does_nothing() {
        forget();
        key(false, true);
        assert_eq!(view(), None);
    }
''',
    helpers={
        "key": '''// The zoom key went down or up: the player's own choice of key.
fn simple_zoom_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_simple_zoom as m;
    if !game.native_on(m::ID) || key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    m::key(game.native_playing(), pressed);
}
''',
        "wheel": '''// While the key is held, the wheel changes the zoom instead of moving the hotbar.
fn simple_zoom_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::lib_simple_zoom as m;
    if game.native_on(m::ID) && m::wheel(steps) {
        *claimed = true;
    }
}
''',
        "zoom": '''// The zoom in effect, unless another mod's came first: instant, like the original.
fn simple_zoom_view(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::lib_simple_zoom as m;
    if zoom.is_none()
        && game.native_on(m::ID)
        && let Some((factor, turn)) = m::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth: false, turn });
    }
}
''',
    },
))

# ------------------------------------------------------------------------------------------------ sprint_indicator
CAM.append(dict(
    usage="Nothing to press: an icon beside the hotbar shows when you sprint (the speed effect's icon) or crouch (the slowness icon). The settings page has Show sprint indicator, Show when sprint key is active, Show crouch indicator, Icon offset and Shift attack indicator.",
    slug="sprint-indicator", id="sprint_indicator", name="Sprint Indicator", title="Sprint Indicator", raw=False, needs_api=405,
    description="Simple UI element showing whether you're currently sprinting, and whether you're crouching too.",
    upstream="Sprint Indicator (sprint-indicator) by Roundaround", upstream_licence="MIT",
    command=None, hooks=["hud", "list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    runtime_extra=r'''
/// The offsets the two Icon offset buttons step through, in GUI pixels (the original takes any whole numbers as `x,y`; the `offset_x`
/// and `offset_y` files do too).
pub const STEPS: [i32; 9] = [-20, -10, -5, -2, 0, 2, 5, 10, 20];
/// `adjustAttackIndicatorX`: how far the hotbar's attack indicator moves out for the icon.
pub const ATTACK_SHIFT: i32 = 24;
/// The icon's size and the alpha of the "armed" sprint icon (`ARMED_ALPHA`).
pub const SIZE: i32 = 18;
pub const ARMED_ALPHA: f32 = 0.7;

/// The settings of `SprintIndicatorConfig`, with its defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `sprintEnabled`: show an icon when sprinting.
    pub sprint: bool,
    /// `sprintShowWhenActive`: also show the sprint icon, dimmed, while the sprint key is active but you are not sprinting.
    pub show_when_active: bool,
    /// `crouchEnabled`: show an icon when crouching.
    pub crouch: bool,
    /// `offset`: x and y.
    pub offset: (i32, i32),
    /// `adjustAttackIndicator`: shift the hotbar's attack indicator out of the icon's way.
    pub adjust_attack_indicator: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { sprint: true, show_when_active: false, crouch: true, offset: (0, 0), adjust_attack_indicator: true }
    }
}

/// What to draw, from `HudMixin.afterRenderHotbar`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Icon {
    /// True for the slowness icon (crouching), false for the speed icon.
    pub crouch: bool,
    pub alpha: f32,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        let num = |key: &str, d: i32| get(key).and_then(|v| v.trim().parse::<i32>().ok()).unwrap_or(d);
        Self {
            sprint: flag("sprint", d.sprint),
            show_when_active: flag("show_when_active", d.show_when_active),
            crouch: flag("crouch", d.crouch),
            offset: (num("offset_x", d.offset.0), num("offset_y", d.offset.1)),
            adjust_attack_indicator: flag("adjust_attack_indicator", d.adjust_attack_indicator),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Show sprint indicator: {}", on(self.sprint)),
            format!("Show when sprint key is active: {}", on(self.show_when_active)),
            format!("Show crouch indicator: {}", on(self.crouch)),
            format!("Icon offset X: {}", self.offset.0),
            format!("Icon offset Y: {}", self.offset.1),
            format!("Shift attack indicator: {}", on(self.adjust_attack_indicator)),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let step = |v: i32| *STEPS.iter().find(|&&s| s > v).unwrap_or(&STEPS[0]);
        match i {
            0 => {
                self.sprint = !self.sprint;
                Some(("sprint", self.sprint.to_string()))
            }
            1 => {
                self.show_when_active = !self.show_when_active;
                Some(("show_when_active", self.show_when_active.to_string()))
            }
            2 => {
                self.crouch = !self.crouch;
                Some(("crouch", self.crouch.to_string()))
            }
            3 => {
                self.offset.0 = step(self.offset.0);
                Some(("offset_x", self.offset.0.to_string()))
            }
            4 => {
                self.offset.1 = step(self.offset.1);
                Some(("offset_y", self.offset.1.to_string()))
            }
            5 => {
                self.adjust_attack_indicator = !self.adjust_attack_indicator;
                Some(("adjust_attack_indicator", self.adjust_attack_indicator.to_string()))
            }
            _ => None,
        }
    }

    /// The icon to draw, if any: `sprinting` is `isSprinting`, `crouching` is `isCrouching`, `sprint_key` is the sprint key being active.
    /// Crouching wins and is drawn in full; sprinting is full; an armed sprint key is dimmed.
    pub fn icon(&self, sprinting: bool, crouching: bool, sprint_key: bool) -> Option<Icon> {
        let sprint = self.sprint && sprinting;
        let armed = !sprint && self.sprint && self.show_when_active && sprint_key;
        let crouch = self.crouch && crouching;
        if !sprint && !armed && !crouch {
            return None;
        }
        Some(Icon { crouch, alpha: if crouch || sprint { 1.0 } else { ARMED_ALPHA } })
    }

    /// Where the icon goes, from the screen's size in GUI pixels: beside the hotbar on the off hand's side, 6 pixels out.
    pub fn position(&self, width: i32, height: i32, main_hand_left: bool) -> (i32, i32) {
        let x = if main_hand_left { (width - 182) / 2 - 18 - 6 } else { (width + 182) / 2 + 6 };
        (x + self.offset.0, height - 20 + self.offset.1)
    }

    /// How far to push the hotbar's attack indicator out (0 when not adjusting).
    pub fn attack_shift(&self) -> i32 {
        if self.adjust_attack_indicator && (self.sprint || self.crouch) { ATTACK_SHIFT } else { 0 }
    }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    Settings::read(get).click(i)
}
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let s = Settings::read(none);
        assert_eq!((s.sprint, s.show_when_active, s.crouch), (true, false, true));
        assert_eq!((s.offset, s.adjust_attack_indicator), ((0, 0), true));
    }

    #[test]
    fn sprinting_and_crouching_show_in_full_and_crouching_wins() {
        let s = Settings::default();
        assert_eq!(s.icon(true, false, false), Some(Icon { crouch: false, alpha: 1.0 }));
        assert_eq!(s.icon(false, true, false), Some(Icon { crouch: true, alpha: 1.0 }));
        assert_eq!(s.icon(true, true, false), Some(Icon { crouch: true, alpha: 1.0 }), "the slowness icon");
        assert_eq!(s.icon(false, false, true), None, "a held sprint key alone shows nothing by default");
    }

    #[test]
    fn an_armed_sprint_key_is_dimmed_when_asked() {
        let s = Settings { show_when_active: true, ..Settings::default() };
        assert_eq!(s.icon(false, false, true), Some(Icon { crouch: false, alpha: 0.7 }));
        assert_eq!(s.icon(true, false, true).unwrap().alpha, 1.0, "sprinting is not dimmed");
        let off = Settings { sprint: false, show_when_active: true, ..Settings::default() };
        assert_eq!(off.icon(false, false, true), None);
        assert_eq!(off.icon(true, false, false), None);
        let no_crouch = Settings { crouch: false, ..Settings::default() };
        assert_eq!(no_crouch.icon(false, true, false), None);
    }

    #[test]
    fn the_icon_sits_beside_the_hotbar_on_the_off_hand_side_plus_the_offset() {
        let s = Settings::default();
        assert_eq!(s.position(320, 240, false), (251 + 6, 220), "right handed: right of the hotbar");
        assert_eq!(s.position(320, 240, true), (69 - 18 - 6, 220), "left handed: left of it");
        let moved = Settings { offset: (3, -4), ..s };
        assert_eq!(moved.position(320, 240, false), (260, 216));
    }

    #[test]
    fn the_attack_indicator_moves_out_only_when_adjusting_and_an_icon_is_on() {
        assert_eq!(Settings::default().attack_shift(), 24);
        assert_eq!(Settings { adjust_attack_indicator: false, ..Settings::default() }.attack_shift(), 0);
        assert_eq!(Settings { sprint: false, crouch: false, ..Settings::default() }.attack_shift(), 0);
    }

    #[test]
    fn the_buttons_flip_and_the_offsets_step_and_wrap() {
        assert_eq!(click(none, 0).unwrap(), ("sprint", "false".to_owned()));
        assert_eq!(click(none, 3).unwrap(), ("offset_x", "2".to_owned()));
        let top = |k: &str| (k == "offset_y").then(|| "20".to_owned());
        assert_eq!(click(top, 4).unwrap().1, "-20");
        assert!(click(none, 9).is_none());
        assert_eq!(labels(none).len(), 6);
    }
''',
    helpers={
        "hud": '''// The speed or slowness icon beside the hotbar, after the hotbar is drawn.
fn sprint_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_sprint_indicator as m;
    use horizon_client::hud_layout::{self, Blend, GuiDraw};
    let _ = &corners;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let Some(motion) = game.native_local_motion() else {
        return;
    };
    let settings = m::Settings::read(|k| game.native_setting(m::ID, k));
    let Some(icon) = settings.icon(motion.sprinting, motion.crouching, motion.sprint_key) else {
        return;
    };
    let (x, y) = settings.position(batch.width as i32, batch.height as i32, motion.main_hand_left);
    let draw = GuiDraw::Sprite {
        sprite: std::borrow::Cow::Borrowed(if icon.crouch { "mob_effect/slowness" } else { "mob_effect/speed" }),
        x,
        y,
        w: m::SIZE,
        h: m::SIZE,
        region: None,
        color: hud_layout::white(icon.alpha),
        blend: Blend::Textured,
    };
    hud_layout::paint(batch, &game.gui.atlas, &[draw]);
}
''',
        "tick": '''// Once a tick: move the hotbar's attack indicator out of the icon's way (put back when the mod is off).
fn sprint_indicator_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_sprint_indicator as m;
    let shift = if game.native_on(m::ID) { m::Settings::read(|k| game.native_setting(m::ID, k)).attack_shift() } else { 0 };
    // The shift goes outwards from the hotbar on whichever side the attack indicator is on, as the game places it.
    game.native_attack_indicator_shift(shift);
}
''',
    },
))
