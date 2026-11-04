"""Multi Key Bindings: more than one key for an action, by extending how Horizon turns a pressed key into the action's default key. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

MKB = []

COMMAND_BODY = '''// `/multikey add|remove|list`.
fn @ID@_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use horizon_client::keybinds::lib_multi_key_bindings as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    let actions: Vec<&str> = horizon_client::keybinds::MAPPINGS.iter().filter(|m| m.2 != "debug").map(|m| m.0).collect();
    let (lines, save) = m::run(&rest, &actions);
    if let Some(text) = save {
        game.native_set_setting(m::ID, "extras", &text);
    }
    if m::is(|k| game.native_setting(m::ID, k), "say") || save_failed(&lines) {
        for line in lines {
            game.native_chat(&line);
        }
    }
}

fn save_failed(lines: &[String]) -> bool {
    lines.iter().any(|l| l.starts_with("Usage") || l.starts_with("Unknown") || l.starts_with("Not a key"))
}
'''

TRANSLATE = '''// KeyBinds.translate_all, plus the extra keys: a key bound again to an action drives that action's default key too.
fn multi_key_bindings_translate(binds: &KeyBinds, key: KeyCode) -> std::ops::ControlFlow<Vec<Result<KeyCode, MouseButton>>> {
    use lib_multi_key_bindings as m;
    let Some(name) = key_name(key) else {
        return std::ops::ControlFlow::Continue(());
    };
    let extra = m::actions_for(name);
    if extra.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let mut bound: Vec<_> = binds
        .translated_mappings(name)
        .chain(MAPPINGS.iter().filter(|mapping| extra.iter().any(|a| a == mapping.0)).map(|mapping| mapping.0))
        .filter_map(|mapping| {
            let default = KeyBinds::default_of(mapping);
            match key_code(default) {
                Some(code) => Some(Ok(code)),
                None => mouse_button(default).map(Err),
            }
        })
        .collect();
    bound = distinct(bound);
    std::ops::ControlFlow::Break(bound)
}
'''

TRANSLATE_MOUSE = '''// KeyBinds.translate_mouse_all, plus the extra buttons.
fn multi_key_bindings_translate_mouse(binds: &KeyBinds, button: MouseButton) -> std::ops::ControlFlow<Vec<Result<KeyCode, MouseButton>>> {
    use lib_multi_key_bindings as m;
    let name = mouse_name(button);
    let extra = m::actions_for(&name);
    if extra.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let mut bound: Vec<_> = binds
        .translated_mappings(&name)
        .chain(MAPPINGS.iter().filter(|mapping| extra.iter().any(|a| a == mapping.0)).map(|mapping| mapping.0))
        .filter_map(|mapping| {
            let default = KeyBinds::default_of(mapping);
            match mouse_button(default) {
                Some(b) => Some(Err(b)),
                None => key_code(default).map(Ok),
            }
        })
        .collect();
    bound = distinct(bound);
    std::ops::ControlFlow::Break(bound)
}
'''

MKB.append(dict(
    slug="multi-key-bindings", id="multi_key_bindings", name="Multi Key Bindings", title="Multi Key Bindings", raw=True,
    usage="| Command | What it does |\n|---|---|\n| `/multikey add <action> <key>` | Also drives `<action>` (for example `forward`, `jump`, `inventory`) from `<key>` (`up`, `space`, `mouse.4`, `left.control`) |\n| `/multikey remove <action> [key]` | Removes one extra key of an action, or all of them |\n| `/multikey list` | Shows the extra keys |\n\nThe key you add works as well as the action's own key, held or pressed, wherever the action works. The first key stays on the Key Binds page; extra ones are managed here.",
    description="More than one key for the same action.",
    upstream="Multi Key Bindings (multi-key-bindings)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command="multikey", hooks=["translate", "mouse", "tick", "command", "list", "settings", "click"],
    options=[("say", "Say in chat when an extra key changes", True)],
    command_body=COMMAND_BODY,
    modpath="horizon_client::keybinds::lib_multi_key_bindings",
    raw_note=("It lives in the input crate's `keybinds.rs`: cancellable HEAD hooks on `KeyBinds::translate_all` and `translate_mouse_all`, the functions that turn a pressed key or button into the default key of the action it is bound to, "
              "rebuilt with the extra keys added. They are not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. "
              "Extra keys are made with a command, not buttons on the Key Binds page (Horizon draws it in a layout function that cannot be extended), and modifier combinations such as Shift+K are not supported. "
              "It was written from the original's public description, not its source, which is GPL-3.0."),
    toml_text=toml_text("multi_key_bindings", "1.0.0", "crates/horizon-client-input/src/keybinds.rs", "lib_multi_key_bindings", [
        dict(fields=[("id", '"translate"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_input::keybinds::KeyBinds::translate_all"'), ("helper", '"translate.rs"')],
             bindings=[("binds", "&KeyBinds", "&*self"), ("key", "KeyCode", "key")]),
        dict(fields=[("id", '"mouse"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_input::keybinds::KeyBinds::translate_mouse_all"'), ("helper", '"mouse.rs"')],
             bindings=[("binds", "&KeyBinds", "&*self"), ("button", "MouseButton", "button")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False), visibility="pub"),
    runtime_extra=r'''
use std::cell::{Cell, RefCell as StateCell};

/// Slash command.
pub const COMMAND: &str = "multikey";

thread_local! {
    /// (action, key name) pairs.
    static EXTRAS: StateCell<Vec<(String, String)>> = const { StateCell::new(Vec::new()) };
    static LOADED: Cell<bool> = const { Cell::new(false) };
}

pub fn loaded() -> bool {
    LOADED.with(Cell::get)
}

/// The stored extras, `key.forward=key.keyboard.up` per line.
pub fn decode(text: &str) -> Vec<(String, String)> {
    text.lines().filter_map(|l| l.split_once('=')).map(|(a, k)| (a.trim().to_owned(), k.trim().to_owned())).filter(|(a, k)| !a.is_empty() && !k.is_empty()).collect()
}

pub fn encode(extras: &[(String, String)]) -> String {
    extras.iter().map(|(a, k)| format!("{a}={k}")).collect::<Vec<_>>().join("\n")
}

/// Loads the stored extras (once) from the setting's text.
pub fn load(text: Option<String>) {
    if !LOADED.with(|l| l.replace(true)) {
        EXTRAS.with_borrow_mut(|e| *e = decode(&text.unwrap_or_default()));
    }
}

pub fn extras() -> Vec<(String, String)> {
    EXTRAS.with_borrow(Clone::clone)
}

/// The actions an extra key drives, by their vanilla mapping names.
pub fn actions_for(key_name: &str) -> Vec<String> {
    EXTRAS.with_borrow(|e| e.iter().filter(|(_, k)| k == key_name).map(|(a, _)| a.clone()).collect())
}

/// What the player typed as a key (`up`, `space`, `mouse.4`, `key.keyboard.up`) as its vanilla name.
pub fn key_full(typed: &str) -> Option<String> {
    let t = typed.to_lowercase();
    let ok = !t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_');
    if !ok {
        return None;
    }
    Some(if t.starts_with("key.") {
        t
    } else if let Some(button) = t.strip_prefix("mouse.") {
        format!("key.mouse.{button}")
    } else {
        format!("key.keyboard.{t}")
    })
}

/// The mapping name for what the player typed (`forward`, `key.forward`), among `actions`.
pub fn action_full(typed: &str, actions: &[&str]) -> Option<String> {
    let t = typed.to_lowercase();
    let full = if t.starts_with("key.") { t } else { format!("key.{t}") };
    actions.iter().find(|a| a.to_lowercase() == full).map(|a| (*a).to_owned())
}

/// Runs the arguments after `/multikey`: the chat lines, and the text to store when the extras changed.
pub fn run(args: &[&str], actions: &[&str]) -> (Vec<String>, Option<String>) {
    match args {
        ["add", action, key] => {
            let Some(a) = action_full(action, actions) else {
                return (vec![format!("Unknown action `{action}`")], None);
            };
            let Some(k) = key_full(key) else {
                return (vec![format!("Not a key name: `{key}`")], None);
            };
            let text = EXTRAS.with_borrow_mut(|e| {
                if !e.contains(&(a.clone(), k.clone())) {
                    e.push((a.clone(), k.clone()));
                }
                encode(e)
            });
            (vec![format!("{} now also on {}", short(&a), short_key(&k))], Some(text))
        }
        ["remove", action, rest @ ..] => {
            let Some(a) = action_full(action, actions) else {
                return (vec![format!("Unknown action `{action}`")], None);
            };
            let key = rest.first().and_then(|k| key_full(k));
            let (removed, text) = EXTRAS.with_borrow_mut(|e| {
                let before = e.len();
                e.retain(|(x, k)| !(*x == a && key.as_ref().is_none_or(|key| key == k)));
                (before - e.len(), encode(e))
            });
            if removed == 0 { (vec![format!("{} has no extra key", short(&a))], None) } else { (vec![format!("Removed {removed} extra key(s) of {}", short(&a))], Some(text)) }
        }
        ["list"] => {
            let e = extras();
            if e.is_empty() {
                (vec!["No extra keys. /multikey add <action> <key>".to_owned()], None)
            } else {
                (e.iter().map(|(a, k)| format!("{}: {}", short(a), short_key(k))).collect(), None)
            }
        }
        _ => (vec!["Usage: /multikey add <action> <key> | remove <action> [key] | list".to_owned()], None),
    }
}

fn short(action: &str) -> String {
    action.strip_prefix("key.").unwrap_or(action).replace('.', " ")
}

fn short_key(key: &str) -> String {
    key.strip_prefix("key.keyboard.").or_else(|| key.strip_prefix("key.")).unwrap_or(key).replace('.', " ")
}

/// The mod is off: the extra keys do nothing, and are read again when it is turned on.
pub fn forget_all() {
    EXTRAS.with_borrow_mut(Vec::clear);
    LOADED.with(|l| l.set(false));
}

/// Forgets everything (tests).
#[cfg(test)]
pub fn forget() {
    EXTRAS.with_borrow_mut(Vec::clear);
    LOADED.with(|l| l.set(true));
}
''',
    tests=r'''
    const ACTIONS: [&str; 4] = ["key.forward", "key.jump", "key.hotbar.1", "key.inventory"];

    #[test]
    fn typed_names_become_vanilla_names() {
        assert_eq!(key_full("Up"), Some("key.keyboard.up".to_owned()));
        assert_eq!(key_full("left.shift"), Some("key.keyboard.left.shift".to_owned()));
        assert_eq!(key_full("mouse.4"), Some("key.mouse.4".to_owned()));
        assert_eq!(key_full("key.keyboard.f9"), Some("key.keyboard.f9".to_owned()));
        assert_eq!(key_full(""), None);
        assert_eq!(key_full("a b"), None);
        assert_eq!(action_full("Forward", &ACTIONS), Some("key.forward".to_owned()));
        assert_eq!(action_full("hotbar.1", &ACTIONS), Some("key.hotbar.1".to_owned()));
        assert_eq!(action_full("nothing", &ACTIONS), None);
    }

    #[test]
    fn adding_and_removing_extra_keys() {
        forget();
        let (lines, save) = run(&["add", "forward", "up"], &ACTIONS);
        assert_eq!(lines, ["forward now also on up"]);
        assert_eq!(save.as_deref(), Some("key.forward=key.keyboard.up"));
        assert_eq!(actions_for("key.keyboard.up"), ["key.forward"]);
        run(&["add", "forward", "up"], &ACTIONS);
        run(&["add", "jump", "up"], &ACTIONS);
        assert_eq!(actions_for("key.keyboard.up"), ["key.forward", "key.jump"], "no duplicates, and a key can drive two actions");
        let (lines, save) = run(&["remove", "forward", "up"], &ACTIONS);
        assert_eq!(lines, ["Removed 1 extra key(s) of forward"]);
        assert_eq!(save.as_deref(), Some("key.jump=key.keyboard.up"));
        let (lines, save) = run(&["remove", "inventory"], &ACTIONS);
        assert_eq!((lines, save), (vec!["inventory has no extra key".to_owned()], None));
        let (_, save) = run(&["remove", "jump"], &ACTIONS);
        assert_eq!(save.as_deref(), Some(""), "all removed");
        assert!(actions_for("key.keyboard.up").is_empty());
    }

    #[test]
    fn bad_input_changes_nothing() {
        forget();
        assert_eq!(run(&["add", "nonsense", "up"], &ACTIONS).0, ["Unknown action `nonsense`"]);
        assert_eq!(run(&["add", "forward", "a b"], &ACTIONS).0, ["Not a key name: `a b`"]);
        assert!(run(&["add", "forward"], &ACTIONS).0[0].starts_with("Usage"));
        assert!(extras().is_empty());
    }

    #[test]
    fn the_list_and_the_stored_text_round_trip() {
        forget();
        run(&["add", "forward", "up"], &ACTIONS);
        run(&["add", "inventory", "mouse.4"], &ACTIONS);
        assert_eq!(run(&["list"], &ACTIONS).0, ["forward: up", "inventory: mouse 4"]);
        let text = encode(&extras());
        assert_eq!(decode(&text), extras());
        assert_eq!(decode("garbage\n=x\nkey.a=\nkey.b=key.keyboard.k"), vec![("key.b".to_owned(), "key.keyboard.k".to_owned())]);
    }
''',
    helpers={
        "translate": TRANSLATE,
        "mouse": TRANSLATE_MOUSE,
        "tick": '''// Once a tick: the first tick loads the stored extra keys.
fn multi_key_bindings_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::keybinds::lib_multi_key_bindings as m;
    if !game.native_window() {
        return;
    }
    if game.native_on(m::ID) {
        m::load(game.native_setting(m::ID, "extras"));
    } else {
        m::forget_all();
    }
}
''',
    },
))
