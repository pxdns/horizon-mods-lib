// A chat line arrived (before the original runs).
fn chat_notify_message(content: &Text, source: ChatSource) {
    lib_chat_notify::message(&content.plain(), matches!(source, ChatSource::Player), matches!(source, ChatSource::Server));
}
