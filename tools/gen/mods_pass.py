"""Password Hider: masks the password in /login, /register and the like while you type it, and keeps it out of the chat history and its file."""
from mods_chat import toml_text, native, G, common_native

PASS = []

PASS.append(dict(
    slug="password-hider", id="password_hider", name="Password Hider", title="Password Hider", raw=True,
    usage="Nothing to press. Type `/login hunter2` or `/register hunter2 hunter2` and the password shows as `*******` in the chat box. Pressing the up arrow never brings it back, and it is not written to Horizon's saved command history. The commands covered are `/login`, `/l`, `/log`, `/register`, `/reg`, `/changepassword`, `/cp`, `/unregister` and `/unreg`.",
    description="Shows the password in /login, /register and /changepassword as asterisks while you type, and keeps it out of the chat history and its file.",
    upstream="the password-hiding in login plugins' clients (an idea, not a port)", upstream_licence="independent implementation, no upstream code",
    command=None, hooks=["format", "recent", "list", "settings", "click", "tick"],
    options=[("input", "Mask the password while typing", True), ("history", "Keep it out of the history and its file", True)],
    modpath="horizon_client::chat::lib_password_hider",
    raw_note=("It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `ChatScreen::format`, the text formatter of the chat box, and one on `Chat::add_recent`, which "
              "stores a sent line in the history. Neither is part of the stable hook-point contract, so it is **pinned to one Horizon commit**. It only changes what is shown and saved on your computer: the server still receives "
              "the real command, as it must. `HORIZON_NET_DEBUG=1` logs every packet the game sends, passwords included; leave it off when you type one."),
    toml_text=toml_text("password_hider", "1.0.0", "crates/horizon-client-render/src/chat.rs", "lib_password_hider", [
        dict(fields=[("id", '"format"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::ChatScreen::format"'), ("helper", '"format.rs"')],
             bindings=[("screen", "&ChatScreen", "&*self"), ("value", "&str", "value"), ("offset", "usize", "offset")]),
        dict(fields=[("id", '"recent"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::chat::Chat::add_recent"'), ("helper", '"recent.rs"')],
             bindings=[("message", "&str", "message")]),
    ] + common_native(), visibility="pub"),
    runtime_extra=r'''
use std::cell::Cell;

/// The commands whose arguments are secret, the common ones of login plugins (AuthMe and its relatives).
pub const COMMANDS: [&str; 9] = ["login", "l", "log", "register", "reg", "changepassword", "cp", "unregister", "unreg"];

thread_local! {
    static INPUT: Cell<bool> = const { Cell::new(true) };
    static HISTORY: Cell<bool> = const { Cell::new(true) };
}

/// Sets what the mod does, from the player's settings (the game tells the chat each tick).
pub fn configure(input: bool, history: bool) {
    INPUT.with(|v| v.set(input));
    HISTORY.with(|v| v.set(history));
}

pub fn input_on() -> bool {
    INPUT.with(Cell::get)
}

pub fn history_on() -> bool {
    HISTORY.with(Cell::get)
}

/// The character index where the secret part of `text` starts: just after the first space of a covered command,
/// or none when it is not one or has no space yet.
pub fn secret_start(text: &str) -> Option<usize> {
    let rest = text.strip_prefix('/')?;
    let word_end = rest.find(' ')?;
    let word = rest[..word_end].to_ascii_lowercase();
    COMMANDS.contains(&word.as_str()).then(|| text[..word_end + 2].chars().count())
}

/// Whether `text` is a covered command with something after it, that is, has a secret to hide.
pub fn has_secret(text: &str) -> bool {
    secret_start(text).is_some_and(|from| text.chars().skip(from).any(|c| !c.is_whitespace()))
}

/// `value`, the piece of the box that starts `offset` characters in, with every letter from character `from` on shown as `*`
/// (spaces stay, so you can still see how many passwords you typed).
pub fn mask(value: &str, from: usize, offset: usize) -> String {
    value.chars().enumerate().map(|(i, c)| if offset + i >= from && !c.is_whitespace() { '*' } else { c }).collect()
}
''',
    tests=r'''
    #[test]
    fn the_secret_starts_after_the_first_space_of_a_covered_command() {
        assert_eq!(secret_start("/login hunter2"), Some(7));
        assert_eq!(secret_start("/LOGIN hunter2"), Some(7), "case does not matter");
        assert_eq!(secret_start("/register a b"), Some(10));
        assert_eq!(secret_start("/l pw"), Some(3));
        assert_eq!(secret_start("/login"), None, "no space yet");
        assert_eq!(secret_start("login hunter2"), None, "not a command");
        assert_eq!(secret_start("/msg bob hi"), None);
        assert_eq!(secret_start("/loginx pw"), None, "only the exact words");
        assert_eq!(secret_start("hello /login x"), None);
    }

    #[test]
    fn only_a_command_with_something_after_it_has_a_secret() {
        assert!(has_secret("/login hunter2"));
        assert!(has_secret("/changepassword old new"));
        assert!(!has_secret("/login "));
        assert!(!has_secret("/login    "));
        assert!(!has_secret("/say hunter2"));
    }

    #[test]
    fn masking_keeps_spaces_and_the_command() {
        assert_eq!(mask("/login hunter2", 7, 0), "/login *******");
        assert_eq!(mask("/register abc def", 10, 0), "/register *** ***");
        // The box draws a window of the text: the piece starting 4 characters in.
        assert_eq!(mask("gin hunter2", 7, 4), "gin *******");
        assert_eq!(mask("hunter2", 7, 7), "*******");
        assert_eq!(mask("/login hunter2", 99, 0), "/login hunter2", "nothing masked before the secret");
    }

    #[test]
    fn multibyte_letters_are_counted_as_characters() {
        assert_eq!(secret_start("/login pässwörd"), Some(7));
        assert_eq!(mask("/login pässwörd", 7, 0), "/login ********");
    }

    #[test]
    fn the_switches_work() {
        configure(false, false);
        assert!(!input_on() && !history_on());
        configure(true, true);
        assert!(input_on() && history_on());
    }
''',
    helpers={
        "format": '''// The chat box draws its text through this formatter: a covered command's password shows as asterisks.
fn password_hider_format(screen: &ChatScreen, value: &str, offset: usize) -> std::ops::ControlFlow<Text> {
    use lib_password_hider as m;
    if !m::input_on() {
        return std::ops::ControlFlow::Continue(());
    }
    match m::secret_start(&screen.input.value) {
        Some(from) => std::ops::ControlFlow::Break(Text::literal(m::mask(value, from, offset))),
        None => std::ops::ControlFlow::Continue(()),
    }
}
''',
        "recent": '''// A sent line goes into the history (and, for a command, the history file): not one with a password in it.
fn password_hider_recent(message: &str) -> std::ops::ControlFlow<()> {
    use lib_password_hider as m;
    if m::history_on() && m::has_secret(message) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
''',
        "tick": '''// Once a tick: tell the chat what the player switched on.
fn password_hider_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_password_hider as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID);
    let get = |k: &str| game.native_setting(m::ID, k);
    m::configure(on && m::is(get, "input"), on && m::is(get, "history"));
}
''',
    },
))
