"""Item Highlighter: a twinkling star on the slots of items you just picked up, until you close the inventory. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

HIGH = []

HIGH.append(dict(
    slug="item-highlighter", id="item_highlighter", name="Item Highlighter", title="Item Highlighter", raw=True,
    usage="Nothing to press: pick something up, open your inventory, and every slot holding that kind of item shows a small twinkling gold star. The stars go away when you close the inventory (a setting keeps them instead until you pick up nothing new).",
    description="Marks newly picked-up items with a twinkling star in your inventory.",
    upstream="Item Highlighter (highlighter)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command=None, hooks=["slot", "tick", "list", "settings", "click"],
    options=[("clear", "Clear the stars when the inventory closes", True)],
    toml_text=toml_text("item_highlighter", "1.0.0", "crates/horizon-client/src/game.rs", "lib_item_highlighter", [
        dict(fields=[("id", '"slot"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client::game::Game<\'_>::draw_slot_item"'), ("helper", '"slot.rs"')],
             bindings=[("game", "&crate::game::Game<'_>", "&*self"), ("batch", "&mut HudBatch", "&mut *batch"), ("drag", "Option<&DragView>", "drag"),
                       ("slot", "usize", "slot"), ("stack", "&Option<ItemStack>", "stack"), ("x", "f32", "x"), ("y", "f32", "y")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False)),
    raw_note=("It puts a cancellable HEAD hook on `Game::draw_slot_item`, which draws one slot's item on a container screen: the hook draws the slot as Horizon would and then the star over it. "
              "It reads the inventory each tick to see what was picked up. Neither is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. "
              "Stars mark a kind of item, not one slot; the original's clearing on hover and its resource-pack look are not included. It was written from the original's public description, not its source, whose licence does not allow reuse."),
    runtime_extra=r'''
use std::cell::{Cell, RefCell as StateCell};
use std::collections::{BTreeMap, BTreeSet};

thread_local! {
    /// What was carried at the last tick, by item id; None until a world is seen.
    static LAST: StateCell<Option<BTreeMap<String, i64>>> = const { StateCell::new(None) };
    /// The kinds of item picked up since the marks were last cleared.
    static NEW: StateCell<BTreeSet<String>> = const { StateCell::new(BTreeSet::new()) };
    static WAS_OPEN: Cell<bool> = const { Cell::new(false) };
    /// True while the mod draws a slot again from inside the hook.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// One tick: the totals now carried, whether a container screen is open, whether to clear the marks when it closes.
pub fn tick(now: BTreeMap<String, i64>, open: bool, clear_on_close: bool) {
    LAST.with_borrow_mut(|last| {
        if let Some(before) = last.as_ref() {
            NEW.with_borrow_mut(|new| {
                for (id, count) in &now {
                    if *count > before.get(id).copied().unwrap_or(0) {
                        new.insert(id.clone());
                    }
                }
            });
        }
        *last = Some(now);
    });
    let was = WAS_OPEN.with(|w| w.replace(open));
    if was && !open && clear_on_close {
        clear();
    }
}

/// No world: forget everything.
pub fn reset() {
    LAST.with_borrow_mut(|l| *l = None);
    clear();
    WAS_OPEN.with(|w| w.set(false));
}

pub fn clear() {
    NEW.with_borrow_mut(BTreeSet::clear);
}

/// Whether items of this kind carry a star.
pub fn is_new(id: &str) -> bool {
    NEW.with_borrow(|n| n.contains(id))
}

/// Marks the nested draw; false when this is the nested one.
pub fn enter() -> bool {
    if INSIDE.with(Cell::get) {
        return false;
    }
    INSIDE.with(|i| i.set(true));
    true
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// The star as rectangles `([x, y, w, h], argb)` relative to the slot's top-left corner; it twinkles over four phases.
pub fn star(millis: u128) -> Vec<([f32; 4], u32)> {
    let phase = (millis / 200) % 4;
    let gold = 0xFF_FFD83A_u32;
    let pale = 0xFF_FFF2A8_u32;
    let arm = if phase.is_multiple_of(2) { 2.0 } else { 1.0 };
    let (centre, arms) = if phase < 2 { (gold, pale) } else { (pale, gold) };
    vec![
        ([1.0, 1.0, 3.0, 3.0], centre),
        ([2.0 - arm + 1.0, 2.0, arm, 1.0], arms),
        ([4.0, 2.0, arm, 1.0], arms),
        ([2.0, 2.0 - arm + 1.0, 1.0, arm], arms),
        ([2.0, 4.0, 1.0, arm], arms),
    ]
}
''',
    tests=r'''
    fn totals(items: &[(&str, i64)]) -> BTreeMap<String, i64> {
        items.iter().map(|(k, v)| ((*k).to_owned(), *v)).collect()
    }

    #[test]
    fn picking_something_up_marks_its_kind() {
        reset();
        tick(totals(&[("stone", 10)]), false, true);
        assert!(!is_new("stone"), "the first look only records");
        tick(totals(&[("stone", 10), ("diamond", 1)]), false, true);
        assert!(is_new("diamond"));
        assert!(!is_new("stone"));
        tick(totals(&[("stone", 12), ("diamond", 1)]), false, true);
        assert!(is_new("stone"), "more of a kind counts");
        tick(totals(&[("stone", 5), ("diamond", 1)]), false, true);
        assert!(is_new("stone"), "losing some does not clear it");
        reset();
    }

    #[test]
    fn closing_the_inventory_clears_when_asked() {
        reset();
        tick(totals(&[]), false, true);
        tick(totals(&[("gold_ingot", 1)]), true, true);
        assert!(is_new("gold_ingot"));
        tick(totals(&[("gold_ingot", 1)]), false, true);
        assert!(!is_new("gold_ingot"), "closed");
        tick(totals(&[("gold_ingot", 2)]), true, false);
        tick(totals(&[("gold_ingot", 2)]), false, false);
        assert!(is_new("gold_ingot"), "kept when the setting is off");
        reset();
        assert!(!is_new("gold_ingot"));
    }

    #[test]
    fn the_nested_draw_is_not_decorated_again() {
        leave();
        assert!(enter());
        assert!(!enter());
        leave();
    }

    #[test]
    fn the_star_twinkles_and_fits_the_corner() {
        let a = star(0);
        let b = star(250);
        assert_ne!(a, b);
        assert_eq!(star(800), star(0), "a cycle is 800 ms");
        for (rect, _) in a.iter().chain(b.iter()) {
            assert!(rect[0] >= 0.0 && rect[1] >= 0.0 && rect[0] + rect[2] <= 8.0 && rect[1] + rect[3] <= 8.0, "{rect:?}");
        }
    }
''',
    helpers={
        "slot": '''// One slot's item on a container screen; a kind picked up recently gets a twinkling star over it.
fn item_highlighter_slot(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    drag: Option<&DragView>,
    slot: usize,
    stack: &Option<ItemStack>,
    x: f32,
    y: f32,
) -> std::ops::ControlFlow<()> {
    use lib_item_highlighter as m;
    let Some(item) = stack else {
        return std::ops::ControlFlow::Continue(());
    };
    if !m::is_new(&item.item) || !m::enter() {
        return std::ops::ControlFlow::Continue(());
    }
    game.draw_slot_item(batch, drag, slot, stack, x, y);
    m::leave();
    let millis = game.started.elapsed().as_millis();
    for ([dx, dy, w, h], argb) in m::star(millis) {
        batch.fill(&game.gui.atlas, x + dx, y + dy, w, h, argb);
    }
    std::ops::ControlFlow::Break(())
}
''',
        "tick": '''// Once a tick: what was picked up, and whether a container screen is open.
fn item_highlighter_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_item_highlighter as m;
    if !game.native_on(m::ID) || !game.native_window() || game.native_player().is_none() {
        m::reset();
        return;
    }
    let mut now: std::collections::BTreeMap<String, i64> = std::collections::BTreeMap::new();
    for stack in game.inventory.iter().flatten().chain(game.offhand.iter()) {
        *now.entry(stack.item.to_string()).or_insert(0) += i64::from(stack.count);
    }
    let open = matches!(game.screen, Screen::Inventory | Screen::Creative);
    m::tick(now, open, m::is(|k| game.native_setting(m::ID, k), "clear"));
}
''',
    },
))
