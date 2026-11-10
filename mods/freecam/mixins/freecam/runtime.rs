// Freecam, a native Horizon mod: Detach the camera and fly it around while your player stands still.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_freecam`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "freecam";
pub const NAME: &str = "Freecam";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Detach the camera and fly it around while your player stands still.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("fast", "Fast flight (3x)", false)
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(
            labels(none).len(),
            OPTIONS.len(),
            "one button per option"
        );
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
