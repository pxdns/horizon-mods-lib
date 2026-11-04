// Once a tick: add the local pearls for the throws, and remove those the server's pearls replaced.
fn pearl_optimizer_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::remote::lib_pearl_optimizer as m;
    use horizon_client::projectiles::{Projectile, ProjectileKind};
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID) && game.remote.is_some() && m::is(|k| game.native_setting(m::ID, k), "fast");
    if !on {
        m::reset();
        game.creatures.projectiles.retain(|p| p.id < m::BASE_ID);
        return;
    }
    let (add, remove) = m::step(game.ticks as u64);
    game.creatures.projectiles.retain(|p| !remove.contains(&p.id));
    for (id, throw) in add {
        let mut pearl = Projectile::new(ProjectileKind::EnderPearl, horizon_core::geometry::Vec3::new(throw.position[0], throw.position[1], throw.position[2]), None);
        pearl.id = id;
        pearl.velocity = horizon_core::geometry::Vec3::new(throw.velocity[0], throw.velocity[1], throw.velocity[2]);
        pearl.yaw = throw.yaw;
        pearl.pitch = throw.pitch;
        game.creatures.projectiles.push(pearl);
    }
}
