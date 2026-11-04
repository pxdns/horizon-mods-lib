"""Chat Patches: timestamps in front of chat lines and an option to drop immediate repeats. Pinned to one Horizon commit."""
from mods_chat import toml_text, common_native

PATCH = []

PATCH.append(dict(
    slug="chat-patches", id="chat_patches", name="Chat Patches", title="Chat Patches", raw=True,
    usage="Nothing to press: every chat line starts with a gray `[14:32]` (your local time). The settings page adds seconds, and can drop a message that repeats the previous one within five seconds.",
    description="Timestamps in front of chat lines, and an option to drop immediate repeats.",
    upstream="Chat Patches (chatpatches)", upstream_licence="see the upstream project",
    command=None, hooks=["add", "list", "settings", "click", "tick"],
    options=[("timestamps", "Timestamps", True), ("seconds", "Seconds in the timestamp", False), ("repeats", "Drop immediate repeats", False)],
    modpath="horizon_client::chat::lib_chat_patches",
    raw_note=("It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::add_message` that runs before the other chat mods' (priority 250), adds the stamp and calls `add_message` again, so Chat Notify and More Chat History see each line once. "
              "It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. It covers timestamps and repeats only; the original's other features (copying, hiding, compact counters) are not included."),
    toml_text=toml_text("chat_patches", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_chat_patches", [
        dict(fields=[("id", '"add"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::Chat::add_message"'), ("helper", '"add.rs"')],
             bindings=[("chat", "&mut Chat", "&mut *self"), ("font", "&crate::font::Font", "font"), ("content", "&Text", "&content"),
                       ("indicator", "Option<u32>", "indicator"), ("source", "ChatSource", "source"), ("ticks", "u64", "ticks"), ("focused", "bool", "focused")]),
    ] + common_native(), visibility="pub", priority=250),
    runtime_extra=r'''
use std::cell::Cell;

/// A message repeating the last one within this many ticks (five seconds) is a repeat.
pub const REPEAT_TICKS: u64 = 100;

thread_local! {
    static STAMPS: Cell<bool> = const { Cell::new(true) };
    static SECONDS: Cell<bool> = const { Cell::new(false) };
    static REPEATS: Cell<bool> = const { Cell::new(false) };
    /// True while the mod adds a message again with its stamp.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
    static LAST: RefCell<Option<(String, u64)>> = const { RefCell::new(None) };
}

/// Sets what the mod does, from the player's settings (the game tells the chat each tick).
pub fn configure(stamps: bool, seconds: bool, repeats: bool) {
    STAMPS.with(|v| v.set(stamps));
    SECONDS.with(|v| v.set(seconds));
    REPEATS.with(|v| v.set(repeats));
}

pub fn stamps_on() -> bool {
    STAMPS.with(Cell::get)
}

pub fn seconds_on() -> bool {
    SECONDS.with(Cell::get)
}

pub fn repeats_on() -> bool {
    REPEATS.with(Cell::get)
}

/// `[14:32]` or `[14:32:07]`, with a trailing space.
pub fn stamp(hour: u32, minute: u32, second: u32, with_seconds: bool) -> String {
    if with_seconds { format!("[{hour:02}:{minute:02}:{second:02}] ") } else { format!("[{hour:02}:{minute:02}] ") }
}

/// Marks the nested call; false when the call is the nested one.
pub fn enter() -> bool {
    if INSIDE.with(Cell::get) {
        return false;
    }
    INSIDE.with(|i| i.set(true));
    true
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// Whether `text` at `ticks` repeats the previous message, and remembers it as the previous one either way.
pub fn repeat(text: &str, ticks: u64) -> bool {
    LAST.with_borrow_mut(|last| {
        let same = last.as_ref().is_some_and(|(t, at)| t == text && ticks.saturating_sub(*at) <= REPEAT_TICKS);
        *last = Some((text.to_owned(), ticks));
        same
    })
}

/// Forgets the previous message (tests).
#[cfg(test)]
pub fn forget() {
    LAST.with_borrow_mut(|l| *l = None);
}
''',
    tests=r'''
    #[test]
    fn stamps_are_zero_padded() {
        assert_eq!(stamp(9, 5, 3, false), "[09:05] ");
        assert_eq!(stamp(14, 32, 7, true), "[14:32:07] ");
    }

    #[test]
    fn the_nested_call_is_not_stamped_again() {
        leave();
        assert!(enter());
        assert!(!enter(), "nested");
        leave();
        assert!(enter());
        leave();
    }

    #[test]
    fn a_repeat_is_the_same_text_soon_after() {
        forget();
        assert!(!repeat("hello", 10), "the first one is new");
        assert!(repeat("hello", 50), "same text, 2 seconds later");
        assert!(!repeat("bye", 60));
        assert!(!repeat("hello", 70), "something else came between");
        assert!(!repeat("hello", 70 + REPEAT_TICKS + 1), "too long ago");
        forget();
    }

    #[test]
    fn the_switches_work() {
        configure(false, true, true);
        assert!(!stamps_on() && seconds_on() && repeats_on());
        configure(true, false, false);
        assert!(stamps_on() && !seconds_on() && !repeats_on());
    }
''',
    helpers={
        "add": '''// A new chat line: dropped when it repeats the last one, else shown again with a stamp in front.
fn chat_patches_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    content: &Text,
    indicator: Option<u32>,
    source: ChatSource,
    ticks: u64,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    use lib_chat_patches as m;
    if !m::stamps_on() && !m::repeats_on() {
        return std::ops::ControlFlow::Continue(());
    }
    if !m::enter() {
        return std::ops::ControlFlow::Continue(());
    }
    let plain = content.plain();
    if m::repeats_on() && m::repeat(&plain, ticks) {
        m::leave();
        return std::ops::ControlFlow::Break(());
    }
    let mut shown = content.clone();
    if m::stamps_on() {
        use chrono::Timelike;
        let now = chrono::Local::now();
        let mut stamped = Text::empty();
        stamped.push(Text::styled(m::stamp(now.hour(), now.minute(), now.second(), m::seconds_on()), Style::color(text::GRAY)));
        stamped.push(content.clone());
        shown = stamped;
    }
    chat.add_message(font, shown, indicator, source, ticks, focused);
    m::leave();
    std::ops::ControlFlow::Break(())
}
''',
        "tick": '''// Once a tick: tell the chat what the player switched on.
fn chat_patches_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_patches as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID);
    let get = |k: &str| game.native_setting(m::ID, k);
    m::configure(on && m::is(get, "timestamps"), m::is(get, "seconds"), on && m::is(get, "repeats"));
}
''',
    },
))
