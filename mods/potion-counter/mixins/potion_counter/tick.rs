// Once a tick: switch PotionCounter on with these settings (off with the mod).
fn potion_counter_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_potion_counter as m;
    if !game.native_on(m::ID) {
        game.native_potion_counter(None);
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_potion_counter(Some(horizon_client::potion_counter::Config {
        enabled: s.enabled,
        show_upgrades: s.show_upgrades,
        x: -1,
        y: -1,
        disabled: s.disabled,
    }));
}
