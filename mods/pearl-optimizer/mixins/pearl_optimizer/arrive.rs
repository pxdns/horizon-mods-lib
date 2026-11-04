// A projectile arrives from the server: a pearl takes the place of a local one.
fn pearl_optimizer_add(p: &horizon_client::projectiles::Projectile) {
    if matches!(p.kind, horizon_client::projectiles::ProjectileKind::EnderPearl) && p.id < crate::game::remote::lib_pearl_optimizer::BASE_ID {
        crate::game::remote::lib_pearl_optimizer::server_pearl();
    }
}
