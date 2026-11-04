// A packet leaves for the server: using an ender pearl starts a local pearl.
fn pearl_optimizer_send(game: &crate::game::Game<'_>, packet: &S) {
    use lib_pearl_optimizer as m;
    let S::UseItem(used) = packet else {
        return;
    };
    let stack = if used.hand == 0 { game.held() } else { game.offhand.as_ref() };
    let Some(stack) = stack.filter(|s| &*s.item == "minecraft:ender_pearl") else {
        return;
    };
    let group = horizon_client::cooldowns::group(stack, game.items().get(&stack.item).and_then(|i| i.use_cooldown.as_ref()));
    if game.screen != Screen::Playing || game.cooldowns.is_on_cooldown(&group) {
        return;
    }
    let (Some(eye), Some(player)) = (game.eye_position(), game.player.as_ref()) else {
        return;
    };
    m::thrown(m::throw(eye, game.yaw, game.pitch, [player.velocity.x, player.velocity.y, player.velocity.z], player.ground.on_ground));
}
