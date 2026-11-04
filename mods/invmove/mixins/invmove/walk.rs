// Runs the tick as if no screen were open while the inventory is open and a movement key is held.
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
