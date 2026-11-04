// A sent line goes into the history (and, for a command, the history file): not one with a password in it.
fn password_hider_recent(message: &str) -> std::ops::ControlFlow<()> {
    use lib_password_hider as m;
    if m::history_on() && m::has_secret(message) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
