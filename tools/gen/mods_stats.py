"""Search Stats: type to filter the statistics screen's Items and Mobs lists. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

STATS = []

STATS.append(dict(
    slug="search-stats", id="search_stats", name="Search Stats", title="Search Stats", raw=True,
    usage="Open Statistics and type: the Items and Mobs lists keep only the entries whose name contains every word you typed. Backspace deletes a letter, Delete clears the search, and it starts empty each time the screen opens. The search box sits left of the Done button.",
    description="Adds a search box to the statistics screen that filters the Items and Mobs lists as you type.",
    upstream="SearchStats (searchstats)", upstream_licence="Apache-2.0",
    command=None, hooks=["items", "mobs", "footer", "open", "key", "list", "settings", "click"],
    options=[("box", "Show the search box", True)],
    modpath="crate::game::stats_screen::lib_search_stats",
    raw_note=("It replaces the Items list builder of Horizon's statistics screen (`stat_items`, rebuilt from its source with the filter added) with a cancellable HEAD hook, adds a RETURN modifier to the Mobs list builder (`stat_mobs`), and draws the box from a HEAD hook on `draw_stats_footer`, in `game/stats_screen.rs`. "
              "Typing arrives through the stable key hook as key names, so only the US letters, digits, space and minus are read. None of these functions is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. "
              "The General tab is not filtered: its rows are drawn inline in the screen's drawing function, which this does not replace."),
    toml_text=toml_text("search_stats", "1.0.0", "crates/horizon-client/src/game/stats_screen.rs", "lib_search_stats", [
        dict(fields=[("id", '"items"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client::game::stats_screen::Game<\'_>::stat_items"'), ("helper", '"items.rs"')],
             bindings=[("game", "&crate::game::Game<'_>", "&*self")]),
        dict(fields=[("id", '"mobs"'), ("kind", '"RETURN"'), ("target", '"horizon_client::game::stats_screen::Game<\'_>::stat_mobs"'), ("helper", '"mobs.rs"')],
             bindings=[("mobs", "Vec<String>", "$value")]),
        dict(fields=[("id", '"footer"'), ("kind", '"HEAD"'), ("target", '"horizon_client::game::stats_screen::Game<\'_>::draw_stats_footer"'), ("helper", '"footer.rs"')],
             bindings=[("game", "&crate::game::Game<'_>", "&*self"), ("batch", "&mut HudBatch", "&mut *batch"), ("w", "f32", "w"), ("h", "f32", "h")]),
        dict(fields=[("id", '"open"'), ("kind", '"HEAD"'), ("target", '"horizon_client::game::stats_screen::Game<\'_>::open_stats"'), ("helper", '"open.rs"')],
             bindings=[("game", "&mut crate::game::Game<'_>", "&mut *self")]),
        native("key", "native_key", [G, ("key", "&str", "key"), ("pressed", "bool", "pressed")]),
    ] + common_native(False)),
    runtime_extra=r'''
use std::cell::Cell;

thread_local! {
    static QUERY: RefCell<String> = const { RefCell::new(String::new()) };
    static SHOW: Cell<bool> = const { Cell::new(true) };
}

/// Longest search the box takes.
pub const MAX: usize = 40;

/// Starts a fresh search (the screen opened); `show` is the "Show the search box" setting.
pub fn reset(show: bool) {
    QUERY.with_borrow_mut(String::clear);
    SHOW.with(|s| s.set(show));
}

pub fn shown() -> bool {
    SHOW.with(Cell::get)
}

pub fn query() -> String {
    QUERY.with_borrow(Clone::clone)
}

/// The character a key name types, if any: `key.keyboard.a` -> `a`, `key.keyboard.space` -> ` `.
pub fn typed(key: &str) -> Option<char> {
    let name = key.strip_prefix("key.keyboard.")?;
    match name {
        "space" => Some(' '),
        "minus" => Some('-'),
        n if n.len() == 1 && n.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) => n.chars().next(),
        _ => None,
    }
}

/// A key went down while the screen is open: edits the search. Returns whether it changed.
pub fn press(key: &str) -> bool {
    if !shown() {
        return false;
    }
    QUERY.with_borrow_mut(|q| match key {
        "key.keyboard.backspace" => q.pop().is_some(),
        "key.keyboard.delete" => {
            let had = !q.is_empty();
            q.clear();
            had
        }
        _ => match typed(key) {
            Some(c) if q.chars().count() < MAX => {
                q.push(c);
                true
            }
            _ => false,
        },
    })
}

/// `minecraft:diamond_pickaxe` as `diamond pickaxe`.
pub fn plain(id: &str) -> String {
    id.rsplit(':').next().unwrap_or(id).replace('_', " ").to_lowercase()
}

/// Whether `id` matches: every word of `query` is somewhere in its plain name. An empty query matches everything.
pub fn matches(query: &str, id: &str) -> bool {
    let name = plain(id);
    query.split_whitespace().all(|word| name.contains(&word.to_lowercase()))
}

/// The ids that match the current search, in their order.
pub fn filter(ids: Vec<String>) -> Vec<String> {
    if !shown() {
        return ids;
    }
    let query = query();
    if query.trim().is_empty() {
        return ids;
    }
    ids.into_iter().filter(|id| matches(&query, id)).collect()
}
''',
    tests=r'''
    fn ids() -> Vec<String> {
        ["minecraft:diamond_pickaxe", "minecraft:diamond", "minecraft:stone_pickaxe", "minecraft:cobblestone"].map(String::from).to_vec()
    }

    fn type_text(text: &str) {
        for c in text.chars() {
            let key = if c == ' ' { "key.keyboard.space".to_owned() } else { format!("key.keyboard.{c}") };
            press(&key);
        }
    }

    #[test]
    fn keys_type_letters_digits_space_and_minus() {
        assert_eq!(typed("key.keyboard.a"), Some('a'));
        assert_eq!(typed("key.keyboard.7"), Some('7'));
        assert_eq!(typed("key.keyboard.space"), Some(' '));
        assert_eq!(typed("key.keyboard.minus"), Some('-'));
        assert_eq!(typed("key.keyboard.enter"), None);
        assert_eq!(typed("key.keyboard.left.shift"), None);
        assert_eq!(typed("key.mouse.left"), None);
    }

    #[test]
    fn names_are_matched_word_by_word() {
        assert!(matches("pick", "minecraft:diamond_pickaxe"));
        assert!(matches("diamond pick", "minecraft:diamond_pickaxe"));
        assert!(matches("PICKAXE diam", "minecraft:diamond_pickaxe"), "any order, any case");
        assert!(!matches("diamond pick", "minecraft:stone_pickaxe"));
        assert!(matches("", "minecraft:stone"));
        assert!(matches("   ", "minecraft:stone"));
        assert!(!matches("minecraft", "minecraft:stone"), "the namespace is not searched");
    }

    #[test]
    fn typing_filters_and_backspace_widens() {
        reset(true);
        assert_eq!(filter(ids()).len(), 4);
        type_text("pickaxe");
        assert_eq!(filter(ids()), ["minecraft:diamond_pickaxe", "minecraft:stone_pickaxe"]);
        type_text(" st");
        assert_eq!(filter(ids()), ["minecraft:stone_pickaxe"]);
        press("key.keyboard.backspace");
        press("key.keyboard.backspace");
        press("key.keyboard.backspace");
        assert_eq!(query(), "pickaxe");
        assert!(press("key.keyboard.delete"));
        assert_eq!(query(), "");
        assert!(!press("key.keyboard.delete"), "nothing to clear");
        assert_eq!(filter(ids()).len(), 4);
    }

    #[test]
    fn the_search_is_capped_and_can_be_switched_off() {
        reset(true);
        for _ in 0..(MAX + 10) {
            press("key.keyboard.a");
        }
        assert_eq!(query().len(), MAX);
        reset(false);
        assert!(!press("key.keyboard.a"));
        assert_eq!(filter(ids()).len(), 4, "off means no filtering");
        reset(true);
    }
''',
    helpers={
        "items": '''// The Items tab's list as Horizon builds it (items with a non-zero column, in the chosen sort), narrowed to the search.
fn search_stats_items(game: &crate::game::Game<'_>) -> std::ops::ControlFlow<Vec<String>> {
    let mut items: Vec<String> = Vec::new();
    for (ty, _) in COLUMNS {
        if let Some(m) = game.stats.values.get(&format!("minecraft:{ty}")) {
            for (k, v) in m {
                if *v > 0 && k != "minecraft:air" && !items.contains(k) {
                    items.push(k.clone());
                }
            }
        }
    }
    if let Some((col, order)) = game.stats_ui.sort {
        let ty = COLUMNS[col].0;
        items.sort_by_key(|i| game.stats.get(ty, i) * -order);
    }
    std::ops::ControlFlow::Break(lib_search_stats::filter(items))
}
''',
        "mobs": '''// The Mobs tab's list, narrowed to the search.
fn search_stats_mobs(mobs: Vec<String>) -> Vec<String> {
    lib_search_stats::filter(mobs)
}
''',
        "open": '''// The statistics screen opens: a fresh, empty search.
fn search_stats_open(game: &mut crate::game::Game<'_>) {
    use lib_search_stats as m;
    let show = game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "box");
    m::reset(show);
}
''',
        "key": '''// A key went down: on the statistics screen it types into the search.
fn @ID@_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::@MOD@ as m;
    if pressed && game.screen == Screen::Stats {
        m::press(key);
    }
}
''',
        "footer": '''// The footer: the search box, left of the Done button.
fn search_stats_footer(game: &crate::game::Game<'_>, batch: &mut HudBatch, w: f32, h: f32) {
    use lib_search_stats as m;
    if !m::shown() {
        return;
    }
    let atlas = &game.gui.atlas;
    let width = (w / 2.0 - 100.0 - 12.0).clamp(60.0, 160.0);
    let (x, y) = (((w / 2.0 - 100.0 - 8.0) - width).max(4.0).floor(), h - FOOTER + 6.0);
    batch.fill(atlas, x - 1.0, y - 1.0, width + 2.0, 22.0, 0xFFA0_A0A0);
    batch.fill(atlas, x, y, width, 20.0, 0xFF00_0000);
    let query = m::query();
    if query.is_empty() {
        batch.text(atlas, "Search...", x + 4.0, y + 6.0, 0xFF70_7070, false);
    } else {
        // Keep the end of a long search in view.
        let mut shown: String = query.clone();
        while HudBatch::text_width(atlas, &shown) > width - 14.0 && !shown.is_empty() {
            shown.remove(0);
        }
        batch.text(atlas, &shown, x + 4.0, y + 6.0, 0xFFFF_FFFF, true);
        let caret = x + 4.0 + HudBatch::text_width(atlas, &shown);
        if (game.started.elapsed().as_millis() / 500) % 2 == 0 {
            batch.fill(atlas, caret + 1.0, y + 5.0, 1.0, 10.0, 0xFFFF_FFFF);
        }
    }
}
''',
    },
))
