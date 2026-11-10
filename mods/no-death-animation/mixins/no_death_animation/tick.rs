// Once a tick: hand the game the death style these settings give (nothing when the mod is off).
fn no_death_animation_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_no_death_animation as m;
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_death_style(game.native_on(m::ID) && s.enabled, &s.affected, s.death_time, s.overlay_time);
}
