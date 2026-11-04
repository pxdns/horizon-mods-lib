"""Better Mount HUD: health, speed and jump of the mount you ride, read from Horizon's own mob state. Pinned to one Horizon commit."""
from mods import BOX_HEAD, HUD_BOX, hud_guard
from mods_raw import DRAW_INK

MOUNT = []

MOUNT.append(dict(
    slug="better-mount-hud", id="better_mount_hud", name="Better Mount HUD", title="Better Mount HUD", raw=True,
    usage="Nothing to press: while you ride, a box in the bottom-left corner shows the mount's health as numbers (red when low), how fast it moves in blocks per second, and for a saddled horse, donkey or similar how high it jumps and how charged your jump is. `/mount` hides or shows it.",
    description="Numbers for the mount you ride: health, speed in blocks per second, jump height and jump charge.",
    upstream="Better Mount HUD (bettermounthud)", upstream_licence="see the upstream project",
    command="mount", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("speed", "Speed", True), ("jump", "Jump height and charge", True)],
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "mount";

/// A mount's health line; red at a quarter or less.
pub fn health_line(health: f64, max: f64) -> (String, u32) {
    let ink = if max > 0.0 && health / max <= 0.25 { 0xFF_5555 } else { INK };
    (format!("Health {:.1} / {:.1}", health.max(0.0), max), ink)
}

/// Blocks per second from the horizontal move of one tick.
pub fn speed(dx: f64, dz: f64) -> f64 {
    dx.hypot(dz) * 20.0
}

pub fn speed_line(dx: f64, dz: f64) -> (String, u32) {
    (format!("Speed {:.1} b/s", speed(dx, dz)), INK)
}

/// How high a horse with this jump strength jumps, in blocks (the polynomial fitted to the game's physics).
pub fn jump_height(strength: f64) -> f64 {
    let s = strength.clamp(0.0, 2.0);
    (-0.181_758_495_2 * s * s * s + 3.689_713_992 * s * s + 2.128_599_134 * s - 0.343_930_367).max(0.0)
}

pub fn jump_line(strength: f64, charge: f32) -> (String, u32) {
    let pct = (charge.clamp(0.0, 1.0) * 100.0).round() as u32;
    (format!("Jump {:.2} blocks, charge {pct}%", jump_height(strength)), INK)
}
''',
    tests=r'''
    #[test]
    fn health_goes_red_at_a_quarter() {
        assert_eq!(health_line(30.0, 30.0), ("Health 30.0 / 30.0".to_owned(), INK));
        assert_eq!(health_line(7.5, 30.0).1, 0xFF_5555);
        assert_eq!(health_line(-1.0, 30.0).0, "Health 0.0 / 30.0");
    }

    #[test]
    fn speed_is_blocks_per_second() {
        assert!((speed(0.3, 0.4) - 10.0).abs() < 1e-9);
        assert_eq!(speed_line(0.0, 0.0).0, "Speed 0.0 b/s");
    }

    #[test]
    fn a_stronger_horse_jumps_higher() {
        assert!(jump_height(0.5) < jump_height(0.8));
        assert!((jump_height(1.0) - 5.29).abs() < 0.05, "a top horse clears about five blocks");
        assert_eq!(jump_height(-3.0), 0.0);
    }

    #[test]
    fn the_jump_line_shows_the_charge() {
        assert!(jump_line(0.7, 0.5).0.ends_with("charge 50%"));
        assert!(jump_line(0.7, 5.0).0.ends_with("charge 100%"));
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the bottom-left corner").replace("@DATA@", hud_guard("Corner::BottomLeft", '''    let Some(id) = game.riding else {
        return;
    };
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id) else {
        return;
    };
    let get = |k: &str| game.native_setting(m::ID, k);
    let mut lines = vec![m::health_line(f64::from(mob.health), f64::from(mob.attrs.max_health))];
    if m::is(get, "speed") {
        lines.push(m::speed_line(mob.position.x - mob.previous.x, mob.position.z - mob.previous.z));
    }
    if m::is(get, "jump") && game.jumpable_mount() {
        lines.push(m::jump_line(f64::from(mob.jump_power()), game.mount.jump_scale));
    }
''') + DRAW_INK.replace("@CORNER@", "Corner::BottomLeft")),
))
