// The chat box draws its text through this formatter: a covered command's password shows as asterisks.
fn password_hider_format(screen: &ChatScreen, value: &str, offset: usize) -> std::ops::ControlFlow<Text> {
    use lib_password_hider as m;
    if !m::input_on() {
        return std::ops::ControlFlow::Continue(());
    }
    match m::secret_start(&screen.input.value) {
        Some(from) => std::ops::ControlFlow::Break(Text::literal(m::mask(value, from, offset))),
        None => std::ops::ControlFlow::Continue(()),
    }
}
