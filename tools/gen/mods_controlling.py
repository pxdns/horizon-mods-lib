"""Controlling: find key bindings by action or by key, and list conflicts, from chat. Reads the key bindings; pinned to one Horizon commit."""
CTRL = []

COMMAND_BODY = '''// `/keybinds find|conflicts|unbound`.
fn @ID@_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::@MOD@ as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    let mut bindings: Vec<(String, String)> = horizon_client::keybinds::MAPPINGS
        .iter()
        .map(|(action, ..)| ((*action).to_owned(), game.settings.keys.get(action).to_owned()))
        .collect();
    let request = m::request(&rest);
    if matches!(request, m::Request::Find(_)) && m::is(|k| game.native_setting(m::ID, k), "hide_none") {
        bindings.retain(|(_, key)| m::key_name(key).is_some());
    }
    for line in m::answer(&request, &bindings) {
        game.native_chat(&line);
    }
}
'''

CTRL.append(dict(
    slug="controlling", id="controlling", name="Controlling", title="Controlling", raw=True,
    usage="| Command | What it does |\n|---|---|\n| `/keybinds find <words>` | Bindings whose action name or key has all the words, with the key each is on |\n| `/keybinds conflicts` | Keys that more than one action uses |\n| `/keybinds unbound` | Actions with no key |\n| `/keybinds` | Lists these |\n\nHorizon's own Key Binds page already searches by action name; this finds by key too and lists conflicts.",
    description="Find key bindings by action or by key, and list conflicts and unbound actions.",
    upstream="Controlling (controlling)", upstream_licence="MIT",
    command="keybinds", hooks=["command", "list", "settings", "click"],
    options=[("hide_none", "Leave out unbound actions from find", False)],
    command_body=COMMAND_BODY,
    raw_note=("It reads the key bindings (`Game::settings.keys`) and Horizon's list of mappings, which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. "
              "It is chat commands, not the original's changes to the Key Binds screen, which Horizon draws in a layout function that cannot be extended; its search by action name is already built in."),
    runtime_extra=r'''
use std::collections::BTreeMap;

/// Slash command.
pub const COMMAND: &str = "keybinds";
pub const HELP: [&str; 4] = [
    "/keybinds find <words>: bindings with all the words in the action or the key",
    "/keybinds conflicts: keys used by more than one action",
    "/keybinds unbound: actions with no key",
    "Action and key are shown as in the controls screen.",
];

/// What the player asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Help,
    Find(Vec<String>),
    Conflicts,
    Unbound,
}

pub fn request(args: &[&str]) -> Request {
    match args {
        ["find", words @ ..] if !words.is_empty() => Request::Find(words.iter().map(|w| w.to_lowercase()).collect()),
        ["conflicts"] => Request::Conflicts,
        ["unbound"] => Request::Unbound,
        _ => Request::Help,
    }
}

/// `key.forward` as `forward`, `key.hotbar.3` as `hotbar 3`.
pub fn action_name(action: &str) -> String {
    action.strip_prefix("key.").unwrap_or(action).replace('.', " ")
}

/// `key.keyboard.left.shift` as `left shift`, `key.mouse.left` as `mouse left`, `key.keyboard.unknown` as none.
pub fn key_name(key: &str) -> Option<String> {
    if key == "key.keyboard.unknown" || key.is_empty() {
        return None;
    }
    let bare = key.strip_prefix("key.keyboard.").map(str::to_owned).or_else(|| key.strip_prefix("key.").map(str::to_owned)).unwrap_or_else(|| key.to_owned());
    Some(bare.replace('.', " "))
}

fn line(action: &str, key: &str) -> String {
    format!("{}: {}", action_name(action), key_name(key).unwrap_or_else(|| "none".to_owned()))
}

/// The chat lines answering a request over (action, bound key) pairs.
pub fn answer(request: &Request, bindings: &[(String, String)]) -> Vec<String> {
    match request {
        Request::Help => HELP.map(String::from).to_vec(),
        Request::Find(words) => {
            let found: Vec<String> = bindings
                .iter()
                .filter(|(action, key)| {
                    let hay = format!("{} {}", action_name(action), key_name(key).unwrap_or_default()).to_lowercase();
                    words.iter().all(|w| hay.contains(w.as_str()))
                })
                .map(|(action, key)| line(action, key))
                .collect();
            if found.is_empty() {
                vec![format!("No binding matches `{}`", words.join(" "))]
            } else {
                let mut out = vec![format!("{} bindings:", found.len())];
                out.extend(found.into_iter().take(25));
                out
            }
        }
        Request::Conflicts => {
            let mut by_key: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
            for (action, key) in bindings {
                if key_name(key).is_some() {
                    by_key.entry(key).or_default().push(action);
                }
            }
            let conflicts: Vec<String> = by_key
                .into_iter()
                .filter(|(_, actions)| actions.len() > 1)
                .map(|(key, actions)| format!("{}: {}", key_name(key).unwrap_or_default(), actions.iter().map(|a| action_name(a)).collect::<Vec<_>>().join(", ")))
                .collect();
            if conflicts.is_empty() { vec!["No key is used by two actions".to_owned()] } else { std::iter::once(format!("{} conflicts:", conflicts.len())).chain(conflicts).collect() }
        }
        Request::Unbound => {
            let none: Vec<String> = bindings.iter().filter(|(_, key)| key_name(key).is_none()).map(|(a, _)| action_name(a)).collect();
            if none.is_empty() { vec!["Every action has a key".to_owned()] } else { vec![format!("Unbound: {}", none.join(", "))] }
        }
    }
}
''',
    tests=r'''
    fn binds() -> Vec<(String, String)> {
        [
            ("key.forward", "key.keyboard.w"),
            ("key.back", "key.keyboard.s"),
            ("key.sneak", "key.keyboard.left.shift"),
            ("key.sprint", "key.keyboard.left.shift"),
            ("key.attack", "key.mouse.left"),
            ("key.advancements", "key.keyboard.unknown"),
        ]
        .map(|(a, k)| (a.to_owned(), k.to_owned()))
        .to_vec()
    }

    #[test]
    fn requests_parse() {
        assert_eq!(request(&[]), Request::Help);
        assert_eq!(request(&["find", "Shift"]), Request::Find(vec!["shift".into()]));
        assert_eq!(request(&["find"]), Request::Help);
        assert_eq!(request(&["conflicts"]), Request::Conflicts);
        assert_eq!(request(&["unbound"]), Request::Unbound);
    }

    #[test]
    fn names_are_readable() {
        assert_eq!(action_name("key.hotbar.3"), "hotbar 3");
        assert_eq!(key_name("key.keyboard.left.shift"), Some("left shift".to_owned()));
        assert_eq!(key_name("key.mouse.left"), Some("mouse left".to_owned()));
        assert_eq!(key_name("key.keyboard.unknown"), None);
    }

    #[test]
    fn find_searches_the_action_and_the_key() {
        let by_action = answer(&Request::Find(vec!["sneak".into()]), &binds());
        assert_eq!(by_action, ["1 bindings:", "sneak: left shift"]);
        let by_key = answer(&Request::Find(vec!["shift".into()]), &binds());
        assert_eq!(by_key, ["2 bindings:", "sneak: left shift", "sprint: left shift"]);
        let both = answer(&Request::Find(vec!["mouse".into(), "attack".into()]), &binds());
        assert_eq!(both, ["1 bindings:", "attack: mouse left"]);
        assert_eq!(answer(&Request::Find(vec!["zebra".into()]), &binds()), ["No binding matches `zebra`"]);
    }

    #[test]
    fn conflicts_and_unbound() {
        assert_eq!(answer(&Request::Conflicts, &binds()), ["1 conflicts:", "left shift: sneak, sprint"]);
        assert_eq!(answer(&Request::Conflicts, &binds()[..2]), ["No key is used by two actions"]);
        assert_eq!(answer(&Request::Unbound, &binds()), ["Unbound: advancements"]);
        assert_eq!(answer(&Request::Unbound, &binds()[..2]), ["Every action has a key"]);
    }
''',
))
