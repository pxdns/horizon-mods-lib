// ChatComponent.addMessage with the player's own history limit: `push_message`'s steps (every message passes through it,
// a signed player message too), with `truncate` taking the mod's limit.
fn more_chat_history_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    message: &Message,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    let message = message.clone();
    if !chat.abilities.shows(message.source) {
        return std::ops::ControlFlow::Break(());
    }
    let plain = message.content.plain();
    crate::logging::write_only(log::Level::Info, &format!("[CHAT] {plain}"));
    if chat.log {
        println!("[CHAT] {plain}");
    }
    chat.add_lines(font, &message, focused);
    chat.messages.insert(0, message);
    chat.messages.truncate(lib_more_chat_history::limit());
    chat.received += 1;
    std::ops::ControlFlow::Break(())
}
