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
    usage="Nothing to press: chat keeps more of what was said and you scroll back through it. The first button picks how many lines (100, 250, 500, 1000 or 2000).",
    description="Chat remembers up to 2000 lines instead of 100, so you can scroll back through a long conversation.",
    upstream="More Chat History (morechathistory)", upstream_licence="see the upstream project",
    command=None, hooks=["add", "lines", "list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    modpath="horizon_client::chat::lib_more_chat_history",
    raw_note=("It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::push_message` (every message passes through it, a signed player message too) that does what the original does with a longer limit, and a "
              "MODIFY_CONSTANT on the wrapped-line limit in `Chat::add_lines`. Neither is part of the stable hook-point contract, so it is **pinned to one Horizon commit**."),
    toml_text=toml_text("more_chat_history", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_more_chat_history", [
        dict(fields=[("id", '"add"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::Chat::push_message"'), ("helper", '"add.rs"')],
             bindings=[("chat", "&mut Chat", "&mut *self"), ("font", "&crate::font::Font", "font"), ("message", "&Message", "&message"),
                       ("focused", "bool", "focused")]),
        dict(fields=[("id", '"lines"'), ("kind", '"MODIFY_CONSTANT"'), ("target", '"horizon_client_render::chat::Chat::add_lines"'), ("selector", '"100"'),
                     ("require", "1"), ("ordinal", "0"), ("helper", '"lines.rs"')],
             bindings=[("value", "usize", "$value")]),
    ] + common_native(), visibility="pub"),
    runtime_extra=r'''
use std::cell::Cell;

/// The line limits the first button cycles through.
pub const LEVELS: [usize; 5] = [100, 250, 500, 1000, 2000];
/// Horizon's own limit, and the default here.
pub const DEFAULT: usize = 1000;

thread_local! {
    static LIMIT: Cell<usize> = const { Cell::new(DEFAULT) };
}

/// How many messages and wrapped lines chat keeps.
pub fn limit() -> usize {
    LIMIT.with(Cell::get)
}

/// Sets the limit from the stored value (the game tells the chat each tick).
pub fn configure(limit: usize) {
    LIMIT.with(|l| l.set(limit));
}

/// The stored `limit`, one of `LEVELS`, else the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> usize {
    get("limit").and_then(|v| v.trim().parse().ok()).filter(|v| LEVELS.contains(v)).unwrap_or(DEFAULT)
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    vec![format!("History: {} lines", read(get))]
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    if i != 0 {
        return None;
    }
    let current = read(get);
    let at = LEVELS.iter().position(|&l| l == current).unwrap_or(0);
    let next = LEVELS[(at + 1) % LEVELS.len()];
    configure(next);
    Some(("limit", next.to_string()))
}
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_default_is_a_thousand_lines() {
        assert_eq!(read(none), 1000);
        assert_eq!(labels(none), ["History: 1000 lines"]);
    }

    #[test]
    fn the_button_cycles_through_the_levels_and_sets_the_limit() {
        let stored = |k: &str| (k == "limit").then(|| "2000".to_owned());
        let (key, value) = click(stored, 0).unwrap();
        assert_eq!((key, value.as_str()), ("limit", "100"), "2000 wraps to 100");
        assert_eq!(limit(), 100);
        let (_, next) = click(none, 0).unwrap();
        assert_eq!(next, "2000", "from the default 1000 the next is 2000");
        assert_eq!(limit(), 2000);
        assert!(click(none, 3).is_none());
        configure(DEFAULT);
    }

    #[test]
    fn stored_junk_falls_back() {
        assert_eq!(read(|_| Some("7".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some("banana".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some(" 500\n".to_owned())), 500);
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
    chat.add_lines(font, &message, focused);
    chat.messages.insert(0, message);
    chat.messages.truncate(lib_more_chat_history::limit());
    chat.received += 1;
    std::ops::ControlFlow::Break(())
}
''',
        "lines": '''// The wrapped-line limit.
fn more_chat_history_lines(value: usize) -> usize {
    let _ = value;
    lib_more_chat_history::limit()
}
''',
        "tick": '''// Once a tick: tell the chat the limit the player picked.
fn more_chat_history_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_more_chat_history as m;
    if !game.native_window() {
        return;
    }
    m::configure(if game.native_on(m::ID) { m::read(|k| game.native_setting(m::ID, k)) } else { 100 });
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
    slug="chat-animation", id="chat_animation", name="Chat Animation", title="Chat Animation", raw=True,
    usage="Nothing to press: new chat lines slide up into place. The settings page switches the slide on or off.",
    description="New chat lines slide up into place instead of appearing at once.",
    upstream="Chat Animation [Smooth Chat] (chatanimation)", upstream_licence="see the upstream project",
    command=None, hooks=["layout", "list", "settings", "click", "tick"],
    options=[("slide", "Slide in", True)],
    modpath="horizon_client::chat::lib_chat_animation",
    raw_note=("**Retired on current Horizon: its smooth chat is built in.** Horizon's chat now slides a new line in and fades it in by itself (the smooth chat extra, `Chat::smooth`), "
              "and `Chat::layout` takes fractional ticks and `Line::tag` where this mod's copy of the layout expects whole ticks and `Line::indicator`, so the mod no longer compiles there. "
              "Use Horizon's own setting; this folder stays for the Horizon commit it was written on. "
              "It lives in Horizon's chat module and replaces `Chat::layout` (a cancellable HEAD hook that rebuilds the draw list the same way, with each new line offset by the slide) - "
              "not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. The chat only knows whole ticks, so the slide moves in 50 ms steps."),
    toml_text=toml_text("chat_animation", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_chat_animation", [
        dict(fields=[("id", '"layout"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::Chat::layout"'), ("helper", '"layout.rs"')],
             bindings=[("chat", "&Chat", "&*self"), ("height", "i32", "height"), ("ticks", "u64", "ticks"), ("focused", "bool", "focused")]),
    ] + common_native(), visibility="pub"),
    runtime_extra=r'''
use std::cell::Cell;

/// How long a line slides, in ticks.
pub const SLIDE_TICKS: u64 = 6;

thread_local! {
    static ON: Cell<bool> = const { Cell::new(true) };
}

/// Switches the slide on or off (the game tells the chat each tick).
pub fn configure(on: bool) {
    ON.with(|o| o.set(on));
}

/// Pixels a line added at `added` is still below its place at tick `ticks`: the whole line height at first, easing to nothing.
pub fn slide(ticks: u64, added: u64, line_height: i32) -> i32 {
    if !ON.with(Cell::get) {
        return 0;
    }
    let age = ticks.saturating_sub(added);
    if age >= SLIDE_TICKS {
        return 0;
    }
    let left = 1.0 - age as f32 / SLIDE_TICKS as f32;
    // Ease out: fast at the start, settling at the end.
    (left * left * line_height as f32).round() as i32
}
''',
    tests=r'''
    #[test]
    fn a_new_line_starts_a_full_line_below_and_settles() {
        configure(true);
        assert_eq!(slide(10, 10, 9), 9);
        assert!(slide(11, 10, 9) < 9 && slide(11, 10, 9) > slide(12, 10, 9));
        assert_eq!(slide(10 + SLIDE_TICKS, 10, 9), 0);
        assert_eq!(slide(500, 10, 9), 0, "old lines do not move");
        assert_eq!(slide(5, 10, 9), 9, "a line from the future counts as new");
    }

    #[test]
    fn the_slide_can_be_switched_off() {
        configure(false);
        assert_eq!(slide(10, 10, 9), 0);
        configure(true);
    }
''',
    helpers={
        "layout": None,  # filled from Horizon's own source below
        "tick": '''// Once a tick: give the chat module the player's setting.
fn chat_animation_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_animation as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "slide");
    m::configure(on);
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


def _layout_helper():
    """Chat::layout rebuilt from Horizon's own source with each new line offset by the slide."""
    import re
    import os
    from pathlib import Path
    here = Path(__file__).resolve().parent
    root = Path(os.environ.get("HORIZON_SRC") or here.parents[1] / "target/horizon" / REV)
    chat = root / "crates/horizon-client-render/src/chat.rs"
    if not chat.is_file():
        raise SystemExit(f"Horizon {REV[:7]} is not downloaded: run `python3 tools/mods.py fetch` (or set HORIZON_SRC to a checkout of that commit)")
    src = chat.read_text()
    start = src.index("    pub fn layout(&self, height: i32, ticks: u64, focused: bool) -> Vec<GuiDraw> {")
    end = src.index("    /// Draws [`Chat::layout`] into the HUD batch")
    body = src[start:end].rstrip()
    body = body[body.index("{") + 1: body.rindex("}")]
    body = body.replace("self.", "chat.")
    body = body.replace("return Vec::new();", "return std::ops::ControlFlow::Break(Vec::new());")
    body = re.sub(r"\n        out\n$", "\n        std::ops::ControlFlow::Break(out)\n", body.rstrip() + "\n")
    bg = ("        for &(i, _, alpha) in &lines {\n            let bottom = chat_bottom - i as i32 * entry_height;",
          "        for &(i, line, alpha) in &lines {\n            let bottom = chat_bottom - i as i32 * entry_height + lib_chat_animation::slide(ticks, line.added, entry_height);")
    fg = ("        for &(i, line, alpha) in &lines {\n            let bottom = chat_bottom - i as i32 * entry_height;\n            let opacity",
          "        for &(i, line, alpha) in &lines {\n            let bottom = chat_bottom - i as i32 * entry_height + lib_chat_animation::slide(ticks, line.added, entry_height);\n            let opacity")
    assert bg[0] in body and fg[0] in body, "the layout source changed"
    body = body.replace(fg[0], fg[1]).replace(bg[0], bg[1])
    return ("// Chat::layout, rebuilt: the same draw list with each line that was just added still sliding up into its place.\n"
            "fn chat_animation_layout(chat: &Chat, height: i32, ticks: u64, focused: bool) -> std::ops::ControlFlow<Vec<GuiDraw>> {" + body + "}\n")


CHAT[2]["helpers"]["layout"] = _layout_helper()
