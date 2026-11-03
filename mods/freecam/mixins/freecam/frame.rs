// Each frame: fly the camera with the movement keys and keep them from moving the player.
fn freecam_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::lib_freecam as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) || !m::active() {
        return;
    }
    let fast = m::is(|k| game.native_setting(m::ID, k), "fast");
    let held = |k: K| game.physical_keys.contains(&k);
    let input = m::Input {
        forward: held(K::KeyW),
        back: held(K::KeyS),
        left: held(K::KeyA),
        right: held(K::KeyD),
        up: held(K::Space),
        down: held(K::ShiftLeft),
        fast: held(K::ControlLeft),
    };
    if game.native_playing() {
        m::step(elapsed.as_secs_f32(), input, fast);
    }
    for k in [K::KeyW, K::KeyA, K::KeyS, K::KeyD, K::Space, K::ShiftLeft, K::ControlLeft] {
        game.keys.remove(&k);
    }
}
