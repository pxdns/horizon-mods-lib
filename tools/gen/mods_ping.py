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
    slug="ping-view", id="ping_view", name="Ping View", title="Ping View", raw=False, needs_api=406,
    usage="Nothing to press: the tab list shows each player's ping as coloured text (`42ms`) instead of the bars, and `N/A` when unknown. The settings page has the five colours, Show 'ms' suffix, Show ping on nametags and Text shadow.",
    description="Displays your and other players ping in the tab list, with color.",
    upstream="Ping View (pingview) by karlssonsmp", upstream_licence="MIT",
    command=None, hooks=["list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    runtime_extra=r'''
/// The colours the five colour buttons cycle through (the original takes any `#RRGGBB`; the colour files do too). Each default is in it.
pub const PALETTE: [&str; 8] = ["#1EFF00", "#FFF100", "#FF9500", "#FF3B3B", "#555555", "#FFFFFF", "#55FFFF", "#AA00AA"];
/// The files and labels of the colours, in the original's order.
pub const COLOURS: [(&str, &str); 5] = [
    ("good", "Good Ping Color"),
    ("ok", "Ok Ping Color"),
    ("bad", "Bad Ping Color"),
    ("terrible", "Terrible Ping Color"),
    ("unknown", "Unknown Ping Color"),
];

/// The settings of `PingViewConfig`, with its defaults.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// good, ok, bad, terrible, unknown as `#RRGGBB` text.
    pub colors: [String; 5],
    /// `showMs`.
    pub show_ms: bool,
    /// `showNametagPing`.
    pub show_nametag: bool,
    /// `textShadow`.
    pub text_shadow: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { colors: ["#1EFF00", "#FFF100", "#FF9500", "#FF3B3B", "#555555"].map(str::to_owned), show_ms: true, show_nametag: false, text_shadow: true }
    }
}

/// `PingViewConfig.parseColor`: an optional `#`, then hex digits; white when it does not parse.
pub fn parse_color(text: &str) -> u32 {
    let digits = text.strip_prefix('#').unwrap_or(text);
    u32::from_str_radix(digits, 16).unwrap_or(0xFF_FFFF)
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        let mut colors = d.colors.clone();
        for (i, (key, _)) in COLOURS.iter().enumerate() {
            if let Some(v) = get(key) {
                colors[i] = v.trim().to_owned();
            }
        }
        Self { colors, show_ms: flag("show_ms", d.show_ms), show_nametag: flag("show_nametag", d.show_nametag), text_shadow: flag("text_shadow", d.text_shadow) }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        let mut out: Vec<String> = COLOURS.iter().zip(&self.colors).map(|((_, label), c)| format!("{label}: {c}")).collect();
        out.push(format!("Show 'ms' suffix: {}", on(self.show_ms)));
        out.push(format!("Show ping on nametags: {}", on(self.show_nametag)));
        out.push(format!("Text shadow: {}", on(self.text_shadow)));
        out
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0..=4 => {
                let now = self.colors[i].to_uppercase();
                let at = PALETTE.iter().position(|p| *p == now).map_or(0, |p| (p + 1) % PALETTE.len());
                self.colors[i] = PALETTE[at].to_owned();
                Some((COLOURS[i].0, self.colors[i].clone()))
            }
            5 => {
                self.show_ms = !self.show_ms;
                Some(("show_ms", self.show_ms.to_string()))
            }
            6 => {
                self.show_nametag = !self.show_nametag;
                Some(("show_nametag", self.show_nametag.to_string()))
            }
            7 => {
                self.text_shadow = !self.text_shadow;
                Some(("text_shadow", self.text_shadow.to_string()))
            }
            _ => None,
        }
    }

    /// The colours as the game takes them, 0xRRGGBB.
    pub fn rgb(&self) -> [u32; 5] {
        std::array::from_fn(|i| parse_color(&self.colors[i]))
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
        assert_eq!(s.colors, ["#1EFF00", "#FFF100", "#FF9500", "#FF3B3B", "#555555"]);
        assert_eq!((s.show_ms, s.show_nametag, s.text_shadow), (true, false, true));
        assert_eq!(s.rgb(), [0x1E_FF00, 0xFF_F100, 0xFF_9500, 0xFF_3B3B, 0x55_5555]);
    }

    #[test]
    fn colours_parse_with_or_without_a_hash_and_fall_back_to_white() {
        assert_eq!(parse_color("#FF9500"), 0xFF_9500);
        assert_eq!(parse_color("ff9500"), 0xFF_9500);
        assert_eq!(parse_color("nope"), 0xFF_FFFF);
        let bad = |k: &str| (k == "good").then(|| "zzz".to_owned());
        assert_eq!(Settings::read(bad).rgb()[0], 0xFF_FFFF);
    }

    #[test]
    fn the_colour_buttons_walk_the_palette_and_report_the_file() {
        let (file, value) = click(none, 0).unwrap();
        assert_eq!((file, value.as_str()), ("good", "#FFF100"), "the default is the first palette entry, so the next one follows");
        let last = |k: &str| (k == "unknown").then(|| "#AA00AA".to_owned());
        assert_eq!(click(last, 4).unwrap().1, "#1EFF00", "the palette wraps");
        let custom = |k: &str| (k == "bad").then(|| "#123456".to_owned());
        assert_eq!(click(custom, 2).unwrap().1, "#1EFF00", "a colour typed into the file starts the walk again");
    }

    #[test]
    fn the_switches_flip_in_the_originals_order() {
        assert_eq!(click(none, 5).unwrap(), ("show_ms", "false".to_owned()));
        assert_eq!(click(none, 6).unwrap(), ("show_nametag", "true".to_owned()));
        assert_eq!(click(none, 7).unwrap(), ("text_shadow", "false".to_owned()));
        assert!(click(none, 8).is_none());
        let l = labels(none);
        assert_eq!(l.len(), 8);
        assert!(l[0].starts_with("Good Ping Color") && l[4].starts_with("Unknown Ping Color") && l[5].starts_with("Show 'ms'"));
    }
''',
    helpers={
        "tick": '''// Once a tick: hand the game the ping view these settings give (the vanilla bars when the mod is off).
fn ping_view_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_ping_view as m;
    if !game.native_on(m::ID) {
        game.native_ping_view_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_ping_view(s.rgb(), s.show_ms, s.show_nametag, s.text_shadow);
}
''',
    },
))
