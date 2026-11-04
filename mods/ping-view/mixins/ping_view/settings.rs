// The mod's settings page: one button per option.
fn ping_view_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client::debug_chart::lib_ping_view as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::labels(|k| game.native_setting(m::ID, k)));
}
