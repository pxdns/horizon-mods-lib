// A new chat line from a player: shown again with the player's face in front of it.
fn chat_heads_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    content: &Text,
    indicator: Option<u32>,
    source: ChatSource,
    ticks: u64,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    use lib_chat_heads as m;
    if !m::enabled() || !m::enter() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(name) = m::sender(&content.plain()) else {
        m::leave();
        return std::ops::ControlFlow::Continue(());
    };
    let json: serde_json::Value = serde_json::from_str(&m::head_json(&name, m::hat())).unwrap_or(serde_json::Value::Null);
    let mut shown = crate::component::to_text(&json, &mut |_| Text::empty());
    shown.push(Text::literal(" "));
    shown.push(content.clone());
    chat.add_message(font, shown, indicator, source, ticks, focused);
    m::leave();
    std::ops::ControlFlow::Break(())
}
