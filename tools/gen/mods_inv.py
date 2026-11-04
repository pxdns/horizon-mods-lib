"""InvMove: lets you walk with the inventory open by running Horizon's tick as if no screen were open. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, LIST_B, SET_B, CLK_B

INV = []

INV.append(dict(
    slug="invmove", id="invmove", name="InvMove", title="InvMove", raw=True,
    usage="Open your inventory and keep holding W A S D: you keep walking. Space and Shift also jump and sneak unless you switch that off in the settings. Clicking in the inventory never attacks or uses an item.",
    description="Keep walking, jumping and sneaking while your inventory is open.",
    upstream="InvMove (invmove)", upstream_licence="see the upstream project",
    command=None, hooks=["walk", "list", "settings", "click"],
    options=[("jump", "Jump and sneak too", True)],
    raw_note=("It puts a cancellable HEAD hook on `Game::tick`, which runs the tick again with the screen set to the in-game one while a movement key is held and your inventory is open, "
              "and puts the screen back afterwards. Only the survival inventory is covered, not chat, the creative inventory (its search box takes typing) or other screens. "
              "`Game::tick` is not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**."),
    toml_text=toml_text("invmove", "1.0.0", "crates/horizon-client/src/game.rs", "lib_invmove", [
        dict(fields=[("id", '"walk"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client::game::Game<\'_>::tick"'), ("helper", '"walk.rs"')],
             bindings=[G]),
        native("list", "native_mods", LIST_B), native("settings", "native_settings", SET_B), native("click", "native_settings_click", CLK_B),
    ]),
    runtime_extra=r'''
use std::cell::Cell;

thread_local! {
    /// True while the mod runs the tick again.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// Whether to run the tick with no screen open: the inventory is open, a movement key is held and this is
/// not the nested run. Marks the nested run when it answers yes.
pub fn enter(inventory_open: bool, moving: bool) -> bool {
    if INSIDE.with(Cell::get) || !inventory_open || !moving {
        return false;
    }
    INSIDE.with(|i| i.set(true));
    true
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}
''',
    tests=r'''
    #[test]
    fn it_only_runs_with_the_inventory_open_and_a_key_held() {
        leave();
        assert!(!enter(false, true), "no inventory open");
        assert!(!enter(true, false), "nothing held");
        assert!(enter(true, true));
        leave();
    }

    #[test]
    fn the_nested_tick_runs_the_original() {
        leave();
        assert!(enter(true, true));
        assert!(!enter(true, true), "nested");
        leave();
        assert!(enter(true, true), "free again after leave");
        leave();
    }
''',
    helpers={
        "walk": '''// Runs the tick as if no screen were open while the inventory is open and a movement key is held.
fn invmove_walk(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<Result<()>> {
    use crate::game::lib_invmove as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    let moving = [K::KeyW, K::KeyA, K::KeyS, K::KeyD, K::Space, K::ShiftLeft].iter().any(|k| game.keys.contains(k));
    if !m::enter(game.screen == Screen::Inventory, moving) {
        return std::ops::ControlFlow::Continue(());
    }
    let jump = m::is(|k| game.native_setting(m::ID, k), "jump");
    let held_jump = [K::Space, K::ShiftLeft].map(|k| game.keys.contains(&k));
    if !jump {
        game.keys.remove(&K::Space);
        game.keys.remove(&K::ShiftLeft);
    }
    // Clicks made in the inventory must not attack or use items in the world.
    let (left, right) = (game.left, game.right);
    game.screen = Screen::Playing;
    game.left = false;
    game.right = false;
    let result = game.tick();
    if game.screen == Screen::Playing {
        game.screen = Screen::Inventory;
    }
    game.left = left;
    game.right = right;
    for (k, held) in [K::Space, K::ShiftLeft].into_iter().zip(held_jump) {
        if held {
            game.keys.insert(k);
        }
    }
    m::leave();
    std::ops::ControlFlow::Break(result)
}
''',
    },
))
