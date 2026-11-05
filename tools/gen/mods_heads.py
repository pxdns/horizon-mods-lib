"""Chat Heads: the sender's face in front of each chat line, through Horizon's inline player heads. Pinned to one Horizon commit."""
from mods_chat import toml_text, common_native

HEADS = []

HEADS.append(dict(
    slug="chat-heads", id="chat_heads", name="Chat Heads", title="Chat Heads", raw=True,
    usage="Nothing to press: a chat line from a player starts with that player's face, the way a skin shows in the player list. It finds the sender in the usual `<Name> message` form, and in `Name: message` or `Name » message` forms for players who are online. The settings page switches the hat layer and the other forms off.",
    description="Shows the sender's face in front of each chat line.",
    upstream="Chat Heads (chat_heads)", upstream_licence="MPL-2.0",
    command=None, hooks=["add", "list", "settings", "click", "tick"],
    options=[("hat", "Show the hat layer", True), ("others", "Also find senders in Name: and Name » forms", True)],
    modpath="horizon_client::chat::lib_chat_heads",
    raw_note=("It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::add_message` that runs after Chat Patches (priority 240) and adds Horizon's own inline player head (the text component `{\"object\":\"player\"}` Horizon already draws in the player list and in text) in front of the line, then calls `add_message` again. "
              "It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. The head is only as good as the skin Horizon has for that name: it uses the default skin until the player list has loaded the real one. "
              "The sender is found from the text, as the original does for servers that do not sign chat."),
    toml_text=toml_text("chat_heads", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_chat_heads", [
        dict(fields=[("id", '"add"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::Chat::add_message"'), ("helper", '"add.rs"')],
             bindings=[("chat", "&mut Chat", "&mut *self"), ("font", "&crate::font::Font", "font"), ("content", "&Text", "&content"),
                       ("tag", "&Option<std::sync::Arc<Tag>>", "&tag"), ("source", "ChatSource", "source"), ("ticks", "u64", "ticks"), ("focused", "bool", "focused")]),
    ] + common_native(), visibility="pub", priority=240),
    runtime_extra=r'''
use std::cell::{Cell, RefCell as StateCell};
use std::collections::BTreeSet;

/// How far into a line the sender's name may be.
pub const REACH: usize = 48;

thread_local! {
    static HAT: Cell<bool> = const { Cell::new(true) };
    static OTHERS: Cell<bool> = const { Cell::new(true) };
    static ENABLED: Cell<bool> = const { Cell::new(true) };
    /// The names of the players on the server, lower case.
    static ONLINE: StateCell<BTreeSet<String>> = const { StateCell::new(BTreeSet::new()) };
    /// True while the mod adds a message again with its head.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// Sets what the mod does (the game tells the chat each tick).
pub fn configure(enabled: bool, hat: bool, others: bool) {
    ENABLED.with(|v| v.set(enabled));
    HAT.with(|v| v.set(hat));
    OTHERS.with(|v| v.set(others));
}

/// The names on the server (told once a second), lower case.
pub fn set_online(online: BTreeSet<String>) {
    ONLINE.with_borrow_mut(|o| *o = online);
}

pub fn enabled() -> bool {
    ENABLED.with(Cell::get)
}

pub fn hat() -> bool {
    HAT.with(Cell::get)
}

/// Marks the nested call; false when this is the nested one.
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

/// A name a player can have: 1 to 16 letters, digits or underscores.
pub fn valid(name: &str) -> bool {
    (1..=16).contains(&name.len()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The sender of a chat line: `<Name> hi`, anywhere in the first characters (after a prefix such as `[VIP]`), or, when the
/// player is online, `Name: hi` or `Name » hi`. The name as written.
pub fn sender(line: &str) -> Option<String> {
    let head: String = line.chars().take(REACH).collect();
    if let Some(open) = head.find('<')
        && let Some(close) = head[open + 1..].find('>')
    {
        let name = &head[open + 1..open + 1 + close];
        if valid(name) {
            return Some(name.to_owned());
        }
    }
    if !OTHERS.with(Cell::get) {
        return None;
    }
    // `[VIP] Name: hi`, `Name » hi`: the last word before the separator.
    let separator = head.find(": ").or_else(|| head.find(" \u{bb} "))?;
    let name = head[..separator].rsplit([' ', ']']).next()?;
    (valid(name) && ONLINE.with_borrow(|o| o.contains(&name.to_lowercase()))).then(|| name.to_owned())
}

/// The text component Horizon draws as a head.
pub fn head_json(name: &str, hat: bool) -> String {
    format!("{{\"object\":\"player\",\"player\":{{\"name\":\"{name}\"}},\"hat\":{hat}}}")
}
''',
    tests=r'''
    fn online(names: &[&str]) {
        configure(true, true, true);
        set_online(names.iter().map(|n| n.to_lowercase()).collect());
    }

    #[test]
    fn the_usual_form_is_found() {
        online(&[]);
        assert_eq!(sender("<Steve> hello"), Some("Steve".to_owned()));
        assert_eq!(sender("[VIP] <Alex_99> hi there"), Some("Alex_99".to_owned()));
        assert_eq!(sender("<bad name> hi"), None);
        assert_eq!(sender("<> hi"), None);
        assert_eq!(sender("Server restarting in 5 minutes"), None);
    }

    #[test]
    fn other_forms_need_the_player_to_be_online() {
        online(&["steve"]);
        assert_eq!(sender("Steve: hello"), Some("Steve".to_owned()));
        assert_eq!(sender("[Mod] Steve \u{bb} hello"), Some("Steve".to_owned()));
        assert_eq!(sender("Notch: hello"), None, "not online");
        configure(true, true, false);
        assert_eq!(sender("Steve: hello"), None, "switched off");
        assert_eq!(sender("<Steve> hello"), Some("Steve".to_owned()), "the usual form always counts");
        online(&[]);
    }

    #[test]
    fn a_name_far_into_a_line_is_not_a_sender() {
        online(&[]);
        let line = format!("{}<Steve> hi", "x".repeat(REACH));
        assert_eq!(sender(&line), None);
    }

    #[test]
    fn names_are_valid_minecraft_names() {
        assert!(valid("Steve") && valid("a_b9") && valid("1234567890123456"));
        assert!(!valid("") && !valid("12345678901234567") && !valid("a b") && !valid("a-b"));
    }

    #[test]
    fn the_head_component_is_json() {
        let json = head_json("Steve", false);
        assert_eq!(json, r#"{"object":"player","player":{"name":"Steve"},"hat":false}"#);
    }

    #[test]
    fn the_nested_call_is_not_decorated_again() {
        leave();
        assert!(enter());
        assert!(!enter());
        leave();
    }
''',
    helpers={
        "add": '''// A new chat line from a player: shown again with the player's face in front of it.
fn chat_heads_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    content: &Text,
    tag: &Option<std::sync::Arc<Tag>>,
    source: ChatSource,
    ticks: u64,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    use lib_chat_heads as m;
    if !m::enabled() || !m::enter() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(name) = m::sender(&content.plain()) else {
        m::leave();
        return std::ops::ControlFlow::Continue(());
    };
    let json: serde_json::Value = serde_json::from_str(&m::head_json(&name, m::hat())).unwrap_or(serde_json::Value::Null);
    let mut shown = crate::component::to_text(&json, &mut |_| Text::empty());
    shown.push(Text::literal(" "));
    shown.push(content.clone());
    chat.add_message(font, shown, tag.clone(), source, ticks, focused);
    m::leave();
    std::ops::ControlFlow::Break(())
}
''',
        "tick": '''// Once a tick: tell the chat what the player switched on and who is online.
fn chat_heads_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_heads as m;
    if !game.native_window() {
        return;
    }
    let (on, hat, others) = {
        let get = |k: &str| game.native_setting(m::ID, k);
        (game.native_on(m::ID), m::is(get, "hat"), m::is(get, "others"))
    };
    m::configure(on, hat, others);
    if on && game.ticks % 20 == 0 {
        let online = game
            .remote_tab_players()
            .map(|(players, ..)| players.iter().map(|p| p.name.to_lowercase()).collect())
            .unwrap_or_default();
        m::set_online(online);
    }
}
''',
    },
))
