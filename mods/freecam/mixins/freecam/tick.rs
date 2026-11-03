// Once a tick, before the player's movement is read: the movement keys belong to the camera.
fn freecam_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_freecam as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) || !m::active() {
        return;
    }
    for k in [K::KeyW, K::KeyA, K::KeyS, K::KeyD, K::Space, K::ShiftLeft, K::ControlLeft] {
        game.keys.remove(&k);
    }
}
