"""Combat Hitboxes: wraps Horizon's ray-vs-box test so entity boxes are a little bigger for aiming. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, LIST_B, SET_B, CLK_B

HIT = []

HIT.append(dict(
    slug="combat-hitboxes", id="combat_hitboxes", name="Combat Hitboxes", title="Combat Hitboxes", raw=False, needs_api=407,
    usage="Nothing to press: with the entity hitboxes on (F3+B) the box of the entity you are aiming at turns red, line widths are 2.5 and each part has its own colour. The settings page has every switch and colour; colours can be typed into the files as `#RRGGBB` or `#AARRGGBB`.",
    description="Changes the hitbox colour when the entity is targeted and sets the line widths and the colours of the hitbox parts.",
    upstream="Combat Hitboxes (combat-hitbox) by Tektonikal and sootysplash", upstream_licence="Apache-2.0",
    command=None, hooks=["list", "settings", "click", "tick"],
    options=None, custom_settings=True,
    runtime_extra=r'''
/// The colours the colour buttons walk (the files take any `#RRGGBB` or `#AARRGGBB`): the defaults and a few more.
pub const PALETTE: [&str; 8] = ["#FF0000", "#0000FF", "#FFFFFF", "#FF00FF", "#000000", "#FFFF00", "#00FF00", "#00FFFF"];
/// The widths, distances and multipliers their buttons step through (the files take any number).
pub const WIDTHS: [f32; 8] = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0];
pub const DISTANCES: [f64; 6] = [8.0, 16.0, 32.0, 64.0, 128.0, 256.0];
pub const MULTIPLIERS: [f32; 5] = [1.0, 1.5, 2.0, 3.0, 4.0];

/// `Config`, with its defaults.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub enabled: bool,
    pub hide_arrow: bool,
    pub hide_fireworks: bool,
    pub hide_items: bool,
    pub change_target_color: bool,
    pub eye_color: String,
    pub look_color: String,
    pub hit_box_color: String,
    pub target_box_color: String,
    pub hit_box_hurt: bool,
    pub render_eye_height: bool,
    pub render_look_dir: bool,
    pub line_look_dir: bool,
    pub hurt_box_color: String,
    pub line1: f32,
    pub dist_for_2: f64,
    pub line2: f32,
    pub outline_enabled: bool,
    pub outline_color: String,
    pub outline_multiplier: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            hide_arrow: false,
            hide_fireworks: false,
            hide_items: false,
            change_target_color: true,
            eye_color: "#FF0000".into(),
            look_color: "#0000FF".into(),
            hit_box_color: "#FFFFFF".into(),
            target_box_color: "#FF0000".into(),
            hit_box_hurt: false,
            render_eye_height: true,
            render_look_dir: true,
            line_look_dir: true,
            hurt_box_color: "#FF00FF".into(),
            line1: 2.5,
            dist_for_2: 32.0,
            line2: 2.5,
            outline_enabled: false,
            outline_color: "#000000".into(),
            outline_multiplier: 2.0,
        }
    }
}

/// `#RRGGBB` (opaque) or `#AARRGGBB` to ARGB; white when it does not parse.
pub fn parse_color(text: &str) -> u32 {
    let digits = text.trim().trim_start_matches('#');
    match (digits.len(), u32::from_str_radix(digits, 16)) {
        (6, Ok(v)) => 0xFF00_0000 | v,
        (8, Ok(v)) => v,
        _ => 0xFFFF_FFFF,
    }
}

fn next<T: PartialEq + Copy>(list: &[T], now: T) -> T {
    let at = list.iter().position(|v| *v == now).map_or(0, |p| (p + 1) % list.len());
    list[at]
}

fn next_color(now: &str) -> String {
    let at = PALETTE.iter().position(|p| p.eq_ignore_ascii_case(now.trim())).map_or(0, |p| (p + 1) % PALETTE.len());
    PALETTE[at].to_owned()
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, d: bool| get(key).map_or(d, |v| match v.trim() {
            "true" => true,
            "false" => false,
            _ => d,
        });
        let text = |key: &str, d: &str| get(key).map_or_else(|| d.to_owned(), |v| v.trim().to_owned());
        let num = |key: &str, d: f64| get(key).and_then(|v| v.trim().parse::<f64>().ok()).filter(|v| v.is_finite()).unwrap_or(d);
        Self {
            enabled: flag("enabled", d.enabled),
            hide_arrow: flag("hide_arrow", d.hide_arrow),
            hide_fireworks: flag("hide_fireworks", d.hide_fireworks),
            hide_items: flag("hide_items", d.hide_items),
            change_target_color: flag("change_target_color", d.change_target_color),
            eye_color: text("eye_color", &d.eye_color),
            look_color: text("look_color", &d.look_color),
            hit_box_color: text("hit_box_color", &d.hit_box_color),
            target_box_color: text("target_box_color", &d.target_box_color),
            hit_box_hurt: flag("hit_box_hurt", d.hit_box_hurt),
            render_eye_height: flag("render_eye_height", d.render_eye_height),
            render_look_dir: flag("render_look_dir", d.render_look_dir),
            line_look_dir: flag("line_look_dir", d.line_look_dir),
            hurt_box_color: text("hurt_box_color", &d.hurt_box_color),
            line1: num("line1", f64::from(d.line1)) as f32,
            dist_for_2: num("dist_for_2", d.dist_for_2),
            line2: num("line2", f64::from(d.line2)) as f32,
            outline_enabled: flag("outline_enabled", d.outline_enabled),
            outline_color: text("outline_color", &d.outline_color),
            outline_multiplier: num("outline_multiplier", f64::from(d.outline_multiplier)) as f32,
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Enabled: {}", on(self.enabled)),
            format!("Hide Stuck Arrows: {}", on(self.hide_arrow)),
            format!("Hide Fireworks: {}", on(self.hide_fireworks)),
            format!("Hide Items: {}", on(self.hide_items)),
            format!("Change Target Color: {}", on(self.change_target_color)),
            format!("Eye Color: {}", self.eye_color),
            format!("Look Color: {}", self.look_color),
            format!("Hitbox Color: {}", self.hit_box_color),
            format!("Target Box Color: {}", self.target_box_color),
            format!("Hitbox Hurt: {}", on(self.hit_box_hurt)),
            format!("Render Eye Height: {}", on(self.render_eye_height)),
            format!("Render Look Direction: {}", on(self.render_look_dir)),
            format!("Line Look Direction: {}", on(self.line_look_dir)),
            format!("Hurt Box Color: {}", self.hurt_box_color),
            format!("Line Width: {}", self.line1),
            format!("Distance For Second Width: {}", self.dist_for_2),
            format!("Far Line Width: {}", self.line2),
            format!("Outline: {}", on(self.outline_enabled)),
            format!("Outline Color: {}", self.outline_color),
            format!("Outline Multiplier: {}", self.outline_multiplier),
        ]
    }

    pub fn click(&mut self, i: usize) -> Option<(&'static str, String)> {
        let flip = |v: &mut bool, key: &'static str| {
            *v = !*v;
            Some((key, v.to_string()))
        };
        match i {
            0 => flip(&mut self.enabled, "enabled"),
            1 => flip(&mut self.hide_arrow, "hide_arrow"),
            2 => flip(&mut self.hide_fireworks, "hide_fireworks"),
            3 => flip(&mut self.hide_items, "hide_items"),
            4 => flip(&mut self.change_target_color, "change_target_color"),
            5 => {
                self.eye_color = next_color(&self.eye_color);
                Some(("eye_color", self.eye_color.clone()))
            }
            6 => {
                self.look_color = next_color(&self.look_color);
                Some(("look_color", self.look_color.clone()))
            }
            7 => {
                self.hit_box_color = next_color(&self.hit_box_color);
                Some(("hit_box_color", self.hit_box_color.clone()))
            }
            8 => {
                self.target_box_color = next_color(&self.target_box_color);
                Some(("target_box_color", self.target_box_color.clone()))
            }
            9 => flip(&mut self.hit_box_hurt, "hit_box_hurt"),
            10 => flip(&mut self.render_eye_height, "render_eye_height"),
            11 => flip(&mut self.render_look_dir, "render_look_dir"),
            12 => flip(&mut self.line_look_dir, "line_look_dir"),
            13 => {
                self.hurt_box_color = next_color(&self.hurt_box_color);
                Some(("hurt_box_color", self.hurt_box_color.clone()))
            }
            14 => {
                self.line1 = next(&WIDTHS, self.line1);
                Some(("line1", self.line1.to_string()))
            }
            15 => {
                self.dist_for_2 = next(&DISTANCES, self.dist_for_2);
                Some(("dist_for_2", self.dist_for_2.to_string()))
            }
            16 => {
                self.line2 = next(&WIDTHS, self.line2);
                Some(("line2", self.line2.to_string()))
            }
            17 => flip(&mut self.outline_enabled, "outline_enabled"),
            18 => {
                self.outline_color = next_color(&self.outline_color);
                Some(("outline_color", self.outline_color.clone()))
            }
            19 => {
                self.outline_multiplier = next(&MULTIPLIERS, self.outline_multiplier);
                Some(("outline_multiplier", self.outline_multiplier.to_string()))
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
''',
    tests=r'''
    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_the_originals() {
        let s = Settings::read(none);
        assert!(s.enabled && s.change_target_color && s.render_eye_height && s.render_look_dir && s.line_look_dir);
        assert!(!s.hide_arrow && !s.hide_fireworks && !s.hide_items && !s.hit_box_hurt && !s.outline_enabled);
        assert_eq!((s.eye_color.as_str(), s.look_color.as_str(), s.hit_box_color.as_str()), ("#FF0000", "#0000FF", "#FFFFFF"));
        assert_eq!((s.target_box_color.as_str(), s.hurt_box_color.as_str(), s.outline_color.as_str()), ("#FF0000", "#FF00FF", "#000000"));
        assert_eq!((s.line1, s.dist_for_2, s.line2, s.outline_multiplier), (2.5, 32.0, 2.5, 2.0));
    }

    #[test]
    fn colours_parse_as_opaque_rgb_or_argb_and_fall_back_to_white() {
        assert_eq!(parse_color("#FF00FF"), 0xFFFF_00FF);
        assert_eq!(parse_color("80112233"), 0x8011_2233);
        assert_eq!(parse_color("nope"), 0xFFFF_FFFF);
        assert_eq!(parse_color("#12345"), 0xFFFF_FFFF);
    }

    #[test]
    fn the_buttons_flip_and_walk_in_the_configs_order() {
        let l = labels(none);
        assert_eq!(l.len(), 20);
        assert!(l[0].starts_with("Enabled") && l[5].starts_with("Eye Color") && l[13].starts_with("Hurt Box Color") && l[19].starts_with("Outline Multiplier"));
        assert_eq!(click(none, 0).unwrap(), ("enabled", "false".to_owned()));
        assert_eq!(click(none, 7).unwrap(), ("hit_box_color", "#FF00FF".to_owned()), "white is third in the palette, magenta follows");
    }

    #[test]
    fn widths_distances_and_multipliers_step_and_wrap() {
        assert_eq!(click(none, 14).unwrap(), ("line1", "3".to_owned()));
        let top = |k: &str| (k == "line2").then(|| "5".to_owned());
        assert_eq!(click(top, 16).unwrap().1, "0.5");
        assert_eq!(click(none, 15).unwrap(), ("dist_for_2", "64".to_owned()));
        assert_eq!(click(none, 19).unwrap(), ("outline_multiplier", "3".to_owned()));
        assert!(click(none, 40).is_none());
    }
''',
    helpers={
        "tick": '''// Once a tick: hand the game the hitbox style these settings give (vanilla hitboxes when the mod is off).
fn combat_hitboxes_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_combat_hitboxes as m;
    if !game.native_on(m::ID) {
        game.native_combat_hitboxes_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_combat_hitboxes(horizon_client::gizmos::HitboxStyle {
        enabled: s.enabled,
        hide_arrow: s.hide_arrow,
        hide_fireworks: s.hide_fireworks,
        hide_items: s.hide_items,
        change_target_color: s.change_target_color,
        eye_color: m::parse_color(&s.eye_color),
        look_color: m::parse_color(&s.look_color),
        hit_box_color: m::parse_color(&s.hit_box_color),
        target_box_color: m::parse_color(&s.target_box_color),
        hit_box_hurt: s.hit_box_hurt,
        render_eye_height: s.render_eye_height,
        render_look_dir: s.render_look_dir,
        line_look_dir: s.line_look_dir,
        hurt_box_color: m::parse_color(&s.hurt_box_color),
        line1: s.line1,
        dist_for_2: s.dist_for_2,
        line2: s.line2,
        outline_enabled: s.outline_enabled,
        outline_color: m::parse_color(&s.outline_color),
        outline_multiplier: s.outline_multiplier,
    });
}
''',
    },
))
