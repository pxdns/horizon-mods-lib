// `/potions` hides or shows the box.
fn potion_counter_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_potion_counter as m;
    let name = command.split(' ').next().unwrap_or("");
    if !game.native_on(m::ID) || name != m::COMMAND {
        return;
    }
    *ran = true;
    let stored = {
        let game = &*game;
        m::click(|k| game.native_setting(m::ID, k), 0)
    };
    if let Some((key, value)) = stored {
        game.native_set_setting(m::ID, key, &value);
        game.native_chat(&format!(
            "{} {}",
            m::NAME,
            if value == "true" { "shown" } else { "hidden" }
        ));
    }
}
