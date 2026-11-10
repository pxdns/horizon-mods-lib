// Simple Zoom, a native Horizon mod: A simple zoom: hold a key, scroll to change the magnification. No unnecessary stuff.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_simple_zoom`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "simple_zoom";
pub const NAME: &str = "Simple Zoom";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "A simple zoom: hold a key, scroll to change the magnification. No unnecessary stuff.";

use std::cell::Cell;

/// `key.simplezoom.toggle`, "Zoom" in the category "Simple Zoom"; the original's default is `C` (its page says so).
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "key.keyboard.c";
/// `getDefaultZoomLevel`.
pub const DEFAULT_ZOOM: f64 = 3.0;
/// `handleMouseScroll`: each wheel event multiplies the zoom by this up or down, between 1 and 50.
pub const UP: f64 = 1.1;
pub const DOWN: f64 = 0.9;
pub const MIN: f64 = 1.0;
pub const MAX: f64 = 50.0;

thread_local! {
    static HELD: Cell<bool> = const { Cell::new(false) };
    static ZOOM: Cell<f64> = const { Cell::new(DEFAULT_ZOOM) };
}

/// The zoom key went down or up. Letting go resets the zoom (`resetZoomLevel`, which the original does whenever the field of view is
/// asked for with the key up).
pub fn key(playing: bool, pressed: bool) {
    if pressed && !playing {
        return;
    }
    HELD.with(|h| h.set(pressed));
    if !pressed {
        ZOOM.with(|z| z.set(DEFAULT_ZOOM));
    }
}

/// A wheel event (up is positive): true while the key is held, when the hotbar must not scroll (`canScrollInventory`).
pub fn wheel(steps: f32) -> bool {
    if !HELD.with(Cell::get) {
        return false;
    }
    ZOOM.with(|z| {
        let mut v = z.get();
        if steps > 0.0 {
            v *= UP;
        } else if steps < 0.0 {
            v *= DOWN;
        }
        z.set(v.clamp(MIN, MAX));
    });
    true
}

/// The zoom to ask the game for while the key is held: the factor (the field of view is divided by it, at once), and the turning scale
/// (mouse movement is divided by the zoom).
pub fn view() -> Option<(f32, f32)> {
    if !HELD.with(Cell::get) {
        return None;
    }
    let z = ZOOM.with(Cell::get);
    Some((z as f32, (1.0 / z) as f32))
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget() {
    HELD.with(|h| h.set(false));
    ZOOM.with(|z| z.set(DEFAULT_ZOOM));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holding_the_key_zooms_three_times_and_letting_go_stops() {
        forget();
        assert_eq!(view(), None);
        key(true, true);
        let (factor, turn) = view().unwrap();
        assert_eq!(factor, 3.0);
        assert!((turn - 1.0 / 3.0).abs() < 1e-6, "mouse turning is divided by the zoom");
        key(true, false);
        assert_eq!(view(), None);
    }

    #[test]
    fn the_wheel_multiplies_by_1_1_and_0_9_between_1_and_50() {
        forget();
        key(true, true);
        wheel(1.0);
        assert!((view().unwrap().0 - 3.3).abs() < 1e-5);
        wheel(-2.0);
        assert!((view().unwrap().0 - 2.97).abs() < 1e-5, "a step is a step whatever its size");
        for _ in 0..200 {
            wheel(1.0);
        }
        assert_eq!(view().unwrap().0, 50.0);
        for _ in 0..200 {
            wheel(-1.0);
        }
        assert_eq!(view().unwrap().0, 1.0);
        forget();
    }

    #[test]
    fn the_zoom_resets_when_the_key_is_let_go() {
        forget();
        key(true, true);
        wheel(1.0);
        wheel(1.0);
        key(true, false);
        key(true, true);
        assert_eq!(view().unwrap().0, 3.0);
        forget();
    }

    #[test]
    fn the_wheel_only_belongs_to_the_mod_while_the_key_is_held() {
        forget();
        assert!(!wheel(1.0), "the hotbar scrolls as usual");
        key(true, true);
        assert!(wheel(1.0), "the hotbar does not");
        key(true, false);
        assert!(!wheel(1.0));
        forget();
    }

    #[test]
    fn a_press_outside_the_game_does_nothing() {
        forget();
        key(false, true);
        assert_eq!(view(), None);
    }
}
