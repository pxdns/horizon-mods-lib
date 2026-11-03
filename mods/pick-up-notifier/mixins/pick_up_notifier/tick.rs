// Once a tick: what the player carries, so the changes can be listed.
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
