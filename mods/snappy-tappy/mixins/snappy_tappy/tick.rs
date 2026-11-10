// Once a tick, before movement is read: of two opposite bound keys held together, only the last pressed counts.
fn snappy_tappy_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_snappy_tappy as m;
    m::next_tick();
    if !game.native_on(m::ID) || !game.native_window() || !game.native_playing() {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "enabled") {
        return;
    }
    // The keys the player bound, as the original reads Options.keyLeft and the rest.
    let bound = |mapping: &str| {
        let name = game.settings.keys.get(mapping).to_owned();
        horizon_client::keybinds::key_code(&name).map(|code| (name, code))
    };
    for (first, second) in [("key.left", "key.right"), ("key.forward", "key.back")] {
        let (Some((first_name, first_code)), Some((second_name, second_code))) = (bound(first), bound(second)) else {
            continue;
        };
        let held = |k| game.physical_keys.contains(&k);
        let (a, b) = m::resolve(held(first_code), held(second_code), m::press_tick(&first_name), m::press_tick(&second_name));
        for (key, want) in [(first_code, a), (second_code, b)] {
            if want {
                game.keys.insert(key);
            } else {
                game.keys.remove(&key);
            }
        }
    }
}
