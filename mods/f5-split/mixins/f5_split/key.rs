// The back-view and front-view keys: the player's own choice of key.
fn f5_split_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_f5_split as m;
    use crate::game::perspective::CameraType;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let target = if key == game.native_mod_key(m::ID, m::BACK_KEY, m::BACK_DEFAULT) {
        m::View::Back
    } else if key == game.native_mod_key(m::ID, m::FRONT_KEY, m::FRONT_DEFAULT) {
        m::View::Front
    } else {
        return;
    };
    let current = match game.camera_type {
        CameraType::FirstPerson => m::View::First,
        CameraType::ThirdPersonBack => m::View::Back,
        CameraType::ThirdPersonFront => m::View::Front,
    };
    let hold = {
        let game = &*game;
        m::is(|k| game.native_setting(m::ID, k), "hold")
    };
    game.camera_type = match m::after(current, target, pressed, hold) {
        m::View::First => CameraType::FirstPerson,
        m::View::Back => CameraType::ThirdPersonBack,
        m::View::Front => CameraType::ThirdPersonFront,
    };
}
