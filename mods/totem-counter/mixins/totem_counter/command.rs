// `/resetcounter [player]` and `/showpops [player]`.
fn totem_counter_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_totem_counter as m;
    let mut words = command.split(' ');
    let name = words.next().unwrap_or("");
    if !game.native_on(m::ID) || !matches!(name, "resetcounter" | "showpops") {
        return;
    }
    *ran = true;
    let player = words.next().filter(|p| !p.is_empty());
    if name == "resetcounter" {
        let found = game.native_totem_counter_reset(player);
        game.native_chat(&m::reset_message(player, found));
    } else {
        let pops = horizon_client::totem_counter::all_pops();
        for line in m::show(player, &pops) {
            game.native_chat(&line);
        }
    }
}
