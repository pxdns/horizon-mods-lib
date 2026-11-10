// Once a tick: move the hotbar's attack indicator out of the icon's way (put back when the mod is off).
fn sprint_indicator_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_sprint_indicator as m;
    let shift = if game.native_on(m::ID) { m::Settings::read(|k| game.native_setting(m::ID, k)).attack_shift() } else { 0 };
    // The shift goes outwards from the hotbar on whichever side the attack indicator is on, as the game places it.
    game.native_attack_indicator_shift(shift);
}
