"""Raw-hook ports: mods that read Horizon's own game state (inventory, armor, effects, food) from the HUD and tick hook points.
They are pinned to one Horizon commit because they use fields the stable hook points do not promise."""
from mods import BOX_HEAD, HUD_BOX, hud_guard

RAW = []

DRAW_INK = '''    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(@CORNER@, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (line, ink)) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], *ink, true);
    }'''

TITLE = r'''
/// `minecraft:diamond_helmet` as `Diamond Helmet`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}
'''

TITLE_TEST = r'''
    #[test]
    fn ids_become_titles() {
        assert_eq!(title("minecraft:diamond_helmet"), "Diamond Helmet");
        assert_eq!(title("totem_of_undying"), "Totem Of Undying");
        assert_eq!(title(""), "");
    }
'''

# ---------------------------------------------------------------------------------------------- armor_hud
RAW.append(dict(
    slug="armor-hud", id="armor_hud", name="Armor HUD", title="Armor HUD", raw=True,
    description="Your armor pieces and how much durability each has left, in a corner of the screen.",
    upstream="uku's Armor HUD and Detail Armor Bar Reconstructed", upstream_licence="see the upstream projects",
    command="armor", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("percent", "Show percent", True), ("full", "Hide undamaged pieces", False)],
    runtime_extra=BOX_HEAD + TITLE + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "armor";

/// One worn piece.
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub slot: &'static str,
    pub name: String,
    pub damage: u32,
    pub max: u32,
}

impl Piece {
    pub fn new(slot: &'static str, id: &str, damage: u32, max: u32) -> Self {
        Self { slot, name: title(id), damage, max }
    }

    /// Durability left; unbreakable pieces (max 0) read as full.
    pub fn left(&self) -> u32 {
        self.max.saturating_sub(self.damage)
    }

    /// 0 to 100.
    pub fn percent(&self) -> u32 {
        (self.left() * 100).checked_div(self.max).unwrap_or(100)
    }

    /// Green above half, yellow above a quarter, red below.
    pub fn ink(&self) -> u32 {
        match self.percent() {
            51.. => 0x55_FF55,
            26..=50 => 0xFF_FF55,
            _ => 0xFF_5555,
        }
    }

    pub fn is_full(&self) -> bool {
        self.damage == 0
    }
}

/// The box's lines with their colours.
pub fn lines(pieces: &[Piece], as_percent: bool, hide_full: bool) -> Vec<(String, u32)> {
    pieces
        .iter()
        .filter(|p| !(hide_full && p.is_full()))
        .map(|p| {
            let value = match (p.max, as_percent) {
                (0, _) => "unbreakable".to_owned(),
                (_, true) => format!("{}%", p.percent()),
                (_, false) => format!("{}/{}", p.left(), p.max),
            };
            (format!("{}: {} {}", p.slot, p.name, value), p.ink())
        })
        .collect()
}
''',
    tests=TITLE_TEST + r'''
    #[test]
    fn durability_reads_as_left_percent_and_colour() {
        let p = Piece::new("Head", "minecraft:iron_helmet", 55, 165);
        assert_eq!((p.left(), p.percent()), (110, 66));
        assert_eq!(p.ink(), 0x55_FF55);
        assert_eq!(Piece::new("Head", "x", 100, 165).ink(), 0xFF_FF55);
        assert_eq!(Piece::new("Head", "x", 160, 165).ink(), 0xFF_5555);
        assert_eq!(Piece::new("Head", "x", 999, 165).left(), 0, "never below zero");
        assert_eq!(Piece::new("Head", "x", 5, 0).percent(), 100, "unbreakable");
    }

    #[test]
    fn lines_follow_the_options() {
        let list = vec![
            Piece::new("Head", "minecraft:iron_helmet", 55, 165),
            Piece::new("Feet", "minecraft:iron_boots", 0, 195),
        ];
        assert_eq!(lines(&list, true, false)[0].0, "Head: Iron Helmet 66%");
        assert_eq!(lines(&list, false, false)[1].0, "Feet: Iron Boots 195/195");
        assert_eq!(lines(&list, true, true).len(), 1, "undamaged pieces hidden");
        assert_eq!(lines(&[Piece::new("Chest", "elytra", 0, 0)], false, false)[0].0, "Chest: Elytra unbreakable");
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-left corner").replace("@DATA@", hud_guard("Corner::BottomLeft", '''    let get = |k: &str| game.native_setting(m::ID, k);
    let (percent, hide_full) = (m::is(get, "percent"), m::is(get, "full"));
    let mut pieces = Vec::new();
    for (i, slot) in ["Head", "Chest", "Legs", "Feet"].into_iter().enumerate() {
        if let Some(stack) = &game.armor[i] {
            let max = game.items().get(&stack.item).map_or(0, |info| info.max_damage);
            pieces.push(m::Piece::new(slot, &stack.item, stack.damage, max));
        }
    }
    let lines = m::lines(&pieces, percent, hide_full);
''') + DRAW_INK.replace("@CORNER@", "Corner::BottomLeft")),
))

# ---------------------------------------------------------------------------------------------- potion_counter
RAW.append(dict(
    slug="potion-counter", id="potion_counter", name="Potion Counter", title="Potion Counter", raw=True,
    description="How many potions, splash potions and lingering potions you carry.",
    upstream="PotionCounter (potioncounter)", upstream_licence="see the upstream project",
    command="potions", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("split", "Split by kind", True), ("empty", "Show when none", False)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "potions";

/// The potion kinds an item can be.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub drink: u32,
    pub splash: u32,
    pub lingering: u32,
}

impl Counts {
    /// Adds a stack of `count` items called `id`.
    pub fn add(&mut self, id: &str, count: u32) {
        match id.strip_prefix("minecraft:").unwrap_or(id) {
            "potion" => self.drink += count,
            "splash_potion" => self.splash += count,
            "lingering_potion" => self.lingering += count,
            _ => {}
        }
    }

    pub fn total(&self) -> u32 {
        self.drink + self.splash + self.lingering
    }
}

/// The box's lines.
pub fn lines(c: Counts, split: bool, show_empty: bool) -> Vec<String> {
    if c.total() == 0 && !show_empty {
        return Vec::new();
    }
    if !split {
        return vec![format!("Potions {}", c.total())];
    }
    let mut out = vec![format!("Potions {}", c.drink)];
    if c.splash > 0 || show_empty {
        out.push(format!("Splash {}", c.splash));
    }
    if c.lingering > 0 {
        out.push(format!("Lingering {}", c.lingering));
    }
    out
}
''',
    tests=r'''
    #[test]
    fn counts_sort_by_kind() {
        let mut c = Counts::default();
        c.add("minecraft:potion", 3);
        c.add("minecraft:splash_potion", 5);
        c.add("lingering_potion", 1);
        c.add("minecraft:glass_bottle", 9);
        assert_eq!((c.drink, c.splash, c.lingering, c.total()), (3, 5, 1, 9));
    }

    #[test]
    fn lines_follow_the_options() {
        let c = Counts { drink: 2, splash: 4, lingering: 0 };
        assert_eq!(lines(c, true, false), ["Potions 2", "Splash 4"]);
        assert_eq!(lines(c, false, false), ["Potions 6"]);
        assert!(lines(Counts::default(), true, false).is_empty());
        assert_eq!(lines(Counts::default(), true, true), ["Potions 0", "Splash 0"]);
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-left corner").replace("@DATA@", hud_guard("Corner::BottomLeft", '''    let get = |k: &str| game.native_setting(m::ID, k);
    let (split, empty) = (m::is(get, "split"), m::is(get, "empty"));
    let mut counts = m::Counts::default();
    for stack in game.inventory.iter().flatten().chain(game.offhand.iter()) {
        counts.add(&stack.item, stack.count);
    }
    let lines: Vec<(String, u32)> = m::lines(counts, split, empty).into_iter().map(|l| (l, m::INK)).collect();
''') + DRAW_INK.replace("@CORNER@", "Corner::BottomLeft")),
))

# ---------------------------------------------------------------------------------------------- totem_counter
RAW.append(dict(
    slug="totem-counter", id="totem_counter", name="Totem Counter", title="Totem Counter", raw=True,
    description="How many totems of undying you carry, red when you have none.",
    upstream="TotemCounter (totemcounter)", upstream_licence="see the upstream project",
    command="totems", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("hide", "Hide when none", False)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "totems";
/// The item the box counts.
pub const TOTEM: &str = "minecraft:totem_of_undying";

/// Whether `id` is a totem of undying.
pub fn is_totem(id: &str) -> bool {
    id == TOTEM || id == "totem_of_undying"
}

/// The line and its colour: white with totems, red with none; nothing when `hide_none` and there are none.
pub fn line(count: u32, hide_none: bool) -> Option<(String, u32)> {
    match (count, hide_none) {
        (0, true) => None,
        (0, false) => Some(("Totems 0".to_owned(), 0xFF_5555)),
        (n, _) => Some((format!("Totems {n}"), INK)),
    }
}
''',
    tests=r'''
    #[test]
    fn only_totems_count() {
        assert!(is_totem("minecraft:totem_of_undying"));
        assert!(is_totem("totem_of_undying"));
        assert!(!is_totem("minecraft:diamond"));
    }

    #[test]
    fn the_line_goes_red_at_zero_and_can_hide() {
        assert_eq!(line(3, false), Some(("Totems 3".into(), INK)));
        assert_eq!(line(0, false), Some(("Totems 0".into(), 0xFF_5555)));
        assert_eq!(line(0, true), None);
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-left corner").replace("@DATA@", hud_guard("Corner::BottomLeft", '''    let hide = m::is(|k| game.native_setting(m::ID, k), "hide");
    let count: u32 = game
        .inventory
        .iter()
        .flatten()
        .chain(game.offhand.iter())
        .filter(|s| m::is_totem(&s.item))
        .map(|s| s.count)
        .sum();
    let lines: Vec<(String, u32)> = m::line(count, hide).into_iter().collect();
''') + DRAW_INK.replace("@CORNER@", "Corner::BottomLeft")),
))

# ---------------------------------------------------------------------------------------------- effect_timer
RAW.append(dict(
    slug="effect-timer", id="effect_timer", name="Effect Timer", title="Effect Timer", raw=True,
    description="Your active potion effects and how long each has left, red in the last ten seconds.",
    upstream="Status Effect Timer and Status Effect Bars", upstream_licence="see the upstream projects",
    command="effects", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("ambient", "Show beacon effects", True)],
    runtime_extra=BOX_HEAD + TITLE + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "effects";

/// One active effect, as the game keeps it.
#[derive(Clone, Debug, PartialEq)]
pub struct Active {
    pub id: String,
    /// Ticks left; -1 is endless.
    pub duration: i32,
    pub amplifier: i32,
    pub ambient: bool,
}

/// `m:ss`, or `h:mm:ss` from an hour; `endless` for -1.
pub fn clock(ticks: i32) -> String {
    if ticks < 0 {
        return "endless".to_owned();
    }
    let s = (ticks + 19) / 20;
    let (h, m, s) = (s / 3600, s / 60 % 60, s % 60);
    if h > 0 { format!("{h}:{m:02}:{s:02}") } else { format!("{m}:{s:02}") }
}

/// Roman numerals for the amplifier (0 reads as nothing, 1 as II).
pub fn level(amplifier: i32) -> String {
    const NUMERALS: [&str; 10] = ["", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
    match usize::try_from(amplifier) {
        Ok(0) | Err(_) => String::new(),
        Ok(n) if n < NUMERALS.len() => NUMERALS[n].to_owned(),
        Ok(n) => (n + 1).to_string(),
    }
}

/// The box's lines, shortest time first; red when ten seconds or fewer are left.
pub fn lines(list: &[Active], show_ambient: bool) -> Vec<(String, u32)> {
    let mut shown: Vec<&Active> = list.iter().filter(|e| show_ambient || !e.ambient).collect();
    shown.sort_by_key(|e| if e.duration < 0 { i32::MAX } else { e.duration });
    shown
        .into_iter()
        .map(|e| {
            let name = title(&e.id);
            let tier = level(e.amplifier);
            let label = if tier.is_empty() { name } else { format!("{name} {tier}") };
            let ink = if (0..=200).contains(&e.duration) { 0xFF_5555 } else { INK };
            (format!("{label} {}", clock(e.duration)), ink)
        })
        .collect()
}
''',
    tests=TITLE_TEST + r'''
    #[test]
    fn the_clock_rounds_up_like_the_game_does() {
        assert_eq!(clock(1200), "1:00");
        assert_eq!(clock(1), "0:01");
        assert_eq!(clock(0), "0:00");
        assert_eq!(clock(72_000), "1:00:00");
        assert_eq!(clock(-1), "endless");
    }

    #[test]
    fn levels_are_roman() {
        assert_eq!(level(0), "");
        assert_eq!(level(1), "II");
        assert_eq!(level(4), "V");
        assert_eq!(level(11), "12");
        assert_eq!(level(-3), "");
    }

    #[test]
    fn lines_sort_colour_and_filter() {
        let e = |id: &str, duration, amplifier, ambient| Active { id: id.into(), duration, amplifier, ambient };
        let list = [e("speed", 3600, 1, false), e("poison", 100, 0, false), e("haste", -1, 0, true)];
        let out = lines(&list, true);
        assert_eq!(out[0], ("Poison 0:05".to_owned(), 0xFF_5555));
        assert_eq!(out[1], ("Speed II 3:00".to_owned(), INK));
        assert_eq!(out[2].0, "Haste endless");
        assert_eq!(lines(&list, false).len(), 2, "beacon effects hidden");
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-right corner").replace("@DATA@", hud_guard("Corner::BottomRight", '''    let ambient = m::is(|k| game.native_setting(m::ID, k), "ambient");
    let active: Vec<m::Active> = game
        .potions
        .effects
        .list
        .iter()
        .filter(|e| e.visible)
        .map(|e| m::Active { id: e.id.to_owned(), duration: e.duration, amplifier: e.amplifier, ambient: e.ambient })
        .collect();
    let lines = m::lines(&active, ambient);
''') + DRAW_INK.replace("@CORNER@", "Corner::BottomRight")),
))

# ---------------------------------------------------------------------------------------------- apple_skin
RAW.append(dict(
    slug="apple-skin", id="apple_skin", name="Apple Skin", title="Apple Skin", raw=True,
    description="Your hunger and saturation as numbers, and what the food in your hand would restore.",
    upstream="AppleSkin (appleskin)", upstream_licence="The Unlicense",
    command="food", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("preview", "Held food preview", True), ("saturation", "Show saturation", True)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "food";
/// A full food bar.
pub const MAX_FOOD: i32 = 20;

/// What eating a food would do, by `FoodData.eat`: food and saturation after, and the nutrition that is wasted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Preview {
    pub food: i32,
    pub saturation: f32,
    pub restored: i32,
    pub wasted: i32,
}

/// `food.food = (food + nutrition).clamp(0, 20); saturation = (saturation + gain).clamp(0, food)`.
pub fn preview(food: i32, saturation: f32, nutrition: u32, gain: f32) -> Preview {
    let after = (food + nutrition as i32).clamp(0, MAX_FOOD);
    let sat = (saturation + gain).clamp(0.0, after as f32);
    Preview { food: after, saturation: sat, restored: after - food, wasted: nutrition as i32 - (after - food) }
}

/// The box's lines.
pub fn lines(food: i32, saturation: f32, held: Option<Preview>, show_saturation: bool) -> Vec<String> {
    let mut out = vec![format!("Food {food}/{MAX_FOOD}")];
    if show_saturation {
        out.push(format!("Saturation {saturation:.1}"));
    }
    if let Some(p) = held {
        let mut text = format!("Eat: +{} food", p.restored);
        if show_saturation {
            text.push_str(&format!(", saturation to {:.1}", p.saturation));
        }
        if p.wasted > 0 {
            text.push_str(&format!(" ({} wasted)", p.wasted));
        }
        out.push(text);
    }
    out
}
''',
    tests=r'''
    #[test]
    fn eating_clamps_like_the_game() {
        let p = preview(10, 2.0, 6, 7.2);
        assert_eq!((p.food, p.restored, p.wasted), (16, 6, 0));
        assert!((p.saturation - 9.2).abs() < 1e-4);
        let full = preview(18, 5.0, 8, 12.8);
        assert_eq!((full.food, full.restored, full.wasted), (20, 2, 6));
        assert_eq!(full.saturation, 17.8_f32.min(20.0));
        let capped = preview(20, 19.0, 4, 10.0);
        assert_eq!(capped.saturation, 20.0, "saturation never passes the food level");
    }

    #[test]
    fn lines_follow_the_options() {
        assert_eq!(lines(18, 4.25, None, true), ["Food 18/20", "Saturation 4.2"]);
        assert_eq!(lines(18, 4.25, None, false), ["Food 18/20"]);
        let held = Some(preview(18, 4.0, 8, 12.8));
        let out = lines(18, 4.0, held, true);
        assert_eq!(out[2], "Eat: +2 food, saturation to 16.8 (6 wasted)");
        assert_eq!(lines(10, 0.0, Some(preview(10, 0.0, 4, 2.4)), false)[1], "Eat: +4 food");
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the top-right corner").replace("@DATA@", hud_guard("Corner::TopRight", '''    let get = |k: &str| game.native_setting(m::ID, k);
    let (want_preview, show_saturation) = (m::is(get, "preview"), m::is(get, "saturation"));
    let Some(p) = game.player.as_ref() else {
        return;
    };
    let (food, saturation) = (p.health.food.food, p.health.food.saturation);
    let held = if want_preview {
        game.held()
            .and_then(|stack| game.items().get(&stack.item).and_then(|info| info.food))
            .map(|(nutrition, gain)| m::preview(food, saturation, nutrition, gain))
    } else {
        None
    };
    let lines: Vec<(String, u32)> = m::lines(food, saturation, held, show_saturation).into_iter().map(|l| (l, m::INK)).collect();
''') + DRAW_INK.replace("@CORNER@", "Corner::TopRight")),
))

# ---------------------------------------------------------------------------------------------- pick_up_notifier
RAW.append(dict(
    slug="pick-up-notifier", id="pick_up_notifier", name="Pick Up Notifier", title="Pick Up Notifier", raw=True,
    description="+3 Cobblestone: a short list of what you just picked up (and, if you like, dropped).",
    upstream="Pick Up Notifier (pickupnotifier)", upstream_licence="see the upstream project",
    command="pickups", hooks=["hud", "tick", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("losses", "Show losses", False)],
    runtime_extra=BOX_HEAD + TITLE + r'''
use std::collections::BTreeMap;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "pickups";
/// How long a line stays, in ticks (3 seconds).
pub const LIFE: u32 = 60;
/// At most this many lines at once.
pub const MAX_LINES: usize = 6;

#[derive(Default)]
struct Notes {
    /// What was carried at the last tick, by item id; None until a world is seen.
    last: Option<BTreeMap<String, i64>>,
    /// (item id, change, ticks left), newest last.
    lines: Vec<(String, i64, u32)>,
}

thread_local! {
    static NOTES: RefCell<Notes> = RefCell::new(Notes::default());
}

/// One game tick with what the player carries now (item id, count) pairs; `None` outside a world.
pub fn tick(carried: Option<Vec<(String, u32)>>) {
    NOTES.with_borrow_mut(|n| {
        for line in &mut n.lines {
            line.2 = line.2.saturating_sub(1);
        }
        n.lines.retain(|l| l.2 > 0);
        let Some(carried) = carried else {
            n.last = None;
            n.lines.clear();
            return;
        };
        let mut now: BTreeMap<String, i64> = BTreeMap::new();
        for (id, count) in carried {
            *now.entry(id).or_insert(0) += i64::from(count);
        }
        if let Some(before) = &n.last {
            let mut changes = Vec::new();
            for (id, count) in &now {
                let was = before.get(id).copied().unwrap_or(0);
                if *count != was {
                    changes.push((id.clone(), count - was));
                }
            }
            for (id, was) in before {
                if !now.contains_key(id) {
                    changes.push((id.clone(), -was));
                }
            }
            for (id, delta) in changes {
                // The same item again within a line's life adds to it.
                if let Some(line) = n.lines.iter_mut().find(|l| l.0 == id && (l.1 > 0) == (delta > 0)) {
                    line.1 += delta;
                    line.2 = LIFE;
                } else {
                    n.lines.push((id, delta, LIFE));
                }
            }
            let over = n.lines.len().saturating_sub(MAX_LINES);
            n.lines.drain(..over);
        }
        n.last = Some(now);
    });
}

/// The box's lines: green for gains, red for losses (only when `losses`).
pub fn lines(losses: bool) -> Vec<(String, u32)> {
    NOTES.with_borrow(|n| {
        n.lines
            .iter()
            .filter(|l| losses || l.1 > 0)
            .map(|l| {
                let sign = if l.1 > 0 { "+" } else { "" };
                (format!("{sign}{} {}", l.1, title(&l.0)), if l.1 > 0 { 0x55_FF55 } else { 0xFF_5555 })
            })
            .collect()
    })
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget_notes() {
    NOTES.with_borrow_mut(|n| *n = Notes::default());
}
''',
    tests=TITLE_TEST + r'''
    fn carry(items: &[(&str, u32)]) -> Option<Vec<(String, u32)>> {
        Some(items.iter().map(|(i, c)| ((*i).to_owned(), *c)).collect())
    }

    #[test]
    fn the_first_look_is_a_baseline_not_a_pickup() {
        forget_notes();
        tick(carry(&[("minecraft:cobblestone", 64)]));
        assert!(lines(true).is_empty());
        forget_notes();
    }

    #[test]
    fn gains_and_losses_show_and_merge() {
        forget_notes();
        tick(carry(&[("minecraft:cobblestone", 10)]));
        tick(carry(&[("minecraft:cobblestone", 13)]));
        assert_eq!(lines(false), [("+3 Cobblestone".to_owned(), 0x55_FF55)]);
        tick(carry(&[("minecraft:cobblestone", 15)]));
        assert_eq!(lines(false)[0].0, "+5 Cobblestone", "the same item merges");
        tick(carry(&[]));
        assert!(lines(false).iter().all(|l| l.1 == 0x55_FF55));
        assert!(lines(true).iter().any(|l| l.0 == "-15 Cobblestone"));
        forget_notes();
    }

    #[test]
    fn lines_fade_after_their_life_and_clear_outside_a_world() {
        forget_notes();
        tick(carry(&[]));
        tick(carry(&[("minecraft:diamond", 1)]));
        assert_eq!(lines(false).len(), 1);
        for _ in 0..LIFE {
            tick(carry(&[("minecraft:diamond", 1)]));
        }
        assert!(lines(false).is_empty());
        tick(carry(&[("minecraft:diamond", 2)]));
        tick(None);
        assert!(lines(true).is_empty());
        forget_notes();
    }

    #[test]
    fn at_most_six_lines() {
        forget_notes();
        tick(carry(&[]));
        let many: Vec<(String, u32)> = (0..9).map(|i| (format!("item_{i}"), 1)).collect();
        tick(Some(many));
        assert_eq!(lines(false).len(), MAX_LINES);
        forget_notes();
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-right corner").replace("@DATA@", hud_guard("Corner::BottomRight", '''    let lines = m::lines(m::is(|k| game.native_setting(m::ID, k), "losses"));
''') + DRAW_INK.replace("@CORNER@", "Corner::BottomRight")),
    extra_helpers={"tick": '''// Once a tick: what the player carries, so the changes can be listed.
fn pick_up_notifier_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_pick_up_notifier as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    if game.player.is_none() {
        m::tick(None);
        return;
    }
    let carried = game
        .inventory
        .iter()
        .flatten()
        .chain(game.offhand.iter())
        .map(|s| (s.item.to_string(), s.count))
        .collect();
    m::tick(Some(carried));
}
'''},
))
