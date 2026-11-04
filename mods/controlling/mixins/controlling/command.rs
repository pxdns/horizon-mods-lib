// `/keybinds find|conflicts|unbound`.
fn controlling_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_controlling as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    let mut bindings: Vec<(String, String)> = horizon_client::keybinds::MAPPINGS
        .iter()
        .map(|(action, ..)| ((*action).to_owned(), game.settings.keys.get(action).to_owned()))
        .collect();
    let request = m::request(&rest);
    if matches!(request, m::Request::Find(_)) && m::is(|k| game.native_setting(m::ID, k), "hide_none") {
        bindings.retain(|(_, key)| m::key_name(key).is_some());
    }
    for line in m::answer(&request, &bindings) {
        game.native_chat(&line);
    }
}
