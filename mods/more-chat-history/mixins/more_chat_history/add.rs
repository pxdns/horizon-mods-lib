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
    // `add_lines` with the mod's limit (Horizon's own has a closure in it, which a constant cannot be changed in).
    let icon = if message.tag.as_ref().is_some_and(|tag| tag.icon) { 9 + 4 + 2 } else { 0 };
    let width = chat.max_width() - icon;
    let content = chat.displayed(&message.content);
    for line in font.wrap_components(&content, width) {
        if focused && chat.scroll > 0 {
            chat.new_since_scroll = true;
            chat.scroll_by(1, focused);
        }
        chat.lines.insert(0, Line { added: message.added, content: line, tag: message.tag.clone() });
    }
    chat.lines.truncate(lib_more_chat_history::limit());
    chat.messages.insert(0, message);
    chat.messages.truncate(lib_more_chat_history::limit());
    chat.received += 1;
    std::ops::ControlFlow::Break(())
}
