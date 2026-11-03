// Once a tick, before movement is read: of two opposite keys held together, only the last pressed counts.
fn snappy_tappy_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_snappy_tappy as m;
    use winit::keyboard::KeyCode as K;
    if !game.native_on(m::ID) || !game.native_window() || !game.native_playing() {
        return;
    }
    let (sideways, forwards) = {
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "sideways"), m::is(get, "forwards"))
    };
    let (last_sideways, last_forwards) = m::last();
    let held = |k: K| game.physical_keys.contains(&k);
    let pairs = [
        (sideways, K::KeyA, K::KeyD, held(K::KeyA), held(K::KeyD), last_sideways),
        (forwards, K::KeyW, K::KeyS, held(K::KeyW), held(K::KeyS), last_forwards),
    ];
    for (on, first, second, first_held, second_held, last) in pairs {
        if !on {
            continue;
        }
        let (a, b) = m::resolve(first_held, second_held, last);
        for (key, want) in [(first, a), (second, b)] {
            if want {
                game.keys.insert(key);
            } else {
                game.keys.remove(&key);
            }
        }
    }
}
