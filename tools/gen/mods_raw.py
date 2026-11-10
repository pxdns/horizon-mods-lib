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

OVER_APPLE = """    if game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "overlay") && game.riding.is_none() && !game.creative {
        if let Some(p) = game.player.as_ref() {
            let get = |k: &str| game.native_setting(m::ID, k);
            let (food, saturation) = (p.health.food.food, p.health.food.saturation);
            let after = if m::is(get, "preview") {
                game.held()
                    .and_then(|stack| game.items().get(&stack.item).and_then(|info| info.food))
                    .map(|(nutrition, gain)| m::preview(food, saturation, nutrition, gain).food)
            } else {
                None
            };
            for (rect, ink) in m::food_overlay(batch.width as i32, batch.height as i32, food, saturation, after, m::is(get, "saturation")) {
                game.native_fill(batch, rect, ink);
            }
        }
    }
"""

OVER_EFFECT = """    if game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "bars") {
        let icons: Vec<m::Icon> = game
            .potions
            .effects
            .list
            .iter()
            .filter(|e| e.visible)
            .map(|e| {
                let def = horizon_client_core::potions::effect(e.id);
                m::Icon {
                    id: e.id.to_owned(),
                    duration: e.duration,
                    ambient: e.ambient,
                    beneficial: def.is_some_and(|d| d.category == horizon_client_core::potions::Category::Beneficial),
                    color: def.map_or(0xFF_FFFF, |d| d.color as u32),
                }
            })
            .collect();
        for (rect, ink) in m::bars(batch.width as i32, &icons) {
            game.native_fill(batch, rect, ink);
        }
    }
"""

OVER_ARMOR = """    if game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "bar") && !game.creative {
        {
            let armor = game.player.as_mut().map_or(0, |p| p.attribute("minecraft:armor").or_else(|_| p.attribute("armor")).unwrap_or(0.0) as i32);
            if armor > 0 {
                let (wi, hi) = (batch.width as i32, batch.height as i32);
                let row = (hi - game.health_hud.left_height + 10) as f32;
                let label = m::label(armor);
                let x = (wi / 2 - 91) as f32 - game.native_text_width(&label) - 3.0;
                game.native_text(batch, &label, [x, row + 1.0], m::INK, true);
                for (rect, ink) in m::extra_rows(wi, row as i32, armor) {
                    game.native_fill(batch, rect, ink);
                }
            }
        }
    }
"""

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

# ---------------------------------------------------------------------------------------------- effect_timer
RAW.append(dict(
    slug="effect-timer", id="effect_timer", name="Effect Timer", title="Effect Timer", raw=True,
    description="Your active potion effects and how long each has left, red in the last ten seconds, with a duration bar under each effect icon.",
    upstream="Status Effect Timer and Status Effect Bars", upstream_licence="see the upstream projects",
    command="effects", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("ambient", "Show beacon effects", True), ("bars", "Duration bars under the icons", True)],
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

/// An effect as the icon row shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct Icon {
    pub id: String,
    pub duration: i32,
    pub ambient: bool,
    pub beneficial: bool,
    pub color: u32,
}

/// The game's own order of the icons (`MobEffectInstance.compareTo`), as the row is sorted by it, reversed.
pub fn icon_order(a: &Icon, o: &Icon) -> std::cmp::Ordering {
    if (a.duration <= 32147 || o.duration <= 32147) && !(a.ambient && o.ambient) {
        a.ambient
            .cmp(&o.ambient)
            .then((a.duration == -1).cmp(&(o.duration == -1)))
            .then(a.duration.cmp(&o.duration))
            .then(a.color.cmp(&o.color))
    } else {
        a.ambient.cmp(&o.ambient).then(a.color.cmp(&o.color))
    }
}

/// The longest each effect has been seen to last, to turn the ticks left into a fraction.
#[derive(Default)]
pub struct Longest(std::collections::BTreeMap<String, i32>);

impl Longest {
    /// Notes the current effects and returns each one's longest duration; forgets effects that ended.
    pub fn observe(&mut self, icons: &[Icon]) -> Vec<i32> {
        self.0.retain(|id, _| icons.iter().any(|i| &i.id == id));
        icons
            .iter()
            .map(|i| {
                let longest = self.0.entry(i.id.clone()).or_insert(i.duration);
                if i.duration > *longest {
                    *longest = i.duration;
                }
                *longest
            })
            .collect()
    }
}

/// Rectangles `([x, y, w, h], argb)` for a bar under each timed icon of the vanilla row: icons are 24 wide and
/// sit 25 apart from the right edge, beneficial ones at y 1 and harmful ones at y 27.
pub fn layout(w: i32, icons: &[Icon], longest: &[i32]) -> Vec<([f32; 4], u32)> {
    let mut order: Vec<usize> = (0..icons.len()).collect();
    order.sort_by(|&a, &b| icon_order(&icons[b], &icons[a]));
    let (mut beneficial, mut harmful) = (0, 0);
    let mut out = Vec::new();
    for i in order {
        let icon = &icons[i];
        let (x, y) = if icon.beneficial {
            beneficial += 1;
            (w - 25 * beneficial, 1)
        } else {
            harmful += 1;
            (w - 25 * harmful, 27)
        };
        if icon.duration < 0 {
            continue;
        }
        let fraction = (icon.duration as f32 / longest[i].max(1) as f32).clamp(0.0, 1.0);
        let width = (20.0 * fraction).ceil();
        let (x, y) = ((x + 2) as f32, (y + 21) as f32);
        out.push(([x, y, 20.0, 2.0], 0xAA_000000));
        out.push(([x, y, width, 2.0], 0xFF00_0000 | icon.color));
    }
    out
}

thread_local! {
    static LONGEST: std::cell::RefCell<Longest> = std::cell::RefCell::new(Longest::default());
}

/// `layout` with the longest durations the mod has seen so far.
pub fn bars(w: i32, icons: &[Icon]) -> Vec<([f32; 4], u32)> {
    let longest = LONGEST.with_borrow_mut(|l| l.observe(icons));
    layout(w, icons, &longest)
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
    fn bars_follow_the_game_row_and_fill_with_time_left() {
        let icon = |id: &str, duration, beneficial| Icon { id: id.into(), duration, ambient: false, beneficial, color: 0x33_CC33 };
        let icons = [icon("speed", 600, true), icon("poison", 100, false), icon("haste", 1200, true)];
        let mut longest = Longest::default();
        assert_eq!(longest.observe(&icons), [600, 100, 1200]);
        // Fifteen seconds in, speed has half left.
        let later = [icon("speed", 300, true), icon("poison", 50, false), icon("haste", 900, true)];
        let maxes = longest.observe(&later);
        assert_eq!(maxes, [600, 100, 1200], "the longest stays");
        let out = layout(320, &later, &maxes);
        // The row lists the longest-lasting first: haste, then speed, one icon further left.
        assert_eq!(out[0].0, [297.0, 22.0, 20.0, 2.0]);
        assert_eq!(out[1].0[2], 15.0, "haste has 900 of 1200: 15 of 20 pixels");
        assert_eq!(out[2].0[0], 272.0);
        assert_eq!(out[3].0[2], 10.0);
        // Harmful effects sit on the second row.
        assert_eq!(out[4].0[1], 48.0);
        assert!(layout(320, &[Icon { duration: -1, ..icon("x", 0, true) }], &[1]).is_empty(), "endless effects have no bar");
        longest.observe(&[]);
        assert_eq!(longest.observe(&icons), [600, 100, 1200], "an ended effect is forgotten");
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
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-right corner").replace("@DATA@", OVER_EFFECT + hud_guard("Corner::BottomRight", '''    let ambient = m::is(|k| game.native_setting(m::ID, k), "ambient");
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
