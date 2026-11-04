// A packet leaves for the server: an attack on one of the End crystals it told us about starts the hiding.
fn crystal_optimizer_send(game: &crate::game::Game<'_>, packet: &S) {
    use lib_crystal_optimizer as m;
    if let S::Attack(attack) = packet
        && let Ok(id) = u32::try_from(attack.id)
        && game.crystals.iter().any(|c| c.id == id && !c.invulnerable)
    {
        m::hit(id);
    }
}
