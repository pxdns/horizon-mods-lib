// Better Stats, a native Horizon mod: Search, rank and export your statistics from chat: the biggest entries of each kind, a search across all of them, and a text or CSV file.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_better_stats`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "better_stats";
pub const NAME: &str = "Better Stats";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Search, rank and export your statistics from chat: the biggest entries of each kind, a search across all of them, and a text or CSV file.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("csv", "Also write a CSV when exporting", false)
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

use std::collections::BTreeMap;

/// Slash command.
pub const COMMAND: &str = "statistics";
/// The help lines.
pub const HELP: [&str; 4] = [
    "/statistics top <kind> [n]: biggest entries (mined, crafted, used, broken, picked_up, dropped, killed, killed_by, custom)",
    "/statistics find <words>: entries whose name has all the words",
    "/statistics export: write statistics.txt to the mod's folder",
    "Open the Statistics screen first on a server so it sends the latest numbers.",
];

/// One statistic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub kind: String,
    pub id: String,
    pub value: i32,
}

/// What the player asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Help,
    Top(String, usize),
    Find(Vec<String>),
    Export,
}

/// The arguments after `/statistics`.
pub fn request(args: &[&str]) -> Request {
    match args {
        ["top", kind, rest @ ..] => {
            let n = rest.first().and_then(|n| n.parse::<usize>().ok()).unwrap_or(10).clamp(1, 50);
            Request::Top((*kind).to_lowercase(), n)
        }
        ["find", terms @ ..] if !terms.is_empty() => Request::Find(terms.iter().map(|t| t.to_lowercase()).collect()),
        ["export"] => Request::Export,
        _ => Request::Help,
    }
}

/// Every statistic with a non-zero value: `minecraft:mined` -> kind `mined`, `minecraft:stone` -> id `stone`.
pub fn entries(values: &BTreeMap<String, BTreeMap<String, i32>>) -> Vec<Entry> {
    let bare = |s: &str| s.strip_prefix("minecraft:").unwrap_or(s).to_owned();
    values
        .iter()
        .flat_map(|(kind, map)| map.iter().map(move |(id, v)| Entry { kind: bare(kind), id: bare(id), value: *v }))
        .filter(|e| e.value != 0)
        .collect()
}

/// `1234567` as `1,234,567`.
pub fn grouped(n: i64) -> String {
    let digits = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 { format!("-{out}") } else { out }
}

/// `diamond_pickaxe` as `Diamond Pickaxe`.
pub fn title(id: &str) -> String {
    id.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The `n` biggest entries of `kind`, as chat lines.
pub fn top(entries: &[Entry], kind: &str, n: usize) -> Vec<String> {
    let mut of: Vec<&Entry> = entries.iter().filter(|e| e.kind == kind).collect();
    if of.is_empty() {
        return vec![format!("Nothing recorded for `{kind}`. Kinds: mined, crafted, used, broken, picked_up, dropped, killed, killed_by, custom")];
    }
    of.sort_by(|a, b| b.value.cmp(&a.value).then(a.id.cmp(&b.id)));
    let mut out = vec![format!("Top {} {kind}:", n.min(of.len()))];
    out.extend(of.iter().take(n).enumerate().map(|(i, e)| format!("{}. {}: {}", i + 1, title(&e.id), grouped(i64::from(e.value)))));
    out
}

/// Entries whose name has every term, grouped by kind, at most 30 lines.
pub fn find(entries: &[Entry], terms: &[String]) -> Vec<String> {
    let matching: Vec<&Entry> = entries.iter().filter(|e| terms.iter().all(|t| e.id.replace('_', " ").contains(t.as_str()))).collect();
    if matching.is_empty() {
        return vec![format!("No statistic matches `{}`", terms.join(" "))];
    }
    let mut out = vec![format!("{} matches:", matching.len())];
    out.extend(matching.iter().take(30).map(|e| format!("{} {}: {}", e.kind, title(&e.id), grouped(i64::from(e.value)))));
    if matching.len() > 30 {
        out.push(format!("... and {} more", matching.len() - 30));
    }
    out
}

/// Every statistic as `kind<TAB>id<TAB>value` lines under a header, sorted by kind then biggest first.
pub fn export_text(entries: &[Entry]) -> String {
    let mut sorted: Vec<&Entry> = entries.iter().collect();
    sorted.sort_by(|a, b| a.kind.cmp(&b.kind).then(b.value.cmp(&a.value)).then(a.id.cmp(&b.id)));
    let mut out = String::from("kind\tid\tvalue\n");
    for e in sorted {
        out.push_str(&format!("{}\t{}\t{}\n", e.kind, e.id, e.value));
    }
    out
}

/// The same as CSV.
pub fn export_csv(entries: &[Entry]) -> String {
    export_text(entries).replace('\t', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Entry> {
        let e = |kind: &str, id: &str, value| Entry { kind: kind.into(), id: id.into(), value };
        vec![e("mined", "stone", 1500), e("mined", "diamond_ore", 12), e("mined", "dirt", 3000), e("killed", "zombie", 40), e("custom", "jump", 99)]
    }

    #[test]
    fn arguments_parse() {
        assert_eq!(request(&[]), Request::Help);
        assert_eq!(request(&["top", "Mined"]), Request::Top("mined".into(), 10));
        assert_eq!(request(&["top", "mined", "3"]), Request::Top("mined".into(), 3));
        assert_eq!(request(&["top", "mined", "999"]), Request::Top("mined".into(), 50), "capped");
        assert_eq!(request(&["top", "mined", "x"]), Request::Top("mined".into(), 10));
        assert_eq!(request(&["find", "Dia", "ore"]), Request::Find(vec!["dia".into(), "ore".into()]));
        assert_eq!(request(&["find"]), Request::Help);
        assert_eq!(request(&["export"]), Request::Export);
        assert_eq!(request(&["nonsense"]), Request::Help);
    }

    #[test]
    fn numbers_are_grouped() {
        assert_eq!(grouped(0), "0");
        assert_eq!(grouped(999), "999");
        assert_eq!(grouped(1000), "1,000");
        assert_eq!(grouped(1_234_567), "1,234,567");
        assert_eq!(grouped(-4200), "-4,200");
    }

    #[test]
    fn top_ranks_biggest_first() {
        assert_eq!(top(&sample(), "mined", 2), ["Top 2 mined:", "1. Dirt: 3,000", "2. Stone: 1,500"]);
        assert_eq!(top(&sample(), "mined", 10)[0], "Top 3 mined:");
        assert!(top(&sample(), "crafted", 5)[0].starts_with("Nothing recorded for `crafted`"));
    }

    #[test]
    fn find_needs_every_word_and_names_the_kind() {
        let found = find(&sample(), &["ore".into(), "diamond".into()]);
        assert_eq!(found, ["1 matches:", "mined Diamond Ore: 12"]);
        assert_eq!(find(&sample(), &["emerald".into()]), ["No statistic matches `emerald`"]);
    }

    #[test]
    fn entries_drop_zero_and_strip_the_namespace() {
        let mut values = BTreeMap::new();
        values.insert("minecraft:mined".to_owned(), BTreeMap::from([("minecraft:stone".to_owned(), 5), ("minecraft:dirt".to_owned(), 0)]));
        assert_eq!(entries(&values), vec![Entry { kind: "mined".into(), id: "stone".into(), value: 5 }]);
    }

    #[test]
    fn export_is_sorted_and_csv_follows() {
        let text = export_text(&sample());
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "kind\tid\tvalue");
        assert_eq!(lines[1], "custom\tjump\t99");
        assert_eq!(lines[2], "killed\tzombie\t40");
        assert_eq!(lines[3], "mined\tdirt\t3000", "biggest first within a kind");
        assert_eq!(export_csv(&sample()).lines().nth(1), Some("custom,jump,99"));
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
