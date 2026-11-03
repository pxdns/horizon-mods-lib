// The key went down or up: the player's own choice of key.
fn freecam_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_freecam as m;
    if !game.native_on(m::ID) {
        return;
    }
    if key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let playing = game.native_playing();
    m::key(playing, pressed, game.yaw, game.pitch);
    if !m::active() {
        // Give the movement keys back that are still held.
        use winit::keyboard::KeyCode as K;
        for k in [K::KeyW, K::KeyA, K::KeyS, K::KeyD, K::Space, K::ShiftLeft, K::ControlLeft] {
            if game.physical_keys.contains(&k) {
                game.keys.insert(k);
            }
        }
    }
}
