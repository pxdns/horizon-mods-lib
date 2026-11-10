// BetterGrassify, a native Horizon mod: Connected grass: a grass block's side shows the grass top where the neighbouring step down is grass too.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_bettergrassify`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "bettergrassify";
pub const NAME: &str = "BetterGrassify";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Connected grass: a grass block's side shows the grass top where the neighbouring step down is grass too.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("fancy", "Better grass on grass block sides", true)
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

use std::cell::{Cell, RefCell as StateCell};
use std::collections::BTreeMap;

/// What the lighting hook notes for the face being drawn: the cell (x, y, z), the face's direction and the block state id.
pub type Pending = (i32, i32, i32, u8, u32);

thread_local! {
    static ON: Cell<bool> = const { Cell::new(true) };
    /// Set by the lighting hook for a grass side with grass one step down, taken by the quad hook that follows.
    static PENDING: Cell<Option<Pending>> = const { Cell::new(None) };
    /// Whether the quad hook is calling the original `quad` itself, which it lets through.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
    /// Per block state id: the grass top texture layer, or none when the model has no up face.
    static TOP: StateCell<BTreeMap<u32, Option<u32>>> = const { StateCell::new(BTreeMap::new()) };
}

/// Sets whether the mesher uses Better Grass (the game tells it each tick).
pub fn configure(on: bool) {
    ON.with(|v| v.set(on));
}

pub fn on() -> bool {
    ON.with(Cell::get)
}

/// Notes (or clears) what the face about to be drawn is.
pub fn set_pending(note: Option<Pending>) {
    PENDING.with(|c| c.set(note));
}

/// The note for the quad being drawn, once.
pub fn take_pending() -> Option<Pending> {
    PENDING.with(Cell::take)
}

/// The quad hook is about to call the original itself.
pub fn enter() {
    INSIDE.with(|c| c.set(true));
}

pub fn leave() {
    INSIDE.with(|c| c.set(false));
}

pub fn inside() -> bool {
    INSIDE.with(Cell::get)
}

/// The remembered top layer of a state: nothing remembered yet, or Some(None) when the model has no up face.
pub fn cached(id: u32) -> Option<Option<u32>> {
    TOP.with_borrow(|t| t.get(&id).copied())
}

pub fn remember(id: u32, layer: Option<u32>) {
    TOP.with_borrow_mut(|t| {
        t.insert(id, layer);
    });
}

/// Forgets the layers (a resource reload changes them).
pub fn forget() {
    TOP.with_borrow_mut(BTreeMap::clear);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_switch_works() {
        configure(false);
        assert!(!on());
        configure(true);
        assert!(on());
    }

    #[test]
    fn a_note_is_taken_once() {
        set_pending(Some((1, 2, 3, 4, 5)));
        assert_eq!(take_pending(), Some((1, 2, 3, 4, 5)));
        assert_eq!(take_pending(), None, "a quad must not reuse the note of the one before");
        set_pending(Some((0, 0, 0, 0, 1)));
        set_pending(None);
        assert_eq!(take_pending(), None, "the next face clears a stale note");
    }

    #[test]
    fn the_guard_lets_the_originals_own_call_through() {
        assert!(!inside());
        enter();
        assert!(inside());
        leave();
        assert!(!inside());
    }

    #[test]
    fn a_states_top_layer_is_remembered() {
        forget();
        assert_eq!(cached(9), None);
        remember(9, Some(42));
        assert_eq!(cached(9), Some(Some(42)));
        remember(10, None);
        assert_eq!(cached(10), Some(None), "a state without an up face is remembered too");
        forget();
        assert_eq!(cached(9), None);
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
