// /wp: name places, list them, track one.
fn waypoints_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_waypoints as m;
    let mut parts = command.splitn(2, ' ');
    let name = parts.next().unwrap_or("");
    if !game.native_on(m::ID) || name != m::COMMAND {
        return;
    }
    *ran = true;
    let args = parts.next().unwrap_or("");
    {
        let game = &*game;
        m::load(|| game.native_setting(m::ID, m::FILE));
    }
    let player = game.native_player();
    let here = player.as_ref().map(|p| m::Here { x: p.x, y: p.y, z: p.z, dimension: p.dimension.to_owned() });
    let yaw = player.as_ref().map_or(0.0, |p| p.yaw);
    let outcome = m::run(args, here.as_ref(), yaw);
    if let Some(text) = &outcome.save {
        game.native_set_setting(m::ID, m::FILE, text);
    }
    for line in &outcome.chat {
        game.native_chat(line);
    }
}
