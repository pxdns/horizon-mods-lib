"""Pearl Optimizer: a thrown ender pearl shows at once on a server and is swapped for the server's when that arrives. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

PEARL = []

PEARL.append(dict(
    slug="pearl-optimizer", id="pearl_optimizer", name="Pearl Optimizer", title="Pearl Optimizer", raw=True,
    usage="Nothing to press: on a server, throwing an ender pearl shows a pearl flying at once instead of after the server's reply, and it is swapped for the server's pearl when that arrives (or removed after two seconds if the server never sends one). The server still decides everything; this only changes what you see.",
    description="Shows your thrown ender pearl immediately on a server, before the server's pearl arrives.",
    upstream="Pearl Optimizer (pearloptimizer)", upstream_licence="independent implementation from the mod's public description; no upstream code or assets",
    command=None, hooks=["send", "arrive", "tick", "list", "settings", "click"],
    options=[("fast", "Show my pearl at once", True)],
    modpath="crate::game::remote::lib_pearl_optimizer",
    raw_note=("It puts a HEAD hook on `Game::send` (to see the throw leave), one on `add_projectile` (the server's pearl arriving) and a tick that adds a pearl without an owner to the projectile list, the way Horizon adds a server's pearl, with the throw's speed and direction. "
              "None of these is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. Because the local pearl has no owner it never teleports or hurts you; it only flies. "
              "It was written from the original's public description, not its source, whose licence does not allow reuse. Some servers forbid client-side prediction mods; check the rules of the server you play on."),
    toml_text=toml_text("pearl_optimizer", "1.0.0", "crates/horizon-client/src/game/remote.rs", "lib_pearl_optimizer", [
        dict(fields=[("id", '"send"'), ("kind", '"HEAD"'), ("target", '"horizon_client::game::remote::Game<\'_>::send"'), ("helper", '"send.rs"')],
             bindings=[("game", "&crate::game::Game<'_>", "&*self"), ("packet", "&S", "&packet")]),
        dict(fields=[("id", '"arrive"'), ("kind", '"HEAD"'), ("target", '"horizon_client::game::creatures::Game<\'_>::add_projectile"'), ("helper", '"arrive.rs"')],
             bindings=[("p", "&horizon_client::projectiles::Projectile", "&p")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False)),
    runtime_extra=r'''
use std::cell::RefCell as StateCell;

/// Local pearls get ids from here up, far from any id a server hands out.
pub const BASE_ID: u32 = 0x7F00_0000;
/// A local pearl is removed after this many ticks (two seconds) if the server's has not come.
pub const LIFE_TICKS: u64 = 40;
/// The speed of a thrown pearl (`ThrowableItemProjectile.shootFromRotation` with power 1.5).
pub const SPEED: f64 = 1.5;

/// A throw that left for the server: where the pearl starts, how fast it goes, and which way it faces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Throw {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Default)]
struct State {
    throws: Vec<Throw>,
    /// The server's pearls that arrived and have not been matched to a local one yet.
    arrived: u32,
    /// Local pearls: id and the tick they were added.
    local: Vec<(u32, u64)>,
    next: u32,
}

thread_local! {
    static STATE: StateCell<State> = StateCell::new(State::default());
}

/// The unit vector a view with this yaw and pitch (degrees) looks along.
pub fn look(yaw: f32, pitch: f32) -> [f64; 3] {
    let (y, p) = (f64::from(yaw).to_radians(), f64::from(pitch).to_radians());
    [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()]
}

/// The throw for a player with eyes at `eye`, looking `yaw`/`pitch`, moving at `motion` (its vertical part only when not on the ground).
pub fn throw(eye: [f64; 3], yaw: f32, pitch: f32, motion: [f64; 3], on_ground: bool) -> Throw {
    let dir = look(yaw, pitch);
    Throw {
        position: [eye[0], eye[1] - 0.1, eye[2]],
        velocity: [dir[0] * SPEED + motion[0], dir[1] * SPEED + if on_ground { 0.0 } else { motion[1] }, dir[2] * SPEED + motion[2]],
        yaw,
        pitch,
    }
}

/// A pearl was thrown.
pub fn thrown(t: Throw) {
    STATE.with_borrow_mut(|s| s.throws.push(t));
}

/// The server added a pearl.
pub fn server_pearl() {
    STATE.with_borrow_mut(|s| s.arrived += 1);
}

/// One tick: the throws to add as local pearls with their new ids, and the local pearls to remove (one per server pearl that
/// arrived, and any that outlived `LIFE_TICKS`).
pub fn step(now: u64) -> (Vec<(u32, Throw)>, Vec<u32>) {
    STATE.with_borrow_mut(|s| {
        let mut remove = Vec::new();
        while s.arrived > 0 && !s.local.is_empty() {
            s.arrived -= 1;
            remove.push(s.local.remove(0).0);
        }
        s.arrived = 0;
        let (old, keep): (Vec<_>, Vec<_>) = std::mem::take(&mut s.local).into_iter().partition(|(_, at)| now.saturating_sub(*at) >= LIFE_TICKS);
        s.local = keep;
        remove.extend(old.into_iter().map(|(id, _)| id));
        let mut add = Vec::new();
        for t in std::mem::take(&mut s.throws) {
            s.next += 1;
            let id = BASE_ID + s.next;
            s.local.push((id, now));
            add.push((id, t));
        }
        (add, remove)
    })
}

/// No world or the mod is off: forget everything.
pub fn reset() {
    STATE.with_borrow_mut(|s| *s = State::default());
}
''',
    tests=r'''
    fn near(a: [f64; 3], b: [f64; 3]) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-9)
    }

    #[test]
    fn the_look_vector_follows_yaw_and_pitch() {
        assert!(near(look(0.0, 0.0), [0.0, 0.0, 1.0]), "yaw 0 faces south");
        assert!(near(look(90.0, 0.0), [-1.0, 0.0, 0.0]), "yaw 90 faces west");
        assert!(near(look(0.0, -90.0), [0.0, 1.0, 0.0]), "pitch -90 looks up");
    }

    #[test]
    fn a_throw_starts_below_the_eyes_and_adds_the_players_motion() {
        let t = throw([10.0, 65.62, -3.0], 0.0, 0.0, [0.1, -0.2, 0.05], true);
        assert!(near(t.position, [10.0, 65.52, -3.0]));
        assert!(near(t.velocity, [0.1, 0.0, 1.5 + 0.05]), "no vertical motion on the ground");
        let air = throw([0.0, 0.0, 0.0], 0.0, 0.0, [0.0, -0.4, 0.0], false);
        assert!((air.velocity[1] + 0.4).abs() < 1e-9, "falling adds to the throw");
    }

    #[test]
    fn a_throw_becomes_a_local_pearl_and_the_servers_replaces_it() {
        reset();
        let t = throw([0.0, 1.0, 0.0], 0.0, 0.0, [0.0; 3], true);
        thrown(t);
        let (add, remove) = step(100);
        assert_eq!(add.len(), 1);
        assert!(add[0].0 > BASE_ID && remove.is_empty());
        server_pearl();
        let (add2, remove2) = step(101);
        assert!(add2.is_empty());
        assert_eq!(remove2, vec![add[0].0], "the server's pearl takes the local one's place");
        assert!(step(102).1.is_empty());
    }

    #[test]
    fn a_pearl_the_server_never_sends_is_removed_after_two_seconds() {
        reset();
        thrown(throw([0.0; 3], 0.0, 0.0, [0.0; 3], true));
        let (add, _) = step(100);
        assert!(step(139).1.is_empty());
        assert_eq!(step(140).1, vec![add[0].0]);
    }

    #[test]
    fn a_server_pearl_with_no_local_one_does_nothing() {
        reset();
        server_pearl();
        let (add, remove) = step(5);
        assert!(add.is_empty() && remove.is_empty());
        thrown(throw([0.0; 3], 0.0, 0.0, [0.0; 3], true));
        let (add, remove) = step(6);
        assert_eq!((add.len(), remove.len()), (1, 0), "an old arrival does not cancel a new throw");
    }
''',
    helpers={
        "send": '''// A packet leaves for the server: using an ender pearl starts a local pearl.
fn pearl_optimizer_send(game: &crate::game::Game<'_>, packet: &S) {
    use lib_pearl_optimizer as m;
    let S::UseItem(used) = packet else {
        return;
    };
    let stack = if used.hand == 0 { game.held() } else { game.offhand.as_ref() };
    let Some(stack) = stack.filter(|s| &*s.item == "minecraft:ender_pearl") else {
        return;
    };
    let group = horizon_client::cooldowns::group(stack, game.items().get(&stack.item).and_then(|i| i.use_cooldown.as_ref()));
    if game.screen != Screen::Playing || game.cooldowns.is_on_cooldown(&group) {
        return;
    }
    let (Some(eye), Some(player)) = (game.eye_position(), game.player.as_ref()) else {
        return;
    };
    m::thrown(m::throw(eye, game.yaw, game.pitch, [player.velocity.x, player.velocity.y, player.velocity.z], player.ground.on_ground));
}
''',
        "arrive": '''// A projectile arrives from the server: a pearl takes the place of a local one.
fn pearl_optimizer_add(p: &horizon_client::projectiles::Projectile) {
    if matches!(p.kind, horizon_client::projectiles::ProjectileKind::EnderPearl) && p.id < crate::game::remote::lib_pearl_optimizer::BASE_ID {
        crate::game::remote::lib_pearl_optimizer::server_pearl();
    }
}
''',
        "tick": '''// Once a tick: add the local pearls for the throws, and remove those the server's pearls replaced.
fn pearl_optimizer_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::remote::lib_pearl_optimizer as m;
    use horizon_client::projectiles::{Projectile, ProjectileKind};
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID) && game.remote.is_some() && m::is(|k| game.native_setting(m::ID, k), "fast");
    if !on {
        m::reset();
        game.creatures.projectiles.retain(|p| p.id < m::BASE_ID);
        return;
    }
    let (add, remove) = m::step(game.ticks as u64);
    game.creatures.projectiles.retain(|p| !remove.contains(&p.id));
    for (id, throw) in add {
        let mut pearl = Projectile::new(ProjectileKind::EnderPearl, horizon_core::geometry::Vec3::new(throw.position[0], throw.position[1], throw.position[2]), None);
        pearl.id = id;
        pearl.velocity = horizon_core::geometry::Vec3::new(throw.velocity[0], throw.velocity[1], throw.velocity[2]);
        pearl.yaw = throw.yaw;
        pearl.pitch = throw.pitch;
        game.creatures.projectiles.push(pearl);
    }
}
''',
    },
))
