// Password Hider, a native Horizon mod: Shows the password in /login, /register and /changepassword as asterisks while you type, and keeps it out of the chat history and its file.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_password_hider`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "password_hider";
pub const NAME: &str = "Password Hider";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows the password in /login, /register and /changepassword as asterisks while you type, and keeps it out of the chat history and its file.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("input", "Mask the password while typing", true),
    ("history", "Keep it out of the history and its file", true)
];

thread_local! {
    /// Read once through the game's settings files, then kept.
    static CHOSEN: RefCell<Option<Vec<bool>>> = const { RefCell::new(None) };
}

/// A stored value: `true` or `false`, else the default.
pub fn parse(stored: Option<String>, default: bool) -> bool {
    stored.map_or(default, |v| match v.trim() {
        "true" => true,
        "false" => false,
        _ => default,
    })
}

fn chosen(get: impl Fn(&str) -> Option<String>) -> Vec<bool> {
    CHOSEN.with_borrow_mut(|c| {
        c.get_or_insert_with(|| {
            OPTIONS
                .iter()
                .map(|(key, _, default)| parse(get(key), *default))
                .collect()
        })
        .clone()
    })
}

/// Whether option `key` is on.
pub fn is(get: impl Fn(&str) -> Option<String>, key: &str) -> bool {
    OPTIONS
        .iter()
        .position(|o| o.0 == key)
        .is_some_and(|i| chosen(get)[i])
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let on = chosen(get);
    OPTIONS
        .iter()
        .zip(on)
        .map(|((_, label, _), on)| format!("{label}: {}", if on { "ON" } else { "OFF" }))
        .collect()
}

/// A click on button `i`: flips it and returns what to store (key, value).
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut on = chosen(get);
    let (key, _, _) = *OPTIONS.get(i)?;
    on[i] = !on[i];
    let value = on[i].to_string();
    CHOSEN.with_borrow_mut(|c| *c = Some(on));
    Some((key, value))
}

/// Forgets what was read (tests only).
#[cfg(test)]
pub fn forget_options() {
    CHOSEN.with_borrow_mut(|c| *c = None);
}

use std::cell::Cell;

/// The commands whose arguments are secret, the common ones of login plugins (AuthMe and its relatives).
pub const COMMANDS: [&str; 9] = ["login", "l", "log", "register", "reg", "changepassword", "cp", "unregister", "unreg"];

thread_local! {
    static INPUT: Cell<bool> = const { Cell::new(true) };
    static HISTORY: Cell<bool> = const { Cell::new(true) };
}

/// Sets what the mod does, from the player's settings (the game tells the chat each tick).
pub fn configure(input: bool, history: bool) {
    INPUT.with(|v| v.set(input));
    HISTORY.with(|v| v.set(history));
}

pub fn input_on() -> bool {
    INPUT.with(Cell::get)
}

pub fn history_on() -> bool {
    HISTORY.with(Cell::get)
}

/// The character index where the secret part of `text` starts: just after the first space of a covered command,
/// or none when it is not one or has no space yet.
pub fn secret_start(text: &str) -> Option<usize> {
    let rest = text.strip_prefix('/')?;
    let word_end = rest.find(' ')?;
    let word = rest[..word_end].to_ascii_lowercase();
    COMMANDS.contains(&word.as_str()).then(|| text[..word_end + 2].chars().count())
}

/// Whether `text` is a covered command with something after it, that is, has a secret to hide.
pub fn has_secret(text: &str) -> bool {
    secret_start(text).is_some_and(|from| text.chars().skip(from).any(|c| !c.is_whitespace()))
}

/// `value`, the piece of the box that starts `offset` characters in, with every letter from character `from` on shown as `*`
/// (spaces stay, so you can still see how many passwords you typed).
pub fn mask(value: &str, from: usize, offset: usize) -> String {
    value.chars().enumerate().map(|(i, c)| if offset + i >= from && !c.is_whitespace() { '*' } else { c }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_secret_starts_after_the_first_space_of_a_covered_command() {
        assert_eq!(secret_start("/login hunter2"), Some(7));
        assert_eq!(secret_start("/LOGIN hunter2"), Some(7), "case does not matter");
        assert_eq!(secret_start("/register a b"), Some(10));
        assert_eq!(secret_start("/l pw"), Some(3));
        assert_eq!(secret_start("/login"), None, "no space yet");
        assert_eq!(secret_start("login hunter2"), None, "not a command");
        assert_eq!(secret_start("/msg bob hi"), None);
        assert_eq!(secret_start("/loginx pw"), None, "only the exact words");
        assert_eq!(secret_start("hello /login x"), None);
    }

    #[test]
    fn only_a_command_with_something_after_it_has_a_secret() {
        assert!(has_secret("/login hunter2"));
        assert!(has_secret("/changepassword old new"));
        assert!(!has_secret("/login "));
        assert!(!has_secret("/login    "));
        assert!(!has_secret("/say hunter2"));
    }

    #[test]
    fn masking_keeps_spaces_and_the_command() {
        assert_eq!(mask("/login hunter2", 7, 0), "/login *******");
        assert_eq!(mask("/register abc def", 10, 0), "/register *** ***");
        // The box draws a window of the text: the piece starting 4 characters in.
        assert_eq!(mask("gin hunter2", 7, 4), "gin *******");
        assert_eq!(mask("hunter2", 7, 7), "*******");
        assert_eq!(mask("/login hunter2", 99, 0), "/login hunter2", "nothing masked before the secret");
    }

    #[test]
    fn multibyte_letters_are_counted_as_characters() {
        assert_eq!(secret_start("/login pässwörd"), Some(7));
        assert_eq!(mask("/login pässwörd", 7, 0), "/login ********");
    }

    #[test]
    fn the_switches_work() {
        configure(false, false);
        assert!(!input_on() && !history_on());
        configure(true, true);
        assert!(input_on() && history_on());
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(
            labels(none).len(),
            OPTIONS.len(),
            "one button per option"
        );
        assert_eq!(is(none, OPTIONS[0].0), OPTIONS[0].2);
        forget_options();
        let stored = |k: &str| (k == OPTIONS[0].0).then(|| "false".to_owned());
        assert!(!is(stored, OPTIONS[0].0));
        forget_options();
        assert!(parse(Some("junk".into()), true));
        assert!(!parse(Some(" false\n".into()), true));
    }

    #[test]
    fn a_click_flips_and_reports_what_to_store() {
        forget_options();
        let none = |_: &str| None;
        let (key, value) = click(none, 0).expect("button 0 exists");
        assert_eq!(key, OPTIONS[0].0);
        assert_eq!(value, (!OPTIONS[0].2).to_string());
        assert_eq!(is(none, key), !OPTIONS[0].2);
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
