// Pick Up Notifier, a native Horizon mod: +3 Cobblestone: a short list of what you just picked up (and, if you like, dropped).
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_pick_up_notifier`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "pick_up_notifier";
pub const NAME: &str = "Pick Up Notifier";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "+3 Cobblestone: a short list of what you just picked up (and, if you like, dropped).";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] =
    &[("shown", "Shown", true), ("losses", "Show losses", false)];

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
/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;

/// `minecraft:diamond_helmet` as `Diamond Helmet`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

use std::collections::BTreeMap;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "pickups";
/// How long a line stays, in ticks (3 seconds).
pub const LIFE: u32 = 60;
/// At most this many lines at once.
pub const MAX_LINES: usize = 6;

#[derive(Default)]
struct Notes {
    /// What was carried at the last tick, by item id; None until a world is seen.
    last: Option<BTreeMap<String, i64>>,
    /// (item id, change, ticks left), newest last.
    lines: Vec<(String, i64, u32)>,
}

thread_local! {
    static NOTES: RefCell<Notes> = RefCell::new(Notes::default());
}

/// One game tick with what the player carries now (item id, count) pairs; `None` outside a world.
pub fn tick(carried: Option<Vec<(String, u32)>>) {
    NOTES.with_borrow_mut(|n| {
        for line in &mut n.lines {
            line.2 = line.2.saturating_sub(1);
        }
        n.lines.retain(|l| l.2 > 0);
        let Some(carried) = carried else {
            n.last = None;
            n.lines.clear();
            return;
        };
        let mut now: BTreeMap<String, i64> = BTreeMap::new();
        for (id, count) in carried {
            *now.entry(id).or_insert(0) += i64::from(count);
        }
        if let Some(before) = &n.last {
            let mut changes = Vec::new();
            for (id, count) in &now {
                let was = before.get(id).copied().unwrap_or(0);
                if *count != was {
                    changes.push((id.clone(), count - was));
                }
            }
            for (id, was) in before {
                if !now.contains_key(id) {
                    changes.push((id.clone(), -was));
                }
            }
            for (id, delta) in changes {
                // The same item again within a line's life adds to it.
                if let Some(line) = n
                    .lines
                    .iter_mut()
                    .find(|l| l.0 == id && (l.1 > 0) == (delta > 0))
                {
                    line.1 += delta;
                    line.2 = LIFE;
                } else {
                    n.lines.push((id, delta, LIFE));
                }
            }
            let over = n.lines.len().saturating_sub(MAX_LINES);
            n.lines.drain(..over);
        }
        n.last = Some(now);
    });
}

/// The box's lines: green for gains, red for losses (only when `losses`).
pub fn lines(losses: bool) -> Vec<(String, u32)> {
    NOTES.with_borrow(|n| {
        n.lines
            .iter()
            .filter(|l| losses || l.1 > 0)
            .map(|l| {
                let sign = if l.1 > 0 { "+" } else { "" };
                (
                    format!("{sign}{} {}", l.1, title(&l.0)),
                    if l.1 > 0 { 0x55_FF55 } else { 0xFF_5555 },
                )
            })
            .collect()
    })
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget_notes() {
    NOTES.with_borrow_mut(|n| *n = Notes::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_become_titles() {
        assert_eq!(title("minecraft:diamond_helmet"), "Diamond Helmet");
        assert_eq!(title("totem_of_undying"), "Totem Of Undying");
        assert_eq!(title(""), "");
    }

    fn carry(items: &[(&str, u32)]) -> Option<Vec<(String, u32)>> {
        Some(items.iter().map(|(i, c)| ((*i).to_owned(), *c)).collect())
    }

    #[test]
    fn the_first_look_is_a_baseline_not_a_pickup() {
        forget_notes();
        tick(carry(&[("minecraft:cobblestone", 64)]));
        assert!(lines(true).is_empty());
        forget_notes();
    }

    #[test]
    fn gains_and_losses_show_and_merge() {
        forget_notes();
        tick(carry(&[("minecraft:cobblestone", 10)]));
        tick(carry(&[("minecraft:cobblestone", 13)]));
        assert_eq!(lines(false), [("+3 Cobblestone".to_owned(), 0x55_FF55)]);
        tick(carry(&[("minecraft:cobblestone", 15)]));
        assert_eq!(lines(false)[0].0, "+5 Cobblestone", "the same item merges");
        tick(carry(&[]));
        assert!(lines(false).iter().all(|l| l.1 == 0x55_FF55));
        assert!(lines(true).iter().any(|l| l.0 == "-15 Cobblestone"));
        forget_notes();
    }

    #[test]
    fn lines_fade_after_their_life_and_clear_outside_a_world() {
        forget_notes();
        tick(carry(&[]));
        tick(carry(&[("minecraft:diamond", 1)]));
        assert_eq!(lines(false).len(), 1);
        for _ in 0..LIFE {
            tick(carry(&[("minecraft:diamond", 1)]));
        }
        assert!(lines(false).is_empty());
        tick(carry(&[("minecraft:diamond", 2)]));
        tick(None);
        assert!(lines(true).is_empty());
        forget_notes();
    }

    #[test]
    fn at_most_six_lines() {
        forget_notes();
        tick(carry(&[]));
        let many: Vec<(String, u32)> = (0..9).map(|i| (format!("item_{i}"), 1)).collect();
        tick(Some(many));
        assert_eq!(lines(false).len(), MAX_LINES);
        forget_notes();
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
