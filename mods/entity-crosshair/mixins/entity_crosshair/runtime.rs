// Entity Crosshair, a native Horizon mod: Replaces the crosshair with a style and colour of your choice, red over living entities.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_entity_crosshair`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "entity_crosshair";
pub const NAME: &str = "Entity Crosshair";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Replaces the crosshair with a style and colour of your choice, red over living entities.";

use std::cell::Cell;

/// The shapes the first button cycles through, and their names.
pub const STYLES: [&str; 5] = ["Plus", "Gap plus", "Small plus", "Dot", "Ring"];
/// The colours the second button cycles through: name and ARGB.
pub const COLOURS: [(&str, u32); 5] = [
    ("White", 0xFFFF_FFFF),
    ("Green", 0xFF55_FF55),
    ("Cyan", 0xFF55_FFFF),
    ("Yellow", 0xFFFF_FF55),
    ("Magenta", 0xFFFF_55FF),
];
/// What the crosshair turns over a living entity.
pub const RED: u32 = 0xFFFF_5555;
/// The crosshair sprite is 15 by 15 pixels; shapes are rectangles (x, y, width, height) inside it.
pub type Rect = (i32, i32, i32, i32);

/// The settings, one stored file per key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub style: usize,
    pub colour: usize,
    /// Turn red over a living, alive entity.
    pub red: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            style: 0,
            colour: 0,
            red: true,
        }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let index = |key: &str, len: usize, default: usize| {
            get(key)
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|&i| i < len)
                .unwrap_or(default)
        };
        Self {
            style: index("style", STYLES.len(), d.style),
            colour: index("colour", COLOURS.len(), d.colour),
            red: get("red").map_or(d.red, |v| match v.trim() {
                "true" => true,
                "false" => false,
                _ => d.red,
            }),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Style: {}", STYLES[self.style]),
            format!("Colour: {}", COLOURS[self.colour].0),
            format!("Red over mobs: {}", if self.red { "ON" } else { "OFF" }),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        match i {
            0 => {
                self.style = (self.style + 1) % STYLES.len();
                Some(("style", self.style.to_string()))
            }
            1 => {
                self.colour = (self.colour + 1) % COLOURS.len();
                Some(("colour", self.colour.to_string()))
            }
            2 => {
                self.red = !self.red;
                Some(("red", self.red.to_string()))
            }
            _ => None,
        }
    }
}

/// The rectangles of a style, from the sprite's top-left corner; the crosshair's centre pixel is (7, 7).
pub fn shape(style: usize) -> Vec<Rect> {
    match style {
        0 => vec![(3, 7, 9, 1), (7, 3, 1, 9)],
        1 => vec![(2, 7, 4, 1), (9, 7, 4, 1), (7, 2, 1, 4), (7, 9, 1, 4)],
        2 => vec![(5, 7, 5, 1), (7, 5, 1, 5)],
        3 => vec![(6, 6, 3, 3)],
        _ => vec![(4, 3, 7, 1), (4, 11, 7, 1), (3, 4, 1, 7), (11, 4, 1, 7)],
    }
}

/// The colour to draw in.
pub fn colour(s: Settings, over_living: bool) -> u32 {
    if s.red && over_living {
        RED
    } else {
        COLOURS[s.colour].1
    }
}

thread_local! {
    static STATE: Cell<Option<(Settings, bool)>> = const { Cell::new(None) };
}

/// The game tells the render code each frame: the settings (None when the mod is off) and whether a living entity is under the crosshair.
pub fn configure(settings: Option<Settings>, over_living: bool) {
    STATE.with(|s| s.set(settings.map(|v| (v, over_living))));
}

/// What to draw this frame: the shape and its colour; None when the mod is off.
pub fn current() -> Option<(Vec<Rect>, u32)> {
    let (s, living) = STATE.with(Cell::get)?;
    Some((shape(s.style), colour(s, living)))
}

/// Whether a living, alive entity is under the crosshair.
pub fn active_target(living: Option<bool>) -> bool {
    living == Some(true)
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut now = Settings::read(get);
    now.click(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn every_shape_fits_in_the_sprite_and_stays_on_its_centre() {
        for style in 0..STYLES.len() {
            for (x, y, w, h) in shape(style) {
                assert!(
                    x >= 0 && y >= 0 && x + w <= 15 && y + h <= 15,
                    "style {style}: {x} {y} {w} {h}"
                );
            }
        }
        // The plus, small plus and dot are symmetric around the centre pixel.
        for style in [0, 2, 3] {
            let rects = shape(style);
            let left = rects.iter().map(|r| r.0).min().unwrap();
            let right = rects.iter().map(|r| r.0 + r.2 - 1).max().unwrap();
            assert_eq!(left + right, 14, "style {style} is centred on pixel 7");
        }
    }

    #[test]
    fn red_over_a_living_entity_unless_switched_off() {
        let s = Settings::default();
        assert_eq!(colour(s, false), 0xFFFF_FFFF);
        assert_eq!(colour(s, true), RED);
        assert_eq!(colour(Settings { red: false, ..s }, true), 0xFFFF_FFFF);
        assert_eq!(colour(Settings { colour: 1, ..s }, false), 0xFF55_FF55);
    }

    #[test]
    fn the_buttons_cycle_and_report_what_to_store() {
        let mut s = Settings::default();
        assert_eq!(s.click(0), Some(("style", "1".to_owned())));
        for _ in 0..4 {
            s.click(0);
        }
        assert_eq!(s.style, 0, "style wraps");
        assert_eq!(s.click(2), Some(("red", "false".to_owned())));
        assert!(s.click(5).is_none());
        assert_eq!(labels(none)[0], "Style: Plus");
    }

    #[test]
    fn stored_junk_and_out_of_range_fall_back() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
        let far = |k: &str| (k == "style").then(|| "9".to_owned());
        assert_eq!(Settings::read(far).style, 0);
    }

    #[test]
    fn the_frame_state_decides_what_is_drawn() {
        configure(None, true);
        assert!(current().is_none(), "off: Horizon's own crosshair");
        configure(Some(Settings::default()), false);
        assert_eq!(current().unwrap().1, 0xFFFF_FFFF);
        configure(Some(Settings::default()), true);
        assert_eq!(current().unwrap().1, RED);
        configure(None, false);
        assert!(active_target(Some(true)) && !active_target(Some(false)) && !active_target(None));
    }
}
