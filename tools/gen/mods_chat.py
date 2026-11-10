"""Chat and tooltip ports. The chat mods put their module in Horizon's chat.rs (the render crate, `horizon_client::chat`) and hook
`Chat::add_message`, `Chat::add_lines` and `Chat::layout`; the tooltip mod returns from `stack_tooltip`. Pinned to one Horizon commit."""
import common as c

CHAT = []
REV = c.HORIZON_REV


def toml_text(mod_id, version, module_file, module_name, injections, visibility=None, priority=100):
    out = f'''[mod]
id = "{mod_id}"
version = "{version}"
priority = {priority}

# The commit it was tested on. It hooks functions that are not Horizon's stable hook points, so it only builds on this commit.
[compatibility]
horizon_revision = "{REV}"

[[module]]
id = "runtime"
file = "{module_file}"
name = "{module_name}"
source = "runtime.rs"
'''
    if visibility:
        out += f'visibility = "{visibility}"\n'
    for inj in injections:
        out += "\n[[injection]]\n"
        for k, v in inj["fields"]:
            out += f"{k} = {v}\n"
        b = ",\n".join(f'  {{ name = "{n}", ty = "{t}", value = "{val}" }}' for n, t, val in inj["bindings"])
        out += f"bindings = [\n{b},\n]\n"
    return out


def native(h, fn, bindings):
    return dict(fields=[("id", f'"{h}"'), ("kind", '"HEAD"'),
                        ("target", f'"horizon_client::game::native_hooks::Game<\'_>::{fn}"'), ("helper", f'"{h}.rs"')], bindings=bindings)


G = ("game", "&mut crate::game::Game<'_>", "&mut *self")
LIST_B = [("mods", "&mut Vec<crate::game::native_hooks::NativeMod>", "&mut *mods")]
SET_B = [G, ("mod_id", "&str", "mod_id"), ("buttons", "&mut Vec<String>", "&mut *buttons")]
CLK_B = [G, ("mod_id", "&str", "mod_id"), ("button", "usize", "button")]


def common_native(extra_tick=True):
    inj = [native("list", "native_mods", LIST_B), native("settings", "native_settings", SET_B), native("click", "native_settings_click", CLK_B)]
    if extra_tick:
        inj.append(native("tick", "native_tick", [G]))
    return inj


# --------------------------------------------------------------------------------------------- more_chat_history
CHAT.append(dict(
    slug="more-chat-history", id="more_chat_history", name="More Chat History", title="More Chat History", raw=True,
    usage="Nothing to press, and nothing to configure (the original has no settings): chat keeps up to 16384 messages and lines instead of vanilla's 100, and you scroll back through them.",
    description="Increases the maximum length of the chat history, from the vanilla 100 messages to 16384.",
    upstream="More Chat History (morechathistory) by JackFred", upstream_licence="CC0-1.0",
    command=None, hooks=["add", "list", "tick"],
    options=None,
    modpath="horizon_client::chat::lib_more_chat_history",
    raw_note=("It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::push_message` (every message passes through it, a signed player message too) that does what the original does with a longer limit, including the line wrapping (Horizon's `add_lines` has a closure now, so its limit cannot be changed with a constant). It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    toml_text=toml_text("more_chat_history", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_more_chat_history", [
        dict(fields=[("id", '"add"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::Chat::push_message"'), ("helper", '"add.rs"')],
             bindings=[("chat", "&mut Chat", "&mut *self"), ("font", "&crate::font::Font", "font"), ("message", "&Message", "&message"),
                       ("focused", "bool", "focused")]),
    ] + [native("list", "native_mods", LIST_B), native("tick", "native_tick", [G])], visibility="pub"),
    runtime_extra=r'''
use std::cell::Cell;

/// Vanilla's limit.
pub const VANILLA: usize = 100;
/// The original's `EXPANDED_MAX_HISTORY`, fixed: it has no setting.
pub const EXPANDED: usize = 16384;

thread_local! {
    static LIMIT: Cell<usize> = const { Cell::new(EXPANDED) };
}

/// How many messages and wrapped lines chat keeps.
pub fn limit() -> usize {
    LIMIT.with(Cell::get)
}

/// The game tells the chat each tick: the expanded limit while the mod is on, vanilla's while it is off.
pub fn configure(on: bool) {
    LIMIT.with(|l| l.set(if on { EXPANDED } else { VANILLA }));
}
''',
    tests=r'''
    #[test]
    fn the_limit_is_the_originals_16384() {
        configure(true);
        assert_eq!(limit(), 16384);
    }

    #[test]
    fn off_gives_vanillas_hundred() {
        configure(false);
        assert_eq!(limit(), 100);
        configure(true);
    }

    #[test]
    fn it_follows_the_switch_both_ways() {
        configure(false);
        configure(true);
        assert_eq!(limit(), EXPANDED);
        configure(false);
        assert_eq!(limit(), VANILLA);
        configure(true);
    }

    #[test]
    fn the_limit_is_far_above_vanilla() {
        assert_eq!(EXPANDED / VANILLA, 163);
    }
''',
    helpers={
        "add": '''// ChatComponent.addMessage with the player's own history limit: `push_message`'s steps (every message passes through it,
// a signed player message too), with `truncate` taking the mod's limit.
fn more_chat_history_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    message: &Message,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    let message = message.clone();
    if !chat.abilities.shows(message.source) {
        return std::ops::ControlFlow::Break(());
    }
    let plain = message.content.plain();
    crate::logging::write_only(log::Level::Info, &format!("[CHAT] {plain}"));
    if chat.log {
        println!("[CHAT] {plain}");
    }
    // `add_lines` with the mod's limit (Horizon's own has a closure in it, which a constant cannot be changed in).
    let icon = if message.tag.as_ref().is_some_and(|tag| tag.icon) { 9 + 4 + 2 } else { 0 };
    let width = chat.max_width() - icon;
    let content = chat.displayed(&message.content);
    for line in font.wrap_components(&content, width) {
        if focused && chat.scroll > 0 {
            chat.new_since_scroll = true;
            chat.scroll_by(1, focused);
        }
        chat.lines.insert(0, Line { added: message.added, content: line, tag: message.tag.clone() });
    }
    chat.lines.truncate(lib_more_chat_history::limit());
    chat.messages.insert(0, message);
    chat.messages.truncate(lib_more_chat_history::limit());
    chat.received += 1;
    std::ops::ControlFlow::Break(())
}
''',
        "tick": '''// Once a tick: tell the chat the limit the player picked.
fn more_chat_history_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_more_chat_history as m;
    if !game.native_window() {
        return;
    }
    m::configure(game.native_on(m::ID));
}
''',
    },
))

# --------------------------------------------------------------------------------------------- chat_notify
CHAT.append(dict(
    slug="chat-notify", id="chat_notify", name="Chat Notify", title="Chat Notify", raw=True,
    usage="Nothing to press: a short sound plays when a message mentions your name. The settings page picks which messages count (players, the server) and whether your own count.",
    description="Plays a sound when someone says your name in chat.",
    upstream="Chat Notify (chatnotify)", upstream_licence="see the upstream project",
    command=None, hooks=["message", "list", "settings", "click", "tick"],
    options=[("players", "Player messages", True), ("server", "Server messages", False), ("own", "My own messages", False)],
    modpath="horizon_client::chat::lib_chat_notify",
    raw_note=("It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`) and hooks `Chat::add_message` (before the original, so it also runs when another mod replaces it), "
              "then plays the sound from the game's tick, using `Game.audio` and `Options::player_name`. None of that is part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    toml_text=toml_text("chat_notify", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_chat_notify", [
        dict(fields=[("id", '"message"'), ("kind", '"HEAD"'), ("target", '"horizon_client_render::chat::Chat::add_message"'), ("helper", '"message.rs"')],
             bindings=[("content", "&Text", "&content"), ("source", "ChatSource", "source")]),
    ] + common_native(), visibility="pub", priority=200),
    runtime_extra=r'''
use std::cell::Cell;

thread_local! {
    static PLAYER: RefCell<String> = const { RefCell::new(String::new()) };
    static PENDING: Cell<bool> = const { Cell::new(false) };
    /// Which sources count: players, the server, the player's own messages.
    static WANT: Cell<[bool; 3]> = const { Cell::new([true, false, false]) };
}

/// The player's name, set each tick.
pub fn set_name(name: &str) {
    PLAYER.with_borrow_mut(|n| {
        if n != name {
            name.clone_into(n);
        }
    });
}

/// Which messages count, set each tick.
pub fn configure(players: bool, server: bool, own: bool) {
    WANT.with(|w| w.set([players, server, own]));
}

/// Whether `name` appears in `text` as a word (case does not matter).
pub fn mentions(text: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let (t, n) = (text.to_lowercase(), name.to_lowercase());
    let word = |c: Option<char>| c.is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
    t.match_indices(&n).any(|(at, _)| word(t[..at].chars().next_back()) && word(t[at + n.len()..].chars().next()))
}

/// Whether the message is the player's own, in the usual `<name> text` form.
pub fn is_own(text: &str, name: &str) -> bool {
    text.to_lowercase().starts_with(&format!("<{}>", name.to_lowercase()))
}

/// A chat line arrived: remember to ping when it counts. `player` and `server` say where it came from.
pub fn message(text: &str, player: bool, server: bool) {
    let [players, servers, own] = WANT.with(Cell::get);
    let allowed = (player && players) || (server && servers);
    if !allowed {
        return;
    }
    let hit = PLAYER.with_borrow(|name| mentions(text, name) && (own || !is_own(text, name)));
    if hit {
        PENDING.with(|p| p.set(true));
    }
}

/// Whether a ping is due, and clears it.
pub fn take() -> bool {
    PENDING.with(|p| p.replace(false))
}
''',
    tests=r'''
    #[test]
    fn names_match_as_words_in_any_case() {
        assert!(mentions("hey Steve, look", "steve"));
        assert!(mentions("STEVE!", "Steve"));
        assert!(mentions("<Alex> steve", "Steve"));
        assert!(!mentions("steven is here", "steve"), "a longer word is not a mention");
        assert!(!mentions("nobody", "steve"));
        assert!(!mentions("anything", ""));
        assert!(!mentions("ping steve_", "steve"), "an underscore continues the word");
    }

    #[test]
    fn own_messages_are_recognised() {
        assert!(is_own("<Steve> hello", "steve"));
        assert!(!is_own("<Alex> hello steve", "steve"));
    }

    #[test]
    fn pings_follow_the_sources_and_the_own_option() {
        set_name("Steve");
        configure(true, false, false);
        take();
        message("<Alex> hi steve", true, false);
        assert!(take());
        message("<Steve> talking to myself, steve", true, false);
        assert!(!take(), "own messages are skipped");
        configure(true, false, true);
        message("<Steve> talking to myself, steve", true, false);
        assert!(take());
        configure(true, false, false);
        message("steve joined", false, true);
        assert!(!take(), "server messages are off");
        configure(true, true, false);
        message("steve joined", false, true);
        assert!(take());
        assert!(!take(), "a ping fires once");
        configure(true, false, false);
    }
''',
    helpers={
        "message": '''// A chat line arrived (before the original runs).
fn chat_notify_message(content: &Text, source: ChatSource) {
    lib_chat_notify::message(&content.plain(), matches!(source, ChatSource::Player), matches!(source, ChatSource::Server));
}
''',
        "tick": '''// Once a tick: give the chat module the player's name and options, and play the sound when a ping is due.
fn chat_notify_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_notify as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    m::set_name(&game.options.player_name());
    let get = |k: &str| game.native_setting(m::ID, k);
    m::configure(m::is(get, "players"), m::is(get, "server"), m::is(get, "own"));
    if m::take() {
        let at = game.native_player().map(|p| [p.x, p.y, p.z]);
        game.audio.play("minecraft:entity.experience_orb.pickup", at, 1.0, 1.0);
    }
}
''',
    },
))

# --------------------------------------------------------------------------------------------- chat_animation
CHAT.append(dict(
    slug="chat-animation", id="chat_animation", name="Chat Animation", title="Chat Animation", raw=False, needs_api=404,
    usage="Nothing to press: new messages rise into place and fade in, the input field slides up when chat opens, and the indicator line is hidden. The settings page has Animate Messages, Duration, Hide Indicator Line, Animate Opacity, Animate Input Field and Duration.",
    description="Makes chat messages appear with a smooth animation.",
    upstream="Chat Animation [Smooth Chat] (chatanimation) by Ezzenix", upstream_licence="MIT",
    command=None, hooks=["list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    runtime_extra=r'''
/// The durations the two Duration buttons step through, in milliseconds (the original takes any number; the `message_ms` and
/// `input_ms` files do too).
pub const STEPS: [i32; 8] = [0, 50, 100, 150, 170, 300, 500, 1000];

/// The settings of `ModConfig`, with its defaults, in the original's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `Animate Messages`.
    pub animate_messages: bool,
    /// `Duration` of the messages' animation.
    pub message_ms: i32,
    /// `Hide Indicator Line`: "Hide the indicator line on the left of messages."
    pub hide_indicator: bool,
    /// `Animate Opacity`.
    pub animate_opacity: bool,
    /// `Animate Input Field`.
    pub animate_input: bool,
    /// `Duration` of the input field's animation.
    pub input_ms: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { animate_messages: true, message_ms: 150, hide_indicator: true, animate_opacity: true, animate_input: true, input_ms: 170 }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        let ms = |key: &str, d: i32| get(key).and_then(|v| v.trim().parse::<i32>().ok()).map_or(d, |v| v.max(0));
        Self {
            animate_messages: flag("animate_messages", d.animate_messages),
            message_ms: ms("message_ms", d.message_ms),
            hide_indicator: flag("hide_indicator", d.hide_indicator),
            animate_opacity: flag("animate_opacity", d.animate_opacity),
            animate_input: flag("animate_input", d.animate_input),
            input_ms: ms("input_ms", d.input_ms),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Animate Messages: {}", on(self.animate_messages)),
            format!("Messages Duration: {} ms", self.message_ms),
            format!("Hide Indicator Line: {}", on(self.hide_indicator)),
            format!("Animate Opacity: {}", on(self.animate_opacity)),
            format!("Animate Input Field: {}", on(self.animate_input)),
            format!("Input Field Duration: {} ms", self.input_ms),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let step = |v: i32| *STEPS.iter().find(|&&s| s > v).unwrap_or(&STEPS[0]);
        match i {
            0 => {
                self.animate_messages = !self.animate_messages;
                Some(("animate_messages", self.animate_messages.to_string()))
            }
            1 => {
                self.message_ms = step(self.message_ms);
                Some(("message_ms", self.message_ms.to_string()))
            }
            2 => {
                self.hide_indicator = !self.hide_indicator;
                Some(("hide_indicator", self.hide_indicator.to_string()))
            }
            3 => {
                self.animate_opacity = !self.animate_opacity;
                Some(("animate_opacity", self.animate_opacity.to_string()))
            }
            4 => {
                self.animate_input = !self.animate_input;
                Some(("animate_input", self.animate_input.to_string()))
            }
            5 => {
                self.input_ms = step(self.input_ms);
                Some(("input_ms", self.input_ms.to_string()))
            }
            _ => None,
        }
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
        assert_eq!((s.animate_messages, s.message_ms, s.hide_indicator), (true, 150, true));
        assert_eq!((s.animate_opacity, s.animate_input, s.input_ms), (true, true, 170));
        assert_eq!(s.labels().len(), 6);
    }

    #[test]
    fn stored_values_are_read_and_junk_falls_back() {
        let stored = |k: &str| match k {
            "message_ms" => Some("300".to_owned()),
            "animate_input" => Some("false".to_owned()),
            _ => None,
        };
        let s = Settings::read(stored);
        assert_eq!((s.message_ms, s.animate_input), (300, false));
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let neg = |k: &str| (k == "input_ms").then(|| "-5".to_owned());
        assert_eq!(Settings::read(neg).input_ms, 0);
    }

    #[test]
    fn the_labels_follow_the_originals_order() {
        let l = labels(none);
        assert!(l[0].starts_with("Animate Messages") && l[1].starts_with("Messages Duration") && l[2].starts_with("Hide Indicator Line"));
        assert!(l[3].starts_with("Animate Opacity") && l[4].starts_with("Animate Input Field") && l[5].starts_with("Input Field Duration"));
    }

    #[test]
    fn the_buttons_flip_and_the_durations_step_and_wrap() {
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("animate_messages", "false"));
        let (key, value) = click(none, 1).unwrap();
        assert_eq!((key, value.as_str()), ("message_ms", "170"), "150 steps to 170");
        let top = |k: &str| (k == "input_ms").then(|| "1000".to_owned());
        assert_eq!(click(top, 5).unwrap().1, "0", "1000 wraps to 0");
        let (key, value) = click(none, 2).unwrap();
        assert_eq!((key, value.as_str()), ("hide_indicator", "false"));
        assert!(click(none, 9).is_none());
    }
''',
    helpers={
        "tick": '''// Once a tick: hand the game the chat animation these settings give (vanilla chat when the mod is off).
fn chat_animation_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_chat_animation as m;
    if !game.native_on(m::ID) {
        game.native_chat_animation_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_chat_animation(s.animate_messages, s.message_ms, s.animate_opacity, s.hide_indicator, s.animate_input, s.input_ms);
}
''',
    },
))

# --------------------------------------------------------------------------------------------- durability_tooltip
CHAT.append(dict(
    slug="durability-tooltip", id="durability_tooltip", name="Durability Tooltip", title="Durability Tooltip", raw=True,
    usage="Nothing to press: hover a damageable item and its tooltip gains a line such as `Durability: 120 / 250 (48%)`, green, yellow or red.",
    description="Adds a Durability line to the tooltip of tools, weapons and armor.",
    upstream="Durability Tooltip (durabilitytooltip)", upstream_licence="see the upstream project",
    command=None, hooks=["tooltip", "list", "settings", "click"],
    options=[("percent", "Show percent", True), ("full", "Show on undamaged items", False)],
    toml_text=toml_text("durability_tooltip", "1.0.0", "crates/horizon-client/src/game.rs", "lib_durability_tooltip", [
        dict(fields=[("id", '"tooltip"'), ("kind", '"RETURN"'), ("target", '"horizon_client::game::stations::Game<\'_>::stack_tooltip"'), ("helper", '"tooltip.rs"')],
             bindings=[("lines", "Vec<(horizon_client::text::Text, u32)>", "$value"), ("game", "&crate::game::Game<'_>", "&*self"), ("stack", "&ItemStack", "stack")]),
    ] + common_native(False)),
    raw_note=("It adds a RETURN modifier to `Game::stack_tooltip` in Horizon's stations code and reads the item registry (`Game::items`) for each item's maximum damage, "
              "neither of which is part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    runtime_extra=r'''
/// A tooltip line's colour as red, green, blue.
pub type Rgb = (u8, u8, u8);

/// The line to add: `Durability: 120 / 250 (48%)`, coloured by how much is left. None for items that do not wear out,
/// and for undamaged ones unless `show_full`.
pub fn line(damage: u32, max: u32, show_percent: bool, show_full: bool) -> Option<(String, Rgb)> {
    if max == 0 || (damage == 0 && !show_full) {
        return None;
    }
    let left = max.saturating_sub(damage);
    let percent = left * 100 / max;
    let colour = match percent {
        51.. => (85, 255, 85),
        26..=50 => (255, 255, 85),
        _ => (255, 85, 85),
    };
    let text = if show_percent { format!("Durability: {left} / {max} ({percent}%)") } else { format!("Durability: {left} / {max}") };
    Some((text, colour))
}
''',
    tests=r'''
    #[test]
    fn only_damaged_wearing_items_get_a_line() {
        assert_eq!(line(0, 250, true, false), None);
        assert_eq!(line(10, 0, true, false), None, "no durability");
        assert_eq!(line(0, 250, true, true).unwrap().0, "Durability: 250 / 250 (100%)");
    }

    #[test]
    fn the_line_and_colour_follow_what_is_left() {
        assert_eq!(line(130, 250, true, false), Some(("Durability: 120 / 250 (48%)".to_owned(), (255, 255, 85))));
        assert_eq!(line(10, 250, false, false), Some(("Durability: 240 / 250".to_owned(), (85, 255, 85))));
        assert_eq!(line(240, 250, true, false).unwrap().1, (255, 85, 85));
        assert_eq!(line(999, 250, true, false).unwrap().0, "Durability: 0 / 250 (0%)", "never below zero");
    }
''',
    helpers={
        "tooltip": '''// ItemStack.getTooltipLines, then a Durability line for items that wear out.
fn durability_tooltip_tooltip(
    mut lines: Vec<(horizon_client::text::Text, u32)>,
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
) -> Vec<(horizon_client::text::Text, u32)> {
    use crate::game::lib_durability_tooltip as m;
    if !game.native_on(m::ID) {
        return lines;
    }
    let max = game.items().get(&stack.item).map_or(0, |info| info.max_damage);
    let get = |k: &str| game.native_setting(m::ID, k);
    if let Some((text, (r, g, b))) = m::line(stack.damage, max, m::is(get, "percent"), m::is(get, "full")) {
        lines.push((horizon_client::text::Text::literal(text), rgba(r, g, b, 255)));
    }
    lines
}
''',
    },
))
