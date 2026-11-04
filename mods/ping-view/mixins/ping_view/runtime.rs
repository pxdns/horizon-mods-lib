// Ping View, a native Horizon mod: Shows your ping to the server: the latest reply, the average of the last 20 and the best and worst.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_ping_view`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "ping_view";
pub const NAME: &str = "Ping View";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows your ping to the server: the latest reply, the average of the last 20 and the best and worst.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("average", "Average of the last 20", true),
    ("range", "Best and worst", false),
    ("colour", "Colour by ping", true),
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
use std::collections::VecDeque;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "ping";
pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;
/// How many replies the average and range look at.
pub const WINDOW: usize = 20;

thread_local! {
    /// The address of the ping logger, set by the game each frame.
    static WATCH: Cell<usize> = const { Cell::new(0) };
    static SAMPLES: RefCell<VecDeque<i64>> = const { RefCell::new(VecDeque::new()) };
}

/// The game tells the mod which logger is the ping logger (its address).
pub fn watch(address: usize) {
    WATCH.with(|w| {
        if w.get() != address {
            w.set(address);
            SAMPLES.with_borrow_mut(VecDeque::clear);
        }
    });
}

/// A logger took a sample: keep it when the logger is the ping logger.
pub fn sample(address: usize, value: i64) {
    if address != 0 && address == WATCH.with(Cell::get) {
        SAMPLES.with_borrow_mut(|s| {
            if s.len() == WINDOW {
                s.pop_front();
            }
            s.push_back(value.max(0));
        });
    }
}

/// What the box shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stats {
    pub last: i64,
    pub average: i64,
    pub best: i64,
    pub worst: i64,
}

/// The stats of the replies kept; None before the first.
pub fn stats() -> Option<Stats> {
    SAMPLES.with_borrow(|s| {
        let last = *s.back()?;
        let sum: i64 = s.iter().sum();
        Some(Stats {
            last,
            average: sum / s.len() as i64,
            best: *s.iter().min()?,
            worst: *s.iter().max()?,
        })
    })
}

/// Green under 80 ms, yellow under 150, red above; white when colouring is off.
pub fn ink(ms: i64, coloured: bool) -> u32 {
    match (coloured, ms) {
        (false, _) => INK,
        (true, ..=79) => 0x55_FF55,
        (true, 80..=149) => 0xFF_FF55,
        (true, _) => 0xFF_5555,
    }
}

/// The box's lines.
pub fn lines(s: Stats, average: bool, range: bool, coloured: bool) -> Vec<(String, u32)> {
    let mut out = vec![(format!("Ping {} ms", s.last), ink(s.last, coloured))];
    if average {
        out.push((
            format!("Average {} ms", s.average),
            ink(s.average, coloured),
        ));
    }
    if range {
        out.push((format!("Best {} / worst {} ms", s.best, s.worst), INK));
    }
    out
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget_samples() {
    WATCH.with(|w| w.set(0));
    SAMPLES.with_borrow_mut(VecDeque::clear);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_watched_logger_is_kept() {
        forget_samples();
        watch(0x1000);
        sample(0x2000, 5_000_000);
        assert_eq!(stats(), None, "another logger (frame times)");
        sample(0x1000, 40);
        assert_eq!(stats().unwrap().last, 40);
        sample(0, 1);
        assert_eq!(stats().unwrap().last, 40, "address 0 means unwatched");
        forget_samples();
    }

    #[test]
    fn stats_cover_the_last_twenty_replies() {
        forget_samples();
        watch(7);
        for ms in 1..=25 {
            sample(7, ms * 10);
        }
        let s = stats().unwrap();
        assert_eq!(
            (s.last, s.best, s.worst),
            (250, 60, 250),
            "the first five fell out"
        );
        assert_eq!(s.average, (60..=250).step_by(10).sum::<i64>() / 20);
        sample(7, -5);
        assert_eq!(stats().unwrap().last, 0, "never negative");
        forget_samples();
    }

    #[test]
    fn a_different_logger_resets_the_samples() {
        forget_samples();
        watch(1);
        sample(1, 30);
        watch(2);
        assert_eq!(stats(), None);
        forget_samples();
    }

    #[test]
    fn colours_and_lines_follow_the_options() {
        assert_eq!(ink(30, true), 0x55_FF55);
        assert_eq!(ink(80, true), 0xFF_FF55);
        assert_eq!(ink(150, true), 0xFF_5555);
        assert_eq!(ink(30, false), INK);
        let s = Stats {
            last: 42,
            average: 55,
            best: 30,
            worst: 120,
        };
        assert_eq!(lines(s, false, false, true).len(), 1);
        let all = lines(s, true, true, true);
        assert_eq!(all[0].0, "Ping 42 ms");
        assert_eq!(all[1].0, "Average 55 ms");
        assert_eq!(all[2].0, "Best 30 / worst 120 ms");
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
