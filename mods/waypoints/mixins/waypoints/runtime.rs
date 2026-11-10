// Waypoints, a native Horizon mod: Name places with /wp, list them, and track one: its distance and direction show on screen.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_waypoints`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "waypoints";
pub const NAME: &str = "Waypoints";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Name places with /wp, list them, and track one: its distance and direction show on screen.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Show tracked waypoint", true)
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
/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;

/// The command: `/wp add|del|list|track|untrack`.
pub const COMMAND: &str = "wp";
/// The settings file that holds every waypoint.
pub const FILE: &str = "waypoints";

/// A named place.
#[derive(Clone, Debug, PartialEq)]
pub struct Waypoint {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub dimension: String,
}

/// Where the player is: block position and dimension id.
#[derive(Clone, Debug, PartialEq)]
pub struct Here {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dimension: String,
}

/// One line a waypoint is stored as: tab-separated.
pub fn encode(list: &[Waypoint]) -> String {
    list.iter()
        .map(|w| format!("{}\t{}\t{}\t{}\t{}", w.name, w.x, w.y, w.z, w.dimension))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The waypoints of a stored file; lines that do not read are skipped.
pub fn decode(text: &str) -> Vec<Waypoint> {
    text.lines()
        .filter_map(|line| {
            let mut f = line.split('\t');
            Some(Waypoint {
                name: f.next().filter(|n| name_ok(n))?.to_owned(),
                x: f.next()?.parse().ok()?,
                y: f.next()?.parse().ok()?,
                z: f.next()?.parse().ok()?,
                dimension: f.next()?.to_owned(),
            })
        })
        .collect()
}

/// A name is 1 to 24 letters, digits, `_` or `-`.
pub fn name_ok(name: &str) -> bool {
    (1..=24).contains(&name.len())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Blocks from `here` to the waypoint.
pub fn distance(here: &Here, w: &Waypoint) -> f64 {
    let (dx, dy, dz) = (f64::from(w.x) + 0.5 - here.x, f64::from(w.y) - here.y, f64::from(w.z) + 0.5 - here.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// The yaw that faces the waypoint (0 faces south, 90 west), degrees.
pub fn bearing(here: &Here, w: &Waypoint) -> f32 {
    let dx = f64::from(w.x) + 0.5 - here.x;
    let dz = f64::from(w.z) + 0.5 - here.z;
    (-dx).atan2(dz).to_degrees() as f32
}

/// Where the waypoint is relative to the way the player faces.
pub fn relative(yaw: f32, bearing: f32) -> &'static str {
    let mut d = (bearing - yaw).rem_euclid(360.0);
    if d > 180.0 {
        d -= 360.0;
    }
    let names = ["ahead", "ahead right", "right", "behind right", "behind", "behind left", "left", "ahead left"];
    names[((d / 45.0).round() as i32).rem_euclid(8) as usize]
}

/// `home 128 m ahead right`.
pub fn line(here: &Here, yaw: f32, w: &Waypoint) -> String {
    format!("{} {:.0} m {}", w.name, distance(here, w), relative(yaw, bearing(here, w)))
}

#[derive(Default)]
struct State {
    loaded: bool,
    list: Vec<Waypoint>,
    tracked: Option<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// Reads the stored waypoints once.
pub fn load(get: impl FnOnce() -> Option<String>) {
    STATE.with_borrow_mut(|s| {
        if !s.loaded {
            s.loaded = true;
            s.list = decode(&get().unwrap_or_default());
        }
    });
}

/// What a command did: lines for chat, and the file to store when the list changed.
#[derive(Debug, Default, PartialEq)]
pub struct Outcome {
    pub chat: Vec<String>,
    pub save: Option<String>,
}

/// Runs the arguments after `/wp` (`add home`, `track home`, `list`…).
pub fn run(args: &str, here: Option<&Here>, yaw: f32) -> Outcome {
    let mut words = args.split_whitespace();
    let verb = words.next().unwrap_or("");
    let name = words.next().unwrap_or("");
    let say = |text: String| Outcome { chat: vec![text], save: None };
    STATE.with_borrow_mut(|s| match verb {
        "add" | "set" => {
            let Some(here) = here else {
                return say("Not in a world".into());
            };
            if !name_ok(name) {
                return say("Usage: /wp add <name> (1-24 letters, digits, - or _)".into());
            }
            let place = Waypoint {
                name: name.to_owned(),
                x: here.x.floor() as i32,
                y: here.y.floor() as i32,
                z: here.z.floor() as i32,
                dimension: here.dimension.clone(),
            };
            let said = format!("Waypoint {name} set at {} {} {}", place.x, place.y, place.z);
            s.list.retain(|w| w.name != name);
            s.list.push(place);
            Outcome { chat: vec![said], save: Some(encode(&s.list)) }
        }
        "del" | "remove" => {
            let before = s.list.len();
            s.list.retain(|w| w.name != name);
            if s.list.len() == before {
                return say(format!("No waypoint {name}"));
            }
            if s.tracked.as_deref() == Some(name) {
                s.tracked = None;
            }
            Outcome { chat: vec![format!("Waypoint {name} deleted")], save: Some(encode(&s.list)) }
        }
        "list" | "" => {
            if s.list.is_empty() {
                return say("No waypoints. /wp add <name> sets one here.".into());
            }
            let chat = s
                .list
                .iter()
                .map(|w| {
                    let away = here
                        .filter(|h| h.dimension == w.dimension)
                        .map_or(String::new(), |h| format!(", {} m", distance(h, w).round()));
                    format!("{}: {} {} {} ({}{})", w.name, w.x, w.y, w.z, w.dimension.trim_start_matches("minecraft:"), away)
                })
                .collect();
            Outcome { chat, save: None }
        }
        "track" => {
            if !s.list.iter().any(|w| w.name == name) {
                return say(format!("No waypoint {name}"));
            }
            s.tracked = Some(name.to_owned());
            say(match here.and_then(|h| s.list.iter().find(|w| w.name == name).map(|w| line(h, yaw, w))) {
                Some(l) => format!("Tracking {l}"),
                None => format!("Tracking {name}"),
            })
        }
        "untrack" => {
            s.tracked = None;
            say("Not tracking a waypoint".into())
        }
        _ => say("/wp add|del|list|track|untrack <name>".into()),
    })
}

/// The tracked waypoint's line, when the player is in its dimension.
pub fn tracked_line(here: &Here, yaw: f32) -> Option<String> {
    STATE.with_borrow(|s| {
        let name = s.tracked.as_ref()?;
        let w = s.list.iter().find(|w| &w.name == name)?;
        (w.dimension == here.dimension).then(|| line(here, yaw, w))
    })
}

/// Forgets everything (tests only).
#[cfg(test)]
pub fn forget() {
    STATE.with_borrow_mut(|s| *s = State::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn here(x: f64, y: f64, z: f64) -> Here {
        Here { x, y, z, dimension: "minecraft:overworld".into() }
    }

    #[test]
    fn waypoints_round_trip_through_the_file() {
        let list = vec![
            Waypoint { name: "home".into(), x: 1, y: 64, z: -3, dimension: "minecraft:overworld".into() },
            Waypoint { name: "end-base".into(), x: -10, y: 70, z: 22, dimension: "minecraft:the_end".into() },
        ];
        assert_eq!(decode(&encode(&list)), list);
        assert_eq!(decode("bad line\nhome\t1\t2"), vec![]);
        assert_eq!(decode("a b\t1\t2\t3\tx"), vec![], "spaces are not allowed in names");
    }

    #[test]
    fn names_are_checked() {
        assert!(name_ok("home_2"));
        assert!(!name_ok(""));
        assert!(!name_ok("has space"));
        assert!(!name_ok(&"x".repeat(25)));
    }

    #[test]
    fn bearing_faces_the_target_in_vanilla_yaw() {
        let p = here(0.5, 64.0, 0.5);
        let at = |x, z| Waypoint { name: "w".into(), x, y: 64, z, dimension: "minecraft:overworld".into() };
        assert!((bearing(&p, &at(0, 10)) - 0.0).abs() < 0.01, "south is +Z");
        assert!((bearing(&p, &at(-10, 0)) - 90.0).abs() < 0.01, "west is -X");
        assert!((bearing(&p, &at(10, 0)) + 90.0).abs() < 0.01, "east is +X");
        assert!((bearing(&p, &at(0, -10)).abs() - 180.0).abs() < 0.01, "north is -Z");
        assert_eq!(distance(&p, &at(3, 4)).round(), 5.0);
    }

    #[test]
    fn direction_is_relative_to_where_you_face() {
        assert_eq!(relative(0.0, 0.0), "ahead");
        assert_eq!(relative(0.0, 90.0), "right", "facing south, west is on the right hand");
        assert_eq!(relative(0.0, -90.0), "left");
        assert_eq!(relative(0.0, 180.0), "behind");
        assert_eq!(relative(160.0, -160.0), "ahead right");
        assert_eq!(relative(-160.0, 160.0), "ahead left");
    }

    #[test]
    fn commands_add_list_track_and_delete() {
        forget();
        let p = here(10.7, 64.0, -3.2);
        let added = run("add home", Some(&p), 0.0);
        assert_eq!(added.chat, ["Waypoint home set at 10 64 -4"]);
        assert!(added.save.as_deref().is_some_and(|s| s.starts_with("home\t10\t64\t-4")));
        assert_eq!(run("add bad!", Some(&p), 0.0).save, None);
        assert_eq!(run("add home", None, 0.0).chat, ["Not in a world"]);
        let listed = run("list", Some(&p), 0.0);
        assert!(listed.chat[0].starts_with("home: 10 64 -4 (overworld, "));
        assert_eq!(tracked_line(&p, 0.0), None);
        assert!(run("track home", Some(&p), 0.0).chat[0].starts_with("Tracking home"));
        assert!(tracked_line(&here(0.0, 64.0, 0.0), 0.0).unwrap().starts_with("home "));
        let nether = Here { dimension: "minecraft:the_nether".into(), ..p.clone() };
        assert_eq!(tracked_line(&nether, 0.0), None, "another dimension");
        assert_eq!(run("del home", Some(&p), 0.0).save.as_deref(), Some(""));
        assert_eq!(tracked_line(&p, 0.0), None, "deleted waypoints are not tracked");
        assert_eq!(run("del home", Some(&p), 0.0).chat, ["No waypoint home"]);
        forget();
    }

    #[test]
    fn the_file_is_read_once() {
        forget();
        load(|| Some("home\t1\t2\t3\tminecraft:overworld".into()));
        load(|| Some("other\t9\t9\t9\tminecraft:overworld".into()));
        assert_eq!(run("list", None, 0.0).chat, ["home: 1 2 3 (overworld)"]);
        forget();
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
