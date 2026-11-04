// A click on the settings page: flip the option and keep it.
fn password_hider_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use horizon_client::chat::lib_password_hider as m;
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
