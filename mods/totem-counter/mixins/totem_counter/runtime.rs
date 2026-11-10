// TotemCounter, a native Horizon mod: Counts the amounts of skill issues (totem pops).
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_totem_counter`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "totem_counter";
pub const NAME: &str = "TotemCounter";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Counts the amounts of skill issues (totem pops).";

/// The reset key's name in Controls, and its default (C).
pub const KEY: &str = "totemcounter_reset";
pub const DEFAULT_KEY: &str = "key.keyboard.c";
/// `TotemCounter.PREFIX`, in section codes.
pub const PREFIX: &str = "\u{a7}e\u{a7}lTotem\u{a7}a\u{a7}lCounter\u{a7}7\u{a7}l \u{bb} \u{a7}r";
pub const HEADER: &str = " \u{a7}7======\u{a7}e\u{a7}lTotem\u{a7}a\u{a7}lCounter\u{a7}7 ======\u{a7}r";

/// `ModConfig` of the original, with its defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub counter_enabled: bool,
    pub counter_colors: bool,
    pub show_in_tab: bool,
    pub separator: bool,
    pub display_enabled: bool,
    pub use_default_totem: bool,
    pub display_colors: bool,
    pub colored_xp_bar: bool,
    pub always_show_bar: bool,
    pub show_pop_counter: bool,
    /// 0 is the default place over the experience bar, 1 the top left corner.
    pub position: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            counter_enabled: true,
            counter_colors: true,
            show_in_tab: false,
            separator: true,
            display_enabled: true,
            use_default_totem: false,
            display_colors: true,
            colored_xp_bar: false,
            always_show_bar: false,
            show_pop_counter: false,
            position: 0,
        }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        Self {
            counter_enabled: flag("counter", d.counter_enabled),
            counter_colors: flag("counter_colors", d.counter_colors),
            show_in_tab: flag("tab", d.show_in_tab),
            separator: flag("separator", d.separator),
            display_enabled: flag("display", d.display_enabled),
            use_default_totem: flag("default_totem", d.use_default_totem),
            display_colors: flag("display_colors", d.display_colors),
            colored_xp_bar: flag("xp_bar", d.colored_xp_bar),
            always_show_bar: flag("always_bar", d.always_show_bar),
            show_pop_counter: flag("own_pops", d.show_pop_counter),
            position: get("position").and_then(|v| v.trim().parse().ok()).filter(|v| *v <= 1).unwrap_or(d.position),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Pop Counter Enabled: {}", on(self.counter_enabled)),
            format!("Pop Counter Colors: {}", on(self.counter_colors)),
            format!("Show In Player List: {}", on(self.show_in_tab)),
            format!("Separator: {}", on(self.separator)),
            format!("Totem Display Enabled: {}", on(self.display_enabled)),
            format!("Default Totem Icon: {}", on(self.use_default_totem)),
            format!("Display Colors: {}", on(self.display_colors)),
            format!("Colored XP Bar: {}", on(self.colored_xp_bar)),
            format!("Always Show Colored Bar: {}", on(self.always_show_bar)),
            format!("Show Own Pops: {}", on(self.show_pop_counter)),
            format!("Position: {}", if self.position == 0 { "default" } else { "top left" }),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let flip = |v: &mut bool, key: &'static str| {
            *v = !*v;
            Some((key, v.to_string()))
        };
        match i {
            0 => flip(&mut self.counter_enabled, "counter"),
            1 => flip(&mut self.counter_colors, "counter_colors"),
            2 => flip(&mut self.show_in_tab, "tab"),
            3 => flip(&mut self.separator, "separator"),
            4 => flip(&mut self.display_enabled, "display"),
            5 => flip(&mut self.use_default_totem, "default_totem"),
            6 => flip(&mut self.display_colors, "display_colors"),
            7 => flip(&mut self.colored_xp_bar, "xp_bar"),
            8 => flip(&mut self.always_show_bar, "always_bar"),
            9 => flip(&mut self.show_pop_counter, "own_pops"),
            10 => {
                self.position = 1 - self.position;
                Some(("position", self.position.to_string()))
            }
            _ => None,
        }
    }
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    Settings::read(get).labels()
}

pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    Settings::read(get).click(i)
}

/// `/showpops` and `/resetcounter`: what they say. `pops` is everyone's, most first.
pub fn show(player: Option<&str>, pops: &[(String, i32)]) -> Vec<String> {
    match player {
        Some(name) => match pops.iter().find(|(n, _)| n == name) {
            Some((_, n)) => vec![format!("{PREFIX}{name} popped {n} totem(s)!")],
            None => vec![format!("{PREFIX}{name} never popped a totem. Simply built different.")],
        },
        None if pops.is_empty() => vec![format!("{PREFIX}No one popped totems D:")],
        None => {
            let mut lines = vec![HEADER.to_owned()];
            lines.extend(pops.iter().map(|(n, c)| format!("{n}: {c}")));
            lines
        }
    }
}

pub fn reset_message(player: Option<&str>, found: bool) -> String {
    match (player, found) {
        (None, _) => "\u{a7}bSuccessfully reset the pop counter!".to_owned(),
        (Some(name), true) => format!("\u{a7}bSuccessfully reset counter for {name}\u{a7}b!"),
        (Some(name), false) => format!("{PREFIX}{name} has no pops to reset."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let s = Settings::read(none);
        assert!(s.counter_enabled && s.counter_colors && s.separator && s.display_enabled && s.display_colors);
        assert!(!s.show_in_tab && !s.use_default_totem && !s.colored_xp_bar && !s.always_show_bar && !s.show_pop_counter);
        assert_eq!(s.position, 0);
    }

    #[test]
    fn there_is_a_button_for_every_setting() {
        assert_eq!(labels(none).len(), 11);
        assert_eq!(click(none, 0).unwrap(), ("counter", "false".to_owned()));
        assert_eq!(click(none, 7).unwrap(), ("xp_bar", "true".to_owned()));
        assert_eq!(click(none, 10).unwrap(), ("position", "1".to_owned()));
        assert!(click(none, 11).is_none());
    }

    #[test]
    fn showpops_says_what_the_original_says() {
        let pops = vec![("alex".to_owned(), 3), ("sam".to_owned(), 1)];
        assert!(show(None, &[])[0].ends_with("No one popped totems D:"));
        assert!(show(Some("alex"), &pops)[0].ends_with("alex popped 3 totem(s)!"));
        assert!(show(Some("bob"), &pops)[0].ends_with("bob never popped a totem. Simply built different."));
        let all = show(None, &pops);
        assert_eq!((all.len(), all[1].as_str(), all[2].as_str()), (3, "alex: 3", "sam: 1"));
    }

    #[test]
    fn reset_messages_match() {
        assert_eq!(reset_message(None, true), "\u{a7}bSuccessfully reset the pop counter!");
        assert_eq!(reset_message(Some("sam"), true), "\u{a7}bSuccessfully reset counter for sam\u{a7}b!");
    }

    #[test]
    fn junk_falls_back_to_the_defaults() {
        let junk = |_: &str| Some("banana".to_owned());
        assert_eq!(Settings::read(junk), Settings::default());
    }
}
