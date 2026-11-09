// The mod's settings page: one button per option.
fn held_item_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::lib_held_item as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::labels(|k| game.native_setting(m::ID, k)));
}
