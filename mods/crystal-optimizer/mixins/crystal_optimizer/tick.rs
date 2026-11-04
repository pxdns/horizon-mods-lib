// Once a tick: take the crystals the player hit out of the list, and put back any the server did not remove.
fn crystal_optimizer_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::remote::lib_crystal_optimizer as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID) && game.remote.is_some() && m::is(|k| game.native_setting(m::ID, k), "hide");
    if !on {
        m::reset();
        return;
    }
    let now = game.ticks as u64;
    for id in m::take_requests() {
        if let Some(at) = game.crystals.iter().position(|c| c.id == id) {
            let crystal = game.crystals.remove(at);
            m::keep(id, now, Box::new(crystal));
        }
    }
    for back in m::expired(now) {
        if let Ok(crystal) = back.downcast::<crate::game::dragon::Crystal>() {
            game.crystals.push(*crystal);
        }
    }
}
