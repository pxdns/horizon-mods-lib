// Once a tick: hand the game the chat animation these settings give (vanilla chat when the mod is off).
fn chat_animation_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_chat_animation as m;
    if !game.native_on(m::ID) {
        game.native_chat_animation_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_chat_animation(s.animate_messages, s.message_ms, s.animate_opacity, s.hide_indicator, s.animate_input, s.input_ms);
}
