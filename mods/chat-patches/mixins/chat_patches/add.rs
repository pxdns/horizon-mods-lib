// A new chat line: dropped when it repeats the last one, else shown again with a stamp in front.
fn chat_patches_add(
    chat: &mut Chat,
    font: &crate::font::Font,
    content: &Text,
    tag: &Option<std::sync::Arc<Tag>>,
    source: ChatSource,
    ticks: u64,
    focused: bool,
) -> std::ops::ControlFlow<()> {
    use lib_chat_patches as m;
    if !m::stamps_on() && !m::repeats_on() {
        return std::ops::ControlFlow::Continue(());
    }
    if !m::enter() {
        return std::ops::ControlFlow::Continue(());
    }
    let plain = content.plain();
    if m::repeats_on() && m::repeat(&plain, ticks) {
        m::leave();
        return std::ops::ControlFlow::Break(());
    }
    let mut shown = content.clone();
    if m::stamps_on() {
        use chrono::Timelike;
        let now = chrono::Local::now();
        let mut stamped = Text::empty();
        stamped.push(Text::styled(m::stamp(now.hour(), now.minute(), now.second(), m::seconds_on()), Style::color(text::GRAY)));
        stamped.push(content.clone());
        shown = stamped;
    }
    chat.add_message(font, shown, tag.clone(), source, ticks, focused);
    m::leave();
    std::ops::ControlFlow::Break(())
}
