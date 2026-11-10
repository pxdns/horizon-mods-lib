// Pearl Optimizer, a native Horizon mod: Shows your thrown ender pearl immediately on a server, before the server's pearl arrives.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_pearl_optimizer`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "pearl_optimizer";
pub const NAME: &str = "Pearl Optimizer";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows your thrown ender pearl immediately on a server, before the server's pearl arrives.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("fast", "Show my pearl at once", true)
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

use std::cell::RefCell as StateCell;

/// Local pearls get ids from here up, far from any id a server hands out.
pub const BASE_ID: u32 = 0x7F00_0000;
/// A local pearl is removed after this many ticks (two seconds) if the server's has not come.
pub const LIFE_TICKS: u64 = 40;
/// The speed of a thrown pearl (`ThrowableItemProjectile.shootFromRotation` with power 1.5).
pub const SPEED: f64 = 1.5;

/// A throw that left for the server: where the pearl starts, how fast it goes, and which way it faces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Throw {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Default)]
struct State {
    throws: Vec<Throw>,
    /// The server's pearls that arrived and have not been matched to a local one yet.
    arrived: u32,
    /// Local pearls: id and the tick they were added.
    local: Vec<(u32, u64)>,
    next: u32,
}

thread_local! {
    static STATE: StateCell<State> = StateCell::new(State::default());
}

/// The unit vector a view with this yaw and pitch (degrees) looks along.
pub fn look(yaw: f32, pitch: f32) -> [f64; 3] {
    let (y, p) = (f64::from(yaw).to_radians(), f64::from(pitch).to_radians());
    [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()]
}

/// The throw for a player with eyes at `eye`, looking `yaw`/`pitch`, moving at `motion` (its vertical part only when not on the ground).
pub fn throw(eye: [f64; 3], yaw: f32, pitch: f32, motion: [f64; 3], on_ground: bool) -> Throw {
    let dir = look(yaw, pitch);
    Throw {
        position: [eye[0], eye[1] - 0.1, eye[2]],
        velocity: [dir[0] * SPEED + motion[0], dir[1] * SPEED + if on_ground { 0.0 } else { motion[1] }, dir[2] * SPEED + motion[2]],
        yaw,
        pitch,
    }
}

/// A pearl was thrown.
pub fn thrown(t: Throw) {
    STATE.with_borrow_mut(|s| s.throws.push(t));
}

/// The server added a pearl.
pub fn server_pearl() {
    STATE.with_borrow_mut(|s| s.arrived += 1);
}

/// One tick: the throws to add as local pearls with their new ids, and the local pearls to remove (one per server pearl that
/// arrived, and any that outlived `LIFE_TICKS`).
pub fn step(now: u64) -> (Vec<(u32, Throw)>, Vec<u32>) {
    STATE.with_borrow_mut(|s| {
        let mut remove = Vec::new();
        while s.arrived > 0 && !s.local.is_empty() {
            s.arrived -= 1;
            remove.push(s.local.remove(0).0);
        }
        s.arrived = 0;
        let (old, keep): (Vec<_>, Vec<_>) = std::mem::take(&mut s.local).into_iter().partition(|(_, at)| now.saturating_sub(*at) >= LIFE_TICKS);
        s.local = keep;
        remove.extend(old.into_iter().map(|(id, _)| id));
        let mut add = Vec::new();
        for t in std::mem::take(&mut s.throws) {
            s.next += 1;
            let id = BASE_ID + s.next;
            s.local.push((id, now));
            add.push((id, t));
        }
        (add, remove)
    })
}

/// No world or the mod is off: forget everything.
pub fn reset() {
    STATE.with_borrow_mut(|s| *s = State::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: [f64; 3], b: [f64; 3]) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-9)
    }

    #[test]
    fn the_look_vector_follows_yaw_and_pitch() {
        assert!(near(look(0.0, 0.0), [0.0, 0.0, 1.0]), "yaw 0 faces south");
        assert!(near(look(90.0, 0.0), [-1.0, 0.0, 0.0]), "yaw 90 faces west");
        assert!(near(look(0.0, -90.0), [0.0, 1.0, 0.0]), "pitch -90 looks up");
    }

    #[test]
    fn a_throw_starts_below_the_eyes_and_adds_the_players_motion() {
        let t = throw([10.0, 65.62, -3.0], 0.0, 0.0, [0.1, -0.2, 0.05], true);
        assert!(near(t.position, [10.0, 65.52, -3.0]));
        assert!(near(t.velocity, [0.1, 0.0, 1.5 + 0.05]), "no vertical motion on the ground");
        let air = throw([0.0, 0.0, 0.0], 0.0, 0.0, [0.0, -0.4, 0.0], false);
        assert!((air.velocity[1] + 0.4).abs() < 1e-9, "falling adds to the throw");
    }

    #[test]
    fn a_throw_becomes_a_local_pearl_and_the_servers_replaces_it() {
        reset();
        let t = throw([0.0, 1.0, 0.0], 0.0, 0.0, [0.0; 3], true);
        thrown(t);
        let (add, remove) = step(100);
        assert_eq!(add.len(), 1);
        assert!(add[0].0 > BASE_ID && remove.is_empty());
        server_pearl();
        let (add2, remove2) = step(101);
        assert!(add2.is_empty());
        assert_eq!(remove2, vec![add[0].0], "the server's pearl takes the local one's place");
        assert!(step(102).1.is_empty());
    }

    #[test]
    fn a_pearl_the_server_never_sends_is_removed_after_two_seconds() {
        reset();
        thrown(throw([0.0; 3], 0.0, 0.0, [0.0; 3], true));
        let (add, _) = step(100);
        assert!(step(139).1.is_empty());
        assert_eq!(step(140).1, vec![add[0].0]);
    }

    #[test]
    fn a_server_pearl_with_no_local_one_does_nothing() {
        reset();
        server_pearl();
        let (add, remove) = step(5);
        assert!(add.is_empty() && remove.is_empty());
        thrown(throw([0.0; 3], 0.0, 0.0, [0.0; 3], true));
        let (add, remove) = step(6);
        assert_eq!((add.len(), remove.len()), (1, 0), "an old arrival does not cancel a new throw");
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
