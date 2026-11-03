// ChatComponent.addMessage with the player's own history limit: the original's steps, with `truncate` taking the mod's limit.
fn more_chat_history_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    content: &Text,
    indicator: Option<u32>,
    source: ChatSource,
    ticks: u64,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    let content = content.clone();
    if !chat.abilities.shows(source) {
        return std::ops::ControlFlow::Break(());
    }
    crate::logging::write_only(log::Level::Info, &format!("[CHAT] {}", content.plain()));
    if chat.log {
        println!("[CHAT] {}", content.plain());
    }
    let message = Message { added: ticks, content, indicator, source };
    chat.add_lines(font, &message, focused);
    chat.messages.insert(0, message);
    chat.messages.truncate(lib_more_chat_history::limit());
    chat.received += 1;
    std::ops::ControlFlow::Break(())
}
