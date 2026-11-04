// `/multikey add|remove|list`.
fn multi_key_bindings_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use horizon_client::keybinds::lib_multi_key_bindings as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    let actions: Vec<&str> = horizon_client::keybinds::MAPPINGS.iter().filter(|m| m.2 != "debug").map(|m| m.0).collect();
    let (lines, save) = m::run(&rest, &actions);
    if let Some(text) = save {
        game.native_set_setting(m::ID, "extras", &text);
    }
    if m::is(|k| game.native_setting(m::ID, k), "say") || save_failed(&lines) {
        for line in lines {
            game.native_chat(&line);
        }
    }
}

fn save_failed(lines: &[String]) -> bool {
    lines.iter().any(|l| l.starts_with("Usage") || l.starts_with("Unknown") || l.starts_with("Not a key"))
}
