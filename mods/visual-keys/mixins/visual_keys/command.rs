// `/keyboard`, `/keyboard free`, `/keyboard <key>`.
fn visual_keys_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_visual_keys as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let bindings: Vec<(String, String)> = horizon_client::keybinds::MAPPINGS
        .iter()
        .map(|(action, _, _)| ((*action).to_owned(), game.settings.keys.get(action).to_owned()))
        .collect();
    match words.next() {
        None => {
            let stored = {
                let game = &*game;
                m::click(|k| game.native_setting(m::ID, k), 0)
            };
            if let Some((key, value)) = stored {
                game.native_set_setting(m::ID, key, &value);
                game.native_chat(&format!("{} {}", m::NAME, if value == "true" { "shown" } else { "hidden" }));
            }
        }
        Some("free") => game.native_chat(&m::free_keys(&bindings)),
        Some(key) => game.native_chat(&m::describe(key, &bindings)),
    }
}
