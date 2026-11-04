"""Better Statistics: /statistics top, find and export over Horizon's own statistics. Reads game state; pinned to one Horizon commit."""
BSTATS = []

COMMAND_BODY = '''// `/statistics top|find|export`: reads the statistics Horizon keeps.
fn @ID@_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::@MOD@ as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    game.load_stats();
    let entries = m::entries(&game.stats.values);
    match m::request(&rest) {
        m::Request::Help => {
            for line in m::HELP {
                game.native_chat(line);
            }
        }
        m::Request::Top(kind, n) => {
            for line in m::top(&entries, &kind, n) {
                game.native_chat(&line);
            }
        }
        m::Request::Find(terms) => {
            for line in m::find(&entries, &terms) {
                game.native_chat(&line);
            }
        }
        m::Request::Export => {
            let dir = game.native_data_dir(m::ID);
            let csv = m::is(|k| game.native_setting(m::ID, k), "csv");
            let result = std::fs::create_dir_all(&dir)
                .and_then(|()| std::fs::write(dir.join("statistics.txt"), m::export_text(&entries)))
                .and_then(|()| if csv { std::fs::write(dir.join("statistics.csv"), m::export_csv(&entries)) } else { Ok(()) });
            match result {
                Ok(()) => game.native_chat(&format!("Statistics written to {}", dir.join("statistics.txt").display())),
                Err(e) => game.native_chat(&format!("Could not write the statistics: {e}")),
            }
        }
    }
}
'''

BSTATS.append(dict(
    slug="better-stats", id="better_stats", name="Better Stats", title="Better Stats", raw=True,
    usage="| Command | What it does |\n|---|---|\n| `/statistics top <kind> [n]` | The n biggest entries of a kind (`mined`, `crafted`, `used`, `broken`, `picked_up`, `dropped`, `killed`, `killed_by`, `custom`), default 10 |\n| `/statistics find <words>` | Every entry whose name has all the words, across all kinds |\n| `/statistics export` | Writes `statistics.txt` (and `statistics.csv` if switched on) to the mod's folder |\n| `/statistics` | Lists these |",
    description="Search, rank and export your statistics from chat: the biggest entries of each kind, a search across all of them, and a text or CSV file.",
    upstream="Better Statistics Screen (betterstats)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command="statistics", hooks=["command", "list", "settings", "click"],
    options=[("csv", "Also write a CSV when exporting", False)],
    command_body=COMMAND_BODY,
    raw_note=("It reads the statistics Horizon keeps (`Game::stats`) and calls `load_stats`, which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. "
              "It is the original's searching, ranking and exporting, as chat commands. The redesigned screen (a visual grid, rendered mobs, grouping, tabs of saved files and sharing) is not included; "
              "Search Stats adds the search box to Horizon's own screen. This mod was written from the original's public description, not its source, whose licence does not allow reuse."),
    runtime_extra=r'''
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
''',
    tests=r'''
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
''',
))
