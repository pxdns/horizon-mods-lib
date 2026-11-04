// Clumps, a native Horizon mod: Groups nearby experience orbs into a single orb, which cuts the entity count and is collected at once.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_clumps`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "clumps";
pub const NAME: &str = "Clumps";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Groups nearby experience orbs into a single orb, which cuts the entity count and is collected at once.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[("sizes", "Merge orbs of different sizes", true)];

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

/// An orb is looked at on this many ticks out of that many (once a quarter second).
pub const EVERY: u32 = 5;

thread_local! {
    static SIZES: Cell<bool> = const { Cell::new(true) };
}

/// Sets whether different sizes merge, from the player's settings (the game tells the mod each tick); off when the mod is.
pub fn configure(sizes: bool) {
    SIZES.with(|v| v.set(sizes));
}

pub fn on() -> bool {
    SIZES.with(Cell::get)
}

/// Which orbs absorb which: `ticks[i]` is orb i's age in ticks, `touches(i, j)` whether their boxes (grown a little) overlap.
/// Returns (absorber, absorbed orbs); an orb absorbed once is not looked at again, and an absorber has been looked at on its turn.
pub fn plan(ticks: &[u32], touches: impl Fn(usize, usize) -> bool) -> Vec<(usize, Vec<usize>)> {
    let mut gone = vec![false; ticks.len()];
    let mut out = Vec::new();
    for i in 0..ticks.len() {
        if gone[i] || ticks[i] % EVERY != 1 {
            continue;
        }
        let mut taken = Vec::new();
        for (j, absorbed) in gone.iter_mut().enumerate() {
            if j != i && !*absorbed && touches(i, j) {
                *absorbed = true;
                taken.push(j);
            }
        }
        if !taken.is_empty() {
            out.push((i, taken));
        }
    }
    out
}

/// The experience of an orb that stands for `count` pickups of `value`.
pub fn worth(value: u32, count: u32) -> u32 {
    value.saturating_mul(count.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbs_are_looked_at_on_their_turn_only() {
        // Orb 0 is on its turn (tick 1), orb 1 is not (tick 2).
        let touching = |_: usize, _: usize| true;
        assert_eq!(plan(&[1, 2], touching), vec![(0, vec![1])]);
        assert!(plan(&[2, 3], touching).is_empty());
    }

    #[test]
    fn an_absorbed_orb_is_not_an_absorber() {
        // All three are on their turn and touch each other: 0 takes both.
        assert_eq!(plan(&[1, 6, 11], |_, _| true), vec![(0, vec![1, 2])]);
    }

    #[test]
    fn only_touching_orbs_merge() {
        let near = |i: usize, j: usize| (i, j) == (0, 2) || (i, j) == (2, 0);
        assert_eq!(plan(&[1, 1, 1], near), vec![(0, vec![2])]);
        assert!(plan(&[1, 1], |_, _| false).is_empty());
    }

    #[test]
    fn worth_multiplies_and_never_overflows() {
        assert_eq!(worth(7, 3), 21);
        assert_eq!(worth(7, 0), 7);
        assert_eq!(worth(u32::MAX, 2), u32::MAX);
    }

    #[test]
    fn the_switch_works() {
        configure(false);
        assert!(!on());
        configure(true);
        assert!(on());
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
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
