// A click on the settings page: flip the option and keep it.
fn more_chat_history_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use horizon_client::chat::lib_more_chat_history as m;
    if mod_id != m::ID {
        return;
    }
    let stored = {
        let game = &*game;
        m::click(|k| game.native_setting(m::ID, k), button)
    };
    if let Some((key, value)) = stored {
        game.native_set_setting(m::ID, key, &value);
    }
}
