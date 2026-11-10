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
    usage="Nothing to press: hold A and D (or W and S) together and the key you pressed last decides the direction; let go of it and the other carries on.", slug="snappy-tappy", id="snappy_tappy", name="Snappy Tappy", title="Snappy Tappy", raw=True,
    description="When you hold two opposite movement keys, the one pressed last wins, so you switch direction at once.",
    upstream="SnappyTappy (SnappyTappy)", upstream_licence="see the upstream project",
    command=None, hooks=["key", "tick", "list", "settings", "click"],
    options=[("sideways", "A and D", True), ("forwards", "W and S", True)],
    raw_note=("It edits the `Game`'s held-key set before movement is read, which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. "
              "It assumes the default W A S D movement keys."),
    runtime_extra=r'''
use std::cell::Cell;

/// Which key of a pair went down last.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Last {
    #[default]
    None,
    First,
    Second,
}

thread_local! {
    static SIDEWAYS: Cell<Last> = const { Cell::new(Last::None) };
    static FORWARDS: Cell<Last> = const { Cell::new(Last::None) };
}

/// A movement key went down or up, by its vanilla name (`key.keyboard.a`).
pub fn pressed(key: &str, down: bool) {
    let (cell, which) = match key {
        "key.keyboard.a" => (&SIDEWAYS, Last::First),
        "key.keyboard.d" => (&SIDEWAYS, Last::Second),
        "key.keyboard.w" => (&FORWARDS, Last::First),
        "key.keyboard.s" => (&FORWARDS, Last::Second),
        _ => return,
    };
    cell.with(|c| {
        if down {
            c.set(which);
        } else if c.get() == which {
            c.set(Last::None);
        }
    });
}

/// The last-pressed state of the A/D pair and the W/S pair.
pub fn last() -> (Last, Last) {
    (SIDEWAYS.with(Cell::get), FORWARDS.with(Cell::get))
}

/// Which of a pair counts as held: both held means the last pressed one only.
pub fn resolve(first: bool, second: bool, last: Last) -> (bool, bool) {
    match (first, second) {
        (true, true) => match last {
            Last::Second => (false, true),
            _ => (true, false),
        },
        other => other,
    }
}

/// Forgets what was pressed (tests only).
#[cfg(test)]
pub fn forget_keys() {
    SIDEWAYS.with(|c| c.set(Last::None));
    FORWARDS.with(|c| c.set(Last::None));
}
''',
    tests=r'''
    #[test]
    fn the_last_pressed_of_two_held_keys_wins() {
        assert_eq!(resolve(true, true, Last::First), (true, false));
        assert_eq!(resolve(true, true, Last::Second), (false, true));
        assert_eq!(resolve(true, true, Last::None), (true, false), "unknown order: the first key");
        assert_eq!(resolve(true, false, Last::Second), (true, false), "one key held is just held");
        assert_eq!(resolve(false, false, Last::First), (false, false));
    }

    #[test]
    fn presses_and_releases_are_tracked_per_pair() {
        forget_keys();
        pressed("key.keyboard.a", true);
        pressed("key.keyboard.d", true);
        assert_eq!(last(), (Last::Second, Last::None));
        pressed("key.keyboard.d", false);
        assert_eq!(last().0, Last::None, "releasing the newest leaves the older one to carry on");
        pressed("key.keyboard.w", true);
        pressed("key.keyboard.s", true);
        pressed("key.keyboard.a", true);
        assert_eq!(last(), (Last::First, Last::Second));
        pressed("key.keyboard.space", true);
        assert_eq!(last(), (Last::First, Last::Second), "other keys do not matter");
        forget_keys();
    }
''',
    helpers={
        "key": '''// A key went down or up: remember which of a pair was pressed last.
fn snappy_tappy_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_snappy_tappy as m;
    if game.native_on(m::ID) {
        m::pressed(key, pressed);
    }
}
''',
        "tick": '''// Once a tick, before movement is read: of two opposite keys held together, only the last pressed counts.
fn snappy_tappy_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_snappy_tappy as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) || !game.native_window() || !game.native_playing() {
        return;
    }
    let (sideways, forwards) = {
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "sideways"), m::is(get, "forwards"))
    };
    let (last_sideways, last_forwards) = m::last();
    let held = |k: K| game.physical_keys.contains(&k);
    let pairs = [
        (sideways, K::KeyA, K::KeyD, held(K::KeyA), held(K::KeyD), last_sideways),
        (forwards, K::KeyW, K::KeyS, held(K::KeyW), held(K::KeyS), last_forwards),
    ];
    for (on, first, second, first_held, second_held, last) in pairs {
        if !on {
            continue;
        }
        let (a, b) = m::resolve(first_held, second_held, last);
        for (key, want) in [(first, a), (second, b)] {
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
    usage="Nothing to press: when you take damage the camera shakes at a fraction of Horizon's own strength (the first button cycles 0, 25, 50, 75, 100 %), and the roll over when you die can be switched off.", slug="better-hurt-cam", id="better_hurt_cam", name="Better Hurt Cam", title="Better Hurt Cam", raw=True,
    description="Tone the camera shake down when you are hit, and switch the death tilt off.",
    upstream="BetterHurtCam (betterhurtcam), and the death-tilt part of No Death Animation", upstream_licence="see the upstream projects",
    command=None, hooks=["bob", "list", "settings", "click"],
    options=None, custom_settings=True,
    raw_note=("It replaces Horizon's camera-bob function (`Game::bob_matrix`) with a cancellable HEAD hook that rebuilds the same matrix with a weaker hurt tilt, "
              "which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    runtime_extra=r'''
use std::cell::Cell;

/// The tilt strengths the first button cycles through, as a percentage of Horizon's own.
pub const LEVELS: [u32; 5] = [0, 25, 50, 75, 100];

/// The settings, one stored file per key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// The hurt tilt, a percentage of the game's setting; one of `LEVELS`.
    pub tilt: u32,
    /// Whether the camera still rolls over when you die.
    pub death: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { tilt: 50, death: false }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        Self {
            tilt: get("tilt").and_then(|v| v.trim().parse().ok()).filter(|v| LEVELS.contains(v)).unwrap_or(d.tilt),
            death: get("death").map_or(d.death, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => d.death,
            }),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Hit shake: {}%", self.tilt),
            format!("Death tilt: {}", if self.death { "ON" } else { "OFF" }),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                let at = LEVELS.iter().position(|&l| l == self.tilt).unwrap_or(0);
                self.tilt = LEVELS[(at + 1) % LEVELS.len()];
                Some(("tilt", self.tilt.to_string()))
            }
            1 => {
                self.death = !self.death;
                Some(("death", self.death.to_string()))
            }
            _ => None,
        }
    }

    /// The factor to multiply the game's own damage tilt by.
    pub fn factor(&self) -> f64 {
        f64::from(self.tilt) / 100.0
    }

    /// Whether the game's own matrix is exactly what these settings give, so the original can run.
    pub fn is_vanilla(&self) -> bool {
        self.tilt == 100 && self.death
    }
}

thread_local! {
    static SETTINGS: Cell<Option<Settings>> = const { Cell::new(None) };
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|s| {
        let v = s.get().unwrap_or_else(|| Settings::read(get));
        s.set(Some(v));
        v
    })
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    settings(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = settings(get);
    let stored = now.click(i);
    SETTINGS.with(|s| s.set(Some(now)));
    stored
}

/// Forgets the settings (tests only).
#[cfg(test)]
pub fn forget() {
    SETTINGS.with(|s| s.set(None));
}
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_halve_the_shake_and_drop_the_death_tilt() {
        forget();
        let s = settings(none);
        assert_eq!((s.tilt, s.death), (50, false));
        assert_eq!(s.factor(), 0.5);
        assert!(!s.is_vanilla());
        forget();
    }

    #[test]
    fn the_shake_button_cycles_and_reports_what_to_store() {
        forget();
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("tilt", "75"));
        click(none, 0);
        let (_, wrapped) = click(none, 0).unwrap();
        assert_eq!(wrapped, "0", "100 wraps to 0");
        assert_eq!(settings(none).factor(), 0.0);
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("death", "true"));
        assert_eq!(labels(none), ["Hit shake: 0%", "Death tilt: ON"]);
        assert!(click(none, 7).is_none());
        forget();
    }

    #[test]
    fn vanilla_settings_let_the_original_run() {
        assert!(Settings { tilt: 100, death: true }.is_vanilla());
        assert!(!Settings { tilt: 100, death: false }.is_vanilla());
    }

    #[test]
    fn stored_junk_falls_back() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let stored = |k: &str| match k {
            "tilt" => Some("25".to_owned()),
            "death" => Some("true".to_owned()),
            _ => None,
        };
        assert_eq!(Settings::read(stored), Settings { tilt: 25, death: true });
        let off_list = |_: &str| Some("30".to_owned());
        assert_eq!(Settings::read(off_list).tilt, 50, "30 is not one of the levels");
    }
''',
    helpers={
        "bob": '''// The camera's hurt and bob pose: Horizon's own, rebuilt with the hurt tilt scaled and the death tilt left out when asked.
fn better_hurt_cam_bob(game: &crate::game::Game<'_>, partial: f32) -> std::ops::ControlFlow<glam::Mat4> {
    use crate::game::lib_better_hurt_cam as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    let settings = m::settings(|k| game.native_setting(m::ID, k));
    if settings.is_vanilla() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(p) = game.player.as_ref() else {
        return std::ops::ControlFlow::Continue(());
    };
    let hurt = &p.health.hurt;
    let state = BobState {
        living: Some(HurtBob {
            hurt_time: p.health.hurt_animation_time as f32 - partial,
            hurt_duration: hurt.hurt_duration,
            hurt_dir: p.health.hurt_dir,
            death_time: if settings.death { game.death_time().map(|t| t as f32 + partial) } else { None },
        }),
        player: Some((
            game.avatar.backwards_interpolated_walk_distance(partial),
            game.avatar.interpolated_bob(partial),
        )),
    };
    let tilt = game.settings.file.double("damageTiltStrength") * settings.factor();
    std::ops::ControlFlow::Break(glam::Mat4::from_cols_array(&bob_matrix(&state, tilt, game.settings.view_bobbing).m))
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
