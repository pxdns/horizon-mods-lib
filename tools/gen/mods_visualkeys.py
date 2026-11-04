"""VisualKeys: an on-screen keyboard coloured by what each key is bound to, lit while pressed. Pinned to one Horizon commit."""
from mods import BOX_HEAD, HUD_BOX
from mods_raw import DRAW_INK

VK = []

HUD_HELPER = '''// The keyboard, in a corner: each key coloured by what it is bound to, bright while held.
fn visual_keys_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_visual_keys as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let bindings: Vec<(String, String)> = horizon_client::keybinds::MAPPINGS
        .iter()
        .map(|(action, _, category)| ((*category).to_owned(), game.settings.keys.get(action).to_owned()))
        .collect();
    let map = m::bind_map(&bindings);
    let cells = m::cells();
    let (w, h) = m::size(&cells);
    let (x0, y0) = corners.place(Corner::BottomRight, w + 4.0, h + 4.0);
    game.native_fill(batch, [x0, y0, w + 4.0, h + 4.0], m::BACKGROUND);
    for cell in &cells {
        let ink = m::colour(&cell.name, &map, m::pressed(&cell.name));
        let (x, y) = (x0 + 2.0 + cell.x, y0 + 2.0 + cell.y);
        game.native_fill(batch, [x, y, cell.w, cell.h], ink);
        if game.native_text_width(&cell.label) <= cell.w - 1.0 {
            let tx = x + ((cell.w - game.native_text_width(&cell.label)) / 2.0).floor();
            game.native_text(batch, &cell.label, [tx, y + 1.0], m::INK, false);
        }
    }
}
'''

COMMAND_HELPER = '''// `/keyboard`, `/keyboard free`, `/keyboard <key>`.
fn visual_keys_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_visual_keys as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let bindings: Vec<(String, String)> = horizon_client::keybinds::MAPPINGS
        .iter()
        .map(|(action, _, _)| ((*action).to_owned(), game.settings.keys.get(action).to_owned()))
        .collect();
    match words.next() {
        None => {
            let stored = {
                let game = &*game;
                m::click(|k| game.native_setting(m::ID, k), 0)
            };
            if let Some((key, value)) = stored {
                game.native_set_setting(m::ID, key, &value);
                game.native_chat(&format!("{} {}", m::NAME, if value == "true" { "shown" } else { "hidden" }));
            }
        }
        Some("free") => game.native_chat(&m::free_keys(&bindings)),
        Some(key) => game.native_chat(&m::describe(key, &bindings)),
    }
}
'''

KEY_HELPER = '''// Every key: remembered while held for the lit keys, and the toggle key shows or hides the keyboard.
fn visual_keys_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_visual_keys as m;
    m::held(key, pressed);
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if key == game.native_mod_key(m::ID, m::TOGGLE_KEY, m::TOGGLE_DEFAULT) {
        let stored = {
            let game = &*game;
            m::click(|k| game.native_setting(m::ID, k), 0)
        };
        if let Some((name, value)) = stored {
            game.native_set_setting(m::ID, name, &value);
        }
    }
}
'''

VK.append(dict(
    slug="visual-keys", id="visual_keys", name="VisualKeys", title="VisualKeys", raw=True,
    usage="| Key | Default | What it does |\n|---|---|---|\n| Show keyboard | `F9` | Shows or hides the keyboard in the bottom-right corner |\n\nEach key is coloured by the category of the action bound to it (movement green, gameplay red, inventory blue, multiplayer purple, other gold), dark when it has no binding, and bright while you hold it. `/keyboard` shows or hides it, `/keyboard free` lists the letters, digits and function keys with no binding, and `/keyboard q` says what `q` does.",
    description="An on-screen keyboard that shows what every key is bound to and which keys are free.",
    upstream="VisualKeys (visualkeys)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command="keyboard", hooks=["hud", "key", "command", "list", "settings", "click"],
    options=[("shown", "Show the keyboard", False)],
    keys_expr='vec![[m::TOGGLE_KEY.into(), "Show keyboard".into(), m::TOGGLE_DEFAULT.into()]]',
    raw_note=("It reads the key bindings (`Game::settings.keys`) and Horizon's list of mappings, which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. "
              "It shows the keyboard on the HUD only: the original also draws it inside the controls screen, shows Malilib bindings and collapses categories, which need a hook in the options screen. "
              "Labels fit only on the wider keys. It was written from the original's public description, not its source, whose licence does not allow reuse."),
    runtime_extra=BOX_HEAD + r'''
use std::collections::{BTreeMap, BTreeSet};

/// Slash command.
pub const COMMAND: &str = "keyboard";
pub const TOGGLE_KEY: &str = "toggle";
pub const TOGGLE_DEFAULT: &str = "key.keyboard.f9";

/// A key on the picture: its vanilla name, the label, and its box in pixels from the keyboard's corner.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub name: String,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Rows of (name, label, width in key units): a US layout.
pub const ROWS: [&[(&str, &str, f32)]; 6] = [
    &[
        ("escape", "Esc", 1.5), ("f1", "1", 1.0), ("f2", "2", 1.0), ("f3", "3", 1.0), ("f4", "4", 1.0), ("f5", "5", 1.0), ("f6", "6", 1.0),
        ("f7", "7", 1.0), ("f8", "8", 1.0), ("f9", "9", 1.0), ("f10", "10", 1.0), ("f11", "11", 1.0), ("f12", "12", 1.0),
    ],
    &[
        ("grave.accent", "`", 1.0), ("1", "1", 1.0), ("2", "2", 1.0), ("3", "3", 1.0), ("4", "4", 1.0), ("5", "5", 1.0), ("6", "6", 1.0), ("7", "7", 1.0),
        ("8", "8", 1.0), ("9", "9", 1.0), ("0", "0", 1.0), ("minus", "-", 1.0), ("equal", "=", 1.0), ("backspace", "<-", 2.0),
    ],
    &[
        ("tab", "Tab", 1.5), ("q", "Q", 1.0), ("w", "W", 1.0), ("e", "E", 1.0), ("r", "R", 1.0), ("t", "T", 1.0), ("y", "Y", 1.0), ("u", "U", 1.0),
        ("i", "I", 1.0), ("o", "O", 1.0), ("p", "P", 1.0), ("left.bracket", "[", 1.0), ("right.bracket", "]", 1.0), ("backslash", "\\", 1.5),
    ],
    &[
        ("caps.lock", "Cap", 1.75), ("a", "A", 1.0), ("s", "S", 1.0), ("d", "D", 1.0), ("f", "F", 1.0), ("g", "G", 1.0), ("h", "H", 1.0), ("j", "J", 1.0),
        ("k", "K", 1.0), ("l", "L", 1.0), ("semicolon", ";", 1.0), ("apostrophe", "'", 1.0), ("enter", "Ent", 2.25),
    ],
    &[
        ("left.shift", "Sh", 2.25), ("z", "Z", 1.0), ("x", "X", 1.0), ("c", "C", 1.0), ("v", "V", 1.0), ("b", "B", 1.0), ("n", "N", 1.0), ("m", "M", 1.0),
        ("comma", ",", 1.0), ("period", ".", 1.0), ("slash", "/", 1.0), ("right.shift", "Sh", 2.75),
    ],
    &[("left.control", "Ctl", 1.5), ("left.alt", "Alt", 1.5), ("space", "", 7.0), ("right.alt", "Alt", 1.5), ("right.control", "Ctl", 1.5)],
];

const UNIT: f32 = 11.0;
const ROW_H: f32 = 10.0;

/// Every key's box: rows from the top, keys left to right, one pixel between.
pub fn cells() -> Vec<Cell> {
    let mut out = Vec::new();
    for (r, row) in ROWS.iter().enumerate() {
        let mut x = 0.0;
        for (name, label, units) in row.iter() {
            out.push(Cell { name: format!("key.keyboard.{name}"), label: (*label).to_owned(), x, y: r as f32 * ROW_H, w: units * UNIT - 1.0, h: ROW_H - 1.0 });
            x += units * UNIT;
        }
    }
    out
}

/// The picture's width and height in pixels.
pub fn size(cells: &[Cell]) -> (f32, f32) {
    let w = cells.iter().map(|c| c.x + c.w).fold(0.0, f32::max);
    let h = cells.iter().map(|c| c.y + c.h).fold(0.0, f32::max);
    (w, h)
}

thread_local! {
    static HELD: std::cell::RefCell<BTreeSet<String>> = const { std::cell::RefCell::new(BTreeSet::new()) };
}

/// A key went down or up.
pub fn held(key: &str, down: bool) {
    HELD.with_borrow_mut(|set| {
        if down {
            set.insert(key.to_owned());
        } else {
            set.remove(key);
        }
    });
}

pub fn pressed(key: &str) -> bool {
    HELD.with_borrow(|set| set.contains(key))
}

/// Which category's colour a key gets: `bindings` are (category, bound key name).
pub fn bind_map(bindings: &[(String, String)]) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for (category, key) in bindings {
        map.entry(key.clone()).or_insert_with(|| category.clone());
    }
    map
}

/// The colour of a key: its category's, dark when unbound, brighter while held.
pub fn colour(key: &str, map: &BTreeMap<String, String>, down: bool) -> u32 {
    let base = match map.get(key).map(String::as_str) {
        None => 0xB0_202020,
        Some("movement") => 0xD0_2E8B57,
        Some("gameplay") => 0xD0_B03A3A,
        Some("inventory") => 0xD0_3A6EB0,
        Some("multiplayer") => 0xD0_8A4FB0,
        Some(_) => 0xD0_B08A2E,
    };
    if down { 0xFF_FFFFFF } else { base }
}

/// `key.forward` as `forward`, `key.hotbar.3` as `hotbar 3`.
pub fn action_name(action: &str) -> String {
    action.strip_prefix("key.").unwrap_or(action).replace('.', " ")
}

/// A key as the player types it (`q`, `f9`, `left.shift`) or in full, as its vanilla name.
fn full(key: &str) -> String {
    if key.starts_with("key.") { key.to_owned() } else { format!("key.keyboard.{}", key.to_lowercase()) }
}

/// What a key does: `bindings` are (action, bound key name).
pub fn describe(key: &str, bindings: &[(String, String)]) -> String {
    let name = full(key);
    let actions: Vec<String> = bindings.iter().filter(|(_, k)| *k == name).map(|(a, _)| action_name(a)).collect();
    let shown = name.trim_start_matches("key.keyboard.");
    if actions.is_empty() { format!("{shown}: no binding") } else { format!("{shown}: {}", actions.join(", ")) }
}

/// The letters, digits and function keys that nothing is bound to.
pub fn free_keys(bindings: &[(String, String)]) -> String {
    let used: BTreeSet<&str> = bindings.iter().map(|(_, k)| k.as_str()).collect();
    let free: Vec<String> = cells()
        .into_iter()
        .filter(|c| !used.contains(c.name.as_str()))
        .map(|c| c.name.trim_start_matches("key.keyboard.").to_owned())
        .filter(|n| n.len() == 1 && n.chars().all(|c| c.is_ascii_alphanumeric()) || (n.starts_with('f') && n.len() <= 3 && n[1..].chars().all(|c| c.is_ascii_digit())))
        .collect();
    if free.is_empty() { "Every letter, digit and function key has a binding".to_owned() } else { format!("Free keys: {}", free.join(" ")) }
}
''',
    tests=r'''
    fn binds() -> Vec<(String, String)> {
        [("key.forward", "key.keyboard.w"), ("key.back", "key.keyboard.s"), ("key.inventory", "key.keyboard.e"), ("key.sneak", "key.keyboard.left.shift"), ("key.sprint", "key.keyboard.left.shift")]
            .map(|(a, k)| (a.to_owned(), k.to_owned()))
            .to_vec()
    }

    #[test]
    fn the_picture_is_a_keyboard_sized_block() {
        let cells = cells();
        assert_eq!(cells.len(), ROWS.iter().map(|r| r.len()).sum::<usize>());
        let (w, h) = size(&cells);
        assert!(w > 150.0 && w < 200.0, "{w}");
        assert!((h - 59.0).abs() < 0.01, "{h}");
        for c in &cells {
            assert!(c.x >= 0.0 && c.x + c.w <= w + 0.01, "{c:?}");
        }
        let space = cells.iter().find(|c| c.name == "key.keyboard.space").unwrap();
        assert!(space.w > 70.0, "the space bar is wide");
    }

    #[test]
    fn rows_have_no_overlap_and_names_are_unique() {
        let cells = cells();
        let mut names = std::collections::BTreeSet::new();
        assert!(cells.iter().all(|c| names.insert(c.name.clone())));
        for row in ROWS {
            let mut x = 0.0_f32;
            for (_, _, u) in row.iter() {
                x += u * 11.0;
            }
            assert!(x <= 190.0, "{x}");
        }
    }

    #[test]
    fn colours_follow_the_category_and_the_press() {
        let map = bind_map(&[("movement".into(), "key.keyboard.w".into()), ("inventory".into(), "key.keyboard.e".into())]);
        assert_eq!(colour("key.keyboard.w", &map, false), 0xD0_2E8B57);
        assert_eq!(colour("key.keyboard.e", &map, false), 0xD0_3A6EB0);
        assert_eq!(colour("key.keyboard.z", &map, false), 0xB0_202020, "unbound is dark");
        assert_eq!(colour("key.keyboard.w", &map, true), 0xFF_FFFFFF, "held is bright");
    }

    #[test]
    fn held_keys_are_remembered_until_released() {
        held("key.keyboard.q", true);
        assert!(pressed("key.keyboard.q"));
        held("key.keyboard.q", false);
        assert!(!pressed("key.keyboard.q"));
    }

    #[test]
    fn describing_and_listing_free_keys() {
        assert_eq!(describe("w", &binds()), "w: forward");
        assert_eq!(describe("left.shift", &binds()), "left.shift: sneak, sprint");
        assert_eq!(describe("Z", &binds()), "z: no binding");
        assert_eq!(describe("key.keyboard.e", &binds()), "e: inventory");
        let free = free_keys(&binds());
        assert!(free.starts_with("Free keys: "));
        assert!(free.contains(" z") && free.contains(" q") && !free.contains(" w ") && !free.ends_with(" w"), "{free}");
        assert!(free.contains("f9"), "function keys count");
    }

    #[test]
    fn action_names_are_readable() {
        assert_eq!(action_name("key.forward"), "forward");
        assert_eq!(action_name("key.hotbar.3"), "hotbar 3");
    }
''',
    helpers={"hud": HUD_HELPER, "command": COMMAND_HELPER, "key": KEY_HELPER},
))
