// Tiers, a native Horizon mod: Look up a player's tiers on a tier website with /tier. The website interface is a guess; lookups are off until you switch them on.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_tiers`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "tiers";
pub const NAME: &str = "Tiers";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Look up a player's tiers on a tier website with /tier. The website interface is a guess; lookups are off until you switch them on.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("lookups", "Allow lookups (sends names to the website)", false)
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

use std::sync::Mutex;

/// Slash command.
pub const COMMAND: &str = "tier";
/// A guess at the website's interface: `{name}` is replaced by the player's name.
pub const DEFAULT_URL: &str = "https://pvptiers.com/api/profile/{name}";
/// A reply is kept for this many seconds before the website is asked again.
pub const CACHE_SECONDS: u64 = 120;
pub const HELP: [&str; 3] = [
    "/tier <name>: a player's tiers (sends the name to the website; needs lookups switched on)",
    "/tier me: your own",
    "/tier url <template>: where to ask, https:// with {name} in it",
];

/// JSON, as much as the mod needs of it.
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    fn num(&self) -> Option<i64> {
        match self {
            Json::Num(n) if n.is_finite() => Some(*n as i64),
            Json::Str(s) => s.trim().trim_start_matches(['H', 'L', 'T', 'h', 'l', 't']).parse().ok(),
            _ => None,
        }
    }
}

/// One gamemode of a player.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub mode: String,
    pub tier: i64,
    /// 0 or 1 where the website splits a tier into high and low.
    pub pos: Option<i64>,
    pub peak: Option<i64>,
}

const MODE_KEYS: [&str; 4] = ["gamemode", "mode", "kit", "name"];
const LIST_KEYS: [&str; 4] = ["rankings", "tiers", "gamemodes", "modes"];

fn entry_of(mode: &str, v: &Json) -> Option<Entry> {
    Some(Entry { mode: mode.to_owned(), tier: v.get("tier")?.num()?, pos: v.get("pos").and_then(Json::num), peak: v.get("peak_tier").or_else(|| v.get("peak")).and_then(Json::num) })
}

/// The gamemodes found in a reply: under `rankings`/`tiers`/`gamemodes`, as an object keyed by mode or a list of objects naming theirs;
/// failing that, any object in the reply that has a `tier` number.
pub fn extract(reply: &Json) -> Vec<Entry> {
    for key in LIST_KEYS {
        if let Some(found) = reply.get(key) {
            let list = collect(found);
            if !list.is_empty() {
                return list;
            }
        }
    }
    collect(reply)
}

fn collect(v: &Json) -> Vec<Entry> {
    let mut out = Vec::new();
    match v {
        Json::Obj(fields) => {
            if v.get("tier").is_some() {
                let mode = MODE_KEYS.iter().find_map(|k| match v.get(k) {
                    Some(Json::Str(s)) => Some(s.clone()),
                    _ => None,
                });
                out.extend(entry_of(&mode.unwrap_or_else(|| "tier".to_owned()), v));
            } else {
                for (key, child) in fields {
                    match child {
                        Json::Obj(_) if child.get("tier").is_some() => out.extend(entry_of(key, child)),
                        Json::Obj(_) | Json::Arr(_) => out.extend(collect(child)),
                        _ => {}
                    }
                }
            }
        }
        Json::Arr(items) => items.iter().for_each(|i| out.extend(collect(i))),
        _ => {}
    }
    out
}

/// `tier 3 (high, peak 2)`.
pub fn describe(e: &Entry) -> String {
    let mut s = format!("tier {}", e.tier);
    let mut extra = Vec::new();
    match e.pos {
        Some(0) => extra.push("high".to_owned()),
        Some(1) => extra.push("low".to_owned()),
        _ => {}
    }
    if let Some(p) = e.peak {
        extra.push(format!("peak {p}"));
    }
    if !extra.is_empty() {
        s.push_str(&format!(" ({})", extra.join(", ")));
    }
    s
}

/// The chat lines for a reply: the best tier (lowest number) first.
pub fn lines(name: &str, reply: &Json) -> Vec<String> {
    let mut entries = extract(reply);
    if entries.is_empty() {
        return vec![format!("{name}: no tiers found in the reply")];
    }
    entries.sort_by(|a, b| a.tier.cmp(&b.tier).then(a.pos.cmp(&b.pos)).then(a.mode.cmp(&b.mode)));
    std::iter::once(format!("{name}: {} gamemodes", entries.len())).chain(entries.iter().take(20).map(|e| format!("{}: {}", e.mode, describe(e)))).collect()
}

/// What the player asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Help,
    Lookup(String),
    SetUrl(String),
}

pub fn request(args: &[&str], me: &str) -> Request {
    match args {
        ["me"] => Request::Lookup(me.to_owned()),
        ["url", template] => Request::SetUrl((*template).to_owned()),
        [name] if valid_name(name) => Request::Lookup((*name).to_owned()),
        _ => Request::Help,
    }
}

/// A Minecraft name: 1 to 16 letters, digits or underscores. Nothing else is ever put in a web address.
pub fn valid_name(name: &str) -> bool {
    (1..=16).contains(&name.len()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The address to ask must be https and take the name.
pub fn check_url(template: &str) -> Result<(), String> {
    if !template.starts_with("https://") {
        return Err("The address must start with https://".to_owned());
    }
    if !template.contains("{name}") {
        return Err("The address must contain {name}".to_owned());
    }
    if template.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err("The address cannot contain spaces".to_owned());
    }
    Ok(())
}

pub fn fill(template: &str, name: &str) -> String {
    template.replace("{name}", name)
}

/// `https://pvptiers.com/api/profile/{name}` as `pvptiers.com`.
pub fn host_of(url: &str) -> String {
    url.trim_start_matches("https://").split(['/', '?', '#']).next().unwrap_or("").to_owned()
}

type Reply = (String, Result<Vec<String>, String>);
static REPLIES: Mutex<Vec<Reply>> = Mutex::new(Vec::new());
static CACHE: Mutex<Vec<(String, u64, Vec<String>)>> = Mutex::new(Vec::new());

/// A lookup finished (on its own thread): queue the answer for the game thread.
pub fn deliver(name: String, reply: Result<Json, String>) {
    let result = reply.map(|json| lines(&name, &json));
    if let Ok(mut q) = REPLIES.lock() {
        q.push((name, result));
    }
}

/// On the game thread: the finished lookups as chat lines, remembered for `CACHE_SECONDS`.
pub fn take_replies(now: u64) -> Vec<String> {
    let done: Vec<Reply> = REPLIES.lock().map(|mut q| std::mem::take(&mut *q)).unwrap_or_default();
    let mut out = Vec::new();
    for (name, result) in done {
        match result {
            Ok(lines) => {
                if let Ok(mut cache) = CACHE.lock() {
                    cache.retain(|(n, ..)| *n != name);
                    cache.push((name, now, lines.clone()));
                }
                out.extend(lines);
            }
            Err(why) => out.push(format!("{name}: lookup failed: {why}")),
        }
    }
    out
}

/// A reply asked for within `CACHE_SECONDS`.
pub fn cached(name: &str, now: u64) -> Option<Vec<String>> {
    CACHE.lock().ok()?.iter().find(|(n, at, _)| n == name && now.saturating_sub(*at) < CACHE_SECONDS).map(|(.., lines)| lines.clone())
}

/// Forgets everything (tests).
#[cfg(test)]
pub fn forget() {
    REPLIES.lock().unwrap().clear();
    CACHE.lock().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(fields: Vec<(&str, Json)>) -> Json {
        Json::Obj(fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
    }

    fn mode(tier: f64, pos: f64) -> Json {
        obj(vec![("tier", Json::Num(tier)), ("pos", Json::Num(pos)), ("peak_tier", Json::Num(tier - 1.0))])
    }

    #[test]
    fn a_rankings_object_keyed_by_mode_is_read() {
        let reply = obj(vec![("name", Json::Str("Steve".into())), ("rankings", obj(vec![("sword", mode(3.0, 0.0)), ("axe", mode(2.0, 1.0))]))]);
        assert_eq!(lines("Steve", &reply), ["Steve: 2 gamemodes", "axe: tier 2 (low, peak 1)", "sword: tier 3 (high, peak 2)"]);
    }

    #[test]
    fn a_list_of_objects_naming_their_mode_is_read() {
        let reply = obj(vec![("tiers", Json::Arr(vec![obj(vec![("gamemode", Json::Str("crystal".into())), ("tier", Json::Num(1.0))]), obj(vec![("kit", Json::Str("uhc".into())), ("tier", Json::Str("LT4".into()))])]))]);
        let found = extract(&reply);
        assert_eq!(found.len(), 2);
        assert_eq!((found[0].mode.as_str(), found[0].tier), ("crystal", 1));
        assert_eq!((found[1].mode.as_str(), found[1].tier), ("uhc", 4), "LT4 reads as 4");
    }

    #[test]
    fn any_nested_object_with_a_tier_counts_as_a_fallback() {
        let reply = obj(vec![("data", obj(vec![("profile", obj(vec![("mace", mode(5.0, 1.0))]))]))]);
        assert_eq!(extract(&reply).len(), 1);
        assert_eq!(extract(&reply)[0].mode, "mace");
        assert_eq!(lines("X", &obj(vec![("hello", Json::Str("world".into()))])), ["X: no tiers found in the reply"]);
        assert!(extract(&Json::Null).is_empty());
    }

    #[test]
    fn requests_and_names_are_checked() {
        assert_eq!(request(&[], "Me"), Request::Help);
        assert_eq!(request(&["me"], "Alex"), Request::Lookup("Alex".into()));
        assert_eq!(request(&["Steve"], "Alex"), Request::Lookup("Steve".into()));
        assert_eq!(request(&["../etc/passwd"], "Alex"), Request::Help, "a name is never put in an address unless it is a name");
        assert_eq!(request(&["a?b=c"], "Alex"), Request::Help);
        assert_eq!(request(&["url", "https://x.example/{name}"], "A"), Request::SetUrl("https://x.example/{name}".into()));
        assert!(valid_name("Steve_9") && !valid_name("") && !valid_name("12345678901234567") && !valid_name("a b"));
    }

    #[test]
    fn the_address_must_be_https_and_take_the_name() {
        assert!(check_url("https://pvptiers.com/api/profile/{name}").is_ok());
        assert!(check_url("http://x.example/{name}").unwrap_err().contains("https"));
        assert!(check_url("https://x.example/").unwrap_err().contains("{name}"));
        assert!(check_url("https://x.example/{name} z").unwrap_err().contains("spaces"));
        assert_eq!(fill(DEFAULT_URL, "Steve"), "https://pvptiers.com/api/profile/Steve");
        assert_eq!(host_of(DEFAULT_URL), "pvptiers.com");
        assert_eq!(host_of("https://a.b:8080/x?y"), "a.b:8080");
    }

    #[test]
    fn replies_are_queued_cached_and_expire() {
        forget();
        let ok = obj(vec![("rankings", obj(vec![("sword", mode(3.0, 0.0))]))]);
        deliver("Steve".into(), Ok(ok));
        deliver("Bob".into(), Err("timed out".into()));
        let out = take_replies(100);
        assert_eq!(out, ["Steve: 1 gamemodes", "sword: tier 3 (high, peak 2)", "Bob: lookup failed: timed out"]);
        assert!(take_replies(101).is_empty());
        assert_eq!(cached("Steve", 100 + CACHE_SECONDS - 1).unwrap().len(), 2);
        assert!(cached("Steve", 100 + CACHE_SECONDS).is_none());
        assert!(cached("Bob", 100).is_none(), "failures are not cached");
        forget();
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(
            labels(none).len(),
            OPTIONS.len(),
            "one button per option"
        );
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
