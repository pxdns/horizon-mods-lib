// Crystal Optimizer, a native Horizon mod: Makes End crystals you hit disappear at once on a server, and brings one back if the server did not break it.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_crystal_optimizer`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "crystal_optimizer";
pub const NAME: &str = "Crystal Optimizer";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Makes End crystals you hit disappear at once on a server, and brings one back if the server did not break it.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("hide", "Hide a hit crystal at once", true)
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

use std::any::Any;

/// A hit crystal comes back if the server has not removed it after this many ticks (a second and a half).
pub const RESTORE_TICKS: u64 = 30;

#[derive(Default)]
struct State {
    /// Crystals the player hit that are still in the game's list: take them out at the next tick.
    asked: Vec<u32>,
    /// Crystals taken out, with the tick they went and the crystal itself.
    kept: Vec<(u32, u64, Box<dyn Any>)>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The player attacked crystal `id`.
pub fn hit(id: u32) {
    STATE.with_borrow_mut(|s| {
        if !s.asked.contains(&id) && !s.kept.iter().any(|(k, ..)| *k == id) {
            s.asked.push(id);
        }
    });
}

/// The crystals to take out now.
pub fn take_requests() -> Vec<u32> {
    STATE.with_borrow_mut(|s| std::mem::take(&mut s.asked))
}

/// A crystal was taken out at `tick`; the mod holds it until the server answers.
pub fn keep(id: u32, tick: u64, crystal: Box<dyn Any>) {
    STATE.with_borrow_mut(|s| s.kept.push((id, tick, crystal)));
}

/// The server removed crystal `id` (or never needed it): forget it.
pub fn confirmed(id: u32) {
    STATE.with_borrow_mut(|s| {
        s.asked.retain(|k| *k != id);
        s.kept.retain(|(k, ..)| *k != id);
    });
}

/// The crystals the server did not remove in time, to put back.
pub fn expired(now: u64) -> Vec<Box<dyn Any>> {
    STATE.with_borrow_mut(|s| {
        let (old, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut s.kept).into_iter().partition(|(_, at, _)| now.saturating_sub(*at) >= RESTORE_TICKS);
        s.kept = rest;
        old.into_iter().map(|(.., c)| c).collect()
    })
}

/// No world: forget everything.
pub fn reset() {
    STATE.with_borrow_mut(|s| *s = State::default());
}

/// How many are being held (tests).
#[cfg(test)]
pub fn held() -> usize {
    STATE.with_borrow(|s| s.kept.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hit_is_asked_once_and_taken_once() {
        reset();
        hit(5);
        hit(5);
        assert_eq!(take_requests(), vec![5]);
        assert!(take_requests().is_empty());
    }

    #[test]
    fn a_held_crystal_is_not_asked_again() {
        reset();
        keep(7, 100, Box::new("crystal"));
        hit(7);
        assert!(take_requests().is_empty());
        assert_eq!(held(), 1);
        reset();
    }

    #[test]
    fn the_server_removing_it_forgets_it() {
        reset();
        keep(7, 100, Box::new(1));
        confirmed(7);
        assert_eq!(held(), 0);
        assert!(expired(1000).is_empty(), "nothing comes back after the server removed it");
        hit(8);
        confirmed(8);
        assert!(take_requests().is_empty());
    }

    #[test]
    fn an_unanswered_crystal_comes_back_after_a_second_and_a_half() {
        reset();
        keep(7, 100, Box::new(String::from("seven")));
        keep(8, 120, Box::new(String::from("eight")));
        assert!(expired(129).is_empty());
        let back = expired(130);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].downcast_ref::<String>().map(String::as_str), Some("seven"));
        assert_eq!(held(), 1, "the later one is still waiting");
        assert_eq!(expired(150).len(), 1);
        reset();
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
