"""Ping View: Horizon keeps the connection's ping samples in a logger with no getter, so the mod hooks the logger's `log` and keeps its own copy."""
from mods_chat import toml_text, native, G, LIST_B, SET_B, CLK_B
from mods_raw import DRAW_INK

PING = []

COMMAND_HELPER = '''// `/ping` hides or shows the ping box.
fn ping_view_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use horizon_client::debug_chart::lib_ping_view as m;
    let name = command.split(' ').next().unwrap_or("");
    if !game.native_on(m::ID) || name != m::COMMAND {
        return;
    }
    *ran = true;
    let stored = {
        let game = &*game;
        m::click(|k| game.native_setting(m::ID, k), 0)
    };
    if let Some((key, value)) = stored {
        game.native_set_setting(m::ID, key, &value);
        game.native_chat(&format!("{} {}", m::NAME, if value == "true" { "shown" } else { "hidden" }));
    }
}
'''

PING.append(dict(
    slug="ping-view", id="ping_view", name="Ping View", title="Ping View", raw=True,
    usage="| Command | What it does |\\n|---|---|\\n| `/ping` | Hides or shows the ping box |\\n\\nOn a server, a box shows your ping (the last reply) with the average of the last 20 replies and the best and worst, coloured green, yellow or red. On single player there are no pings and nothing shows.",
    description="Shows your ping to the server: the latest reply, the average of the last 20 and the best and worst.",
    upstream="Ping View (pingview)", upstream_licence="see the upstream project",
    command="ping", hooks=["log", "frame", "hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("average", "Average of the last 20", True), ("range", "Best and worst", False), ("colour", "Colour by ping", True)],
    modpath="horizon_client::debug_chart::lib_ping_view",
    raw_note=("Horizon keeps the connection's ping samples in a `SampleLogger` that has no getter, so the mod's module lives in `debug_chart.rs` and a HEAD hook on `SampleLogger::log` copies the samples of "
              "the one logger the game tells it about (the ping logger, found by its address). That is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    toml_text=toml_text("ping_view", "1.0.0", "crates/horizon-client/src/debug_chart.rs", "lib_ping_view", [
        dict(fields=[("id", '"log"'), ("kind", '"HEAD"'), ("target", '"horizon_client_lib::debug_chart::SampleLogger::log"'), ("helper", '"log.rs"')],
             bindings=[("logger", "&SampleLogger", "&*self"), ("value", "i64", "value")]),
        native("frame", "native_frame", [G, ("elapsed", "std::time::Duration", "elapsed")]),
        native("hud", "native_hud", [G, ("batch", "&mut horizon_client::hud::HudBatch", "&mut *batch"), ("corners", "&mut horizon_client::extras_hud::Corners", "&mut *corners")]),
        native("command", "native_command", [G, ("command", "&str", "command"), ("ran", "&mut bool", "&mut *ran")]),
        native("list", "native_mods", LIST_B), native("settings", "native_settings", SET_B), native("click", "native_settings_click", CLK_B),
    ], visibility="pub"),
    runtime_extra=r'''
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
        Some(Stats { last, average: sum / s.len() as i64, best: *s.iter().min()?, worst: *s.iter().max()? })
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
        out.push((format!("Average {} ms", s.average), ink(s.average, coloured)));
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
''',
    tests=r'''
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
        assert_eq!((s.last, s.best, s.worst), (250, 60, 250), "the first five fell out");
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
        let s = Stats { last: 42, average: 55, best: 30, worst: 120 };
        assert_eq!(lines(s, false, false, true).len(), 1);
        let all = lines(s, true, true, true);
        assert_eq!(all[0].0, "Ping 42 ms");
        assert_eq!(all[1].0, "Average 55 ms");
        assert_eq!(all[2].0, "Best 30 / worst 120 ms");
    }
''',
    helpers={
        "log": '''// SampleLogger.logSample: keep a copy when this is the ping logger.
fn ping_view_log(logger: &SampleLogger, value: i64) {
    lib_ping_view::sample(logger as *const SampleLogger as usize, value);
}
''',
        "frame": '''// Each frame: tell the mod which logger is the ping logger.
fn ping_view_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use horizon_client::debug_chart::lib_ping_view as m;
    let _ = elapsed;
    let ping: &horizon_client::debug_chart::SampleLogger = &game.debug_charts.ping;
    m::watch(ping as *const horizon_client::debug_chart::SampleLogger as usize);
}
''',
        "hud": '''// The ping box.
fn ping_view_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use horizon_client::debug_chart::lib_ping_view as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") {
        return;
    }
    let Some(stats) = m::stats() else {
        return;
    };
    let lines = m::lines(stats, m::is(get, "average"), m::is(get, "range"), m::is(get, "colour"));
''' + DRAW_INK.replace("@CORNER@", "Corner::BottomRight").replace("\n    if lines.is_empty() {\n        return;\n    }", "") + '''
}
''',
        "command": COMMAND_HELPER,
    },
))
