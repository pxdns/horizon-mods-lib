"""Marlow's Crystal Optimizer: an End crystal you hit disappears at once and comes back if the server does not break it. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

CRYSTAL = []

CRYSTAL.append(dict(
    slug="crystal-optimizer", id="crystal_optimizer", name="Crystal Optimizer", title="Crystal Optimizer", raw=True,
    usage="Nothing to press: on a server, an End crystal you hit vanishes the moment you swing instead of when the server's answer arrives. If the server does not remove it within 1.5 seconds it comes back. The server still decides everything; this only changes what you see.",
    description="Makes End crystals you hit disappear at once on a server, and brings one back if the server did not break it.",
    upstream="Marlow's Crystal Optimizer (marlowcrystal)", upstream_licence="MIT",
    command=None, hooks=["send", "gone", "tick", "list", "settings", "click"],
    options=[("hide", "Hide a hit crystal at once", True)],
    modpath="crate::game::remote::lib_crystal_optimizer",
    toml_text=toml_text("crystal_optimizer", "1.0.0", "crates/horizon-client/src/game/remote.rs", "lib_crystal_optimizer", [
        dict(fields=[("id", '"send"'), ("kind", '"HEAD"'), ("target", '"horizon_client::game::remote::Game<\'_>::send"'), ("helper", '"send.rs"')],
             bindings=[("game", "&crate::game::Game<'_>", "&*self"), ("packet", "&S", "&packet")]),
        dict(fields=[("id", '"gone"'), ("kind", '"HEAD"'), ("target", '"horizon_client::game::remote_projectiles::Game<\'_>::remove_remote_projectile"'), ("helper", '"gone.rs"')],
             bindings=[("id", "u32", "id")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False)),
    raw_note=("It puts a HEAD hook on `Game::send` (to see an attack on a crystal leave), one on `remove_remote_projectile` (the server removing a crystal), and a tick that takes the hit crystal out of `Game::crystals` and puts it back if it is still there after 30 ticks. "
              "None of these is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. It only changes what you see on a server, and only for crystals the client knows are not invulnerable. "
              "The original's prediction of placing crystals is not included. Some servers forbid client-side prediction mods; check the rules of the server you play on."),
    runtime_extra=r'''
use std::any::Any;

/// A hit crystal comes back if the server has not removed it after this many ticks (a second and a half).
pub const RESTORE_TICKS: u64 = 30;

#[derive(Default)]
struct State {
    /// Crystals the player hit that are still in the game's list: take them out at the next tick.
    asked: Vec<u32>,
    /// Crystals taken out, with the tick they went and the crystal itself.
    kept: Vec<(u32, u64, Box<dyn Any>)>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The player attacked crystal `id`.
pub fn hit(id: u32) {
    STATE.with_borrow_mut(|s| {
        if !s.asked.contains(&id) && !s.kept.iter().any(|(k, ..)| *k == id) {
            s.asked.push(id);
        }
    });
}

/// The crystals to take out now.
pub fn take_requests() -> Vec<u32> {
    STATE.with_borrow_mut(|s| std::mem::take(&mut s.asked))
}

/// A crystal was taken out at `tick`; the mod holds it until the server answers.
pub fn keep(id: u32, tick: u64, crystal: Box<dyn Any>) {
    STATE.with_borrow_mut(|s| s.kept.push((id, tick, crystal)));
}

/// The server removed crystal `id` (or never needed it): forget it.
pub fn confirmed(id: u32) {
    STATE.with_borrow_mut(|s| {
        s.asked.retain(|k| *k != id);
        s.kept.retain(|(k, ..)| *k != id);
    });
}

/// The crystals the server did not remove in time, to put back.
pub fn expired(now: u64) -> Vec<Box<dyn Any>> {
    STATE.with_borrow_mut(|s| {
        let (old, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut s.kept).into_iter().partition(|(_, at, _)| now.saturating_sub(*at) >= RESTORE_TICKS);
        s.kept = rest;
        old.into_iter().map(|(.., c)| c).collect()
    })
}

/// No world: forget everything.
pub fn reset() {
    STATE.with_borrow_mut(|s| *s = State::default());
}

/// How many are being held (tests).
#[cfg(test)]
pub fn held() -> usize {
    STATE.with_borrow(|s| s.kept.len())
}
''',
    tests=r'''
    #[test]
    fn a_hit_is_asked_once_and_taken_once() {
        reset();
        hit(5);
        hit(5);
        assert_eq!(take_requests(), vec![5]);
        assert!(take_requests().is_empty());
    }

    #[test]
    fn a_held_crystal_is_not_asked_again() {
        reset();
        keep(7, 100, Box::new("crystal"));
        hit(7);
        assert!(take_requests().is_empty());
        assert_eq!(held(), 1);
        reset();
    }

    #[test]
    fn the_server_removing_it_forgets_it() {
        reset();
        keep(7, 100, Box::new(1));
        confirmed(7);
        assert_eq!(held(), 0);
        assert!(expired(1000).is_empty(), "nothing comes back after the server removed it");
        hit(8);
        confirmed(8);
        assert!(take_requests().is_empty());
    }

    #[test]
    fn an_unanswered_crystal_comes_back_after_a_second_and_a_half() {
        reset();
        keep(7, 100, Box::new(String::from("seven")));
        keep(8, 120, Box::new(String::from("eight")));
        assert!(expired(129).is_empty());
        let back = expired(130);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].downcast_ref::<String>().map(String::as_str), Some("seven"));
        assert_eq!(held(), 1, "the later one is still waiting");
        assert_eq!(expired(150).len(), 1);
        reset();
    }
''',
    helpers={
        "send": '''// A packet leaves for the server: an attack on one of the End crystals it told us about starts the hiding.
fn crystal_optimizer_send(game: &crate::game::Game<'_>, packet: &S) {
    use lib_crystal_optimizer as m;
    if let S::Attack(attack) = packet
        && let Ok(id) = u32::try_from(attack.id)
        && game.crystals.iter().any(|c| c.id == id && !c.invulnerable)
    {
        m::hit(id);
    }
}
''',
        "gone": '''// The server removed an entity it had told us about.
fn crystal_optimizer_gone(id: u32) {
    crate::game::remote::lib_crystal_optimizer::confirmed(id);
}
''',
        "tick": '''// Once a tick: take the crystals the player hit out of the list, and put back any the server did not remove.
fn crystal_optimizer_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::remote::lib_crystal_optimizer as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID) && game.remote.is_some() && m::is(|k| game.native_setting(m::ID, k), "hide");
    if !on {
        m::reset();
        return;
    }
    let now = game.ticks as u64;
    for id in m::take_requests() {
        if let Some(at) = game.crystals.iter().position(|c| c.id == id) {
            let crystal = game.crystals.remove(at);
            m::keep(id, now, Box::new(crystal));
        }
    }
    for back in m::expired(now) {
        if let Ok(crystal) = back.downcast::<crate::game::dragon::Crystal>() {
            game.crystals.push(*crystal);
        }
    }
}
''',
    },
))
