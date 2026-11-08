// The mod's settings page: one button per option.
fn crosshair_editor_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::lib_crosshair_editor as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::labels(|k| game.native_setting(m::ID, k)));
}
