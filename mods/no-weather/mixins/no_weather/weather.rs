// Refuses the weather the server is about to start, so the sky stays clear in your own worlds.
fn no_weather_weather(game: &crate::game::Game<'_>, raining: bool, thundering: bool, cancel: &mut bool) {
    use crate::game::lib_no_weather as m;
    if !game.native_on(m::ID) {
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    if !m::is(get, "shown") {
        return;
    }
    if m::refuses(m::value(get, "what"), raining, thundering) {
        *cancel = true;
    }
}
