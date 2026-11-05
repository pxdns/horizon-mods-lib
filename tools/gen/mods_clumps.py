"""Clumps: experience orbs of any size merge into one orb worth all their experience, picked up at once. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

CLUMPS = []

CLUMPS.append(dict(
    slug="clumps", id="clumps", name="Clumps", title="Clumps", raw=True,
    usage="Nothing to press: experience orbs that lie close together merge into one orb worth all of them, whatever their sizes, and you collect it the moment you touch it. The first button switches merging of different sizes off.",
    description="Groups nearby experience orbs into a single orb, which cuts the entity count and is collected at once.",
    upstream="Clumps (clumps)", upstream_licence="MIT",
    command=None, hooks=["merge", "list", "settings", "click", "tick"],
    options=[("sizes", "Merge orbs of different sizes", True)],
    modpath="horizon_client::entities::lib_clumps",
    raw_note=("It replaces `Game::merge_orbs` (in `game.rs`; it was `entities::merge_orbs` before Horizon moved it), the experience-orb merge Horizon runs in single player and as a LAN host, with a cancellable HEAD hook that merges every orb touching another (not only equal sizes at matching ids) "
              "into one orb whose value is the sum of theirs, so saving and loading keep the total. It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. "
              "Experience on a multiplayer server is handled by the server, which this does not change. Unlike the original it does not remember the sizes it merged, so Mending repairs once from the total."),
    toml_text=toml_text("clumps", "1.0.0", "crates/horizon-client/src/entities.rs", "lib_clumps", [
        dict(fields=[("id", '"merge"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client::game::Game<\'_>::merge_orbs"'), ("helper", '"merge.rs"')],
             bindings=[("game", "&mut crate::game::Game<'_>", "&mut *self")]),
    ] + common_native(), visibility="pub"),
    runtime_extra=r'''
use std::cell::Cell;

/// An orb is looked at on this many ticks out of that many (once a quarter second).
pub const EVERY: u32 = 5;

thread_local! {
    static SIZES: Cell<bool> = const { Cell::new(true) };
}

/// Sets whether different sizes merge, from the player's settings (the game tells the mod each tick); off when the mod is.
pub fn configure(sizes: bool) {
    SIZES.with(|v| v.set(sizes));
}

pub fn on() -> bool {
    SIZES.with(Cell::get)
}

/// Which orbs absorb which: `ticks[i]` is orb i's age in ticks, `touches(i, j)` whether their boxes (grown a little) overlap.
/// Returns (absorber, absorbed orbs); an orb absorbed once is not looked at again, and an absorber has been looked at on its turn.
pub fn plan(ticks: &[u32], touches: impl Fn(usize, usize) -> bool) -> Vec<(usize, Vec<usize>)> {
    let mut gone = vec![false; ticks.len()];
    let mut out = Vec::new();
    for i in 0..ticks.len() {
        if gone[i] || ticks[i] % EVERY != 1 {
            continue;
        }
        let mut taken = Vec::new();
        for (j, absorbed) in gone.iter_mut().enumerate() {
            if j != i && !*absorbed && touches(i, j) {
                *absorbed = true;
                taken.push(j);
            }
        }
        if !taken.is_empty() {
            out.push((i, taken));
        }
    }
    out
}

/// The experience of an orb that stands for `count` pickups of `value`.
pub fn worth(value: u32, count: u32) -> u32 {
    value.saturating_mul(count.max(1))
}
''',
    tests=r'''
    #[test]
    fn orbs_are_looked_at_on_their_turn_only() {
        // Orb 0 is on its turn (tick 1), orb 1 is not (tick 2).
        let touching = |_: usize, _: usize| true;
        assert_eq!(plan(&[1, 2], touching), vec![(0, vec![1])]);
        assert!(plan(&[2, 3], touching).is_empty());
    }

    #[test]
    fn an_absorbed_orb_is_not_an_absorber() {
        // All three are on their turn and touch each other: 0 takes both.
        assert_eq!(plan(&[1, 6, 11], |_, _| true), vec![(0, vec![1, 2])]);
    }

    #[test]
    fn only_touching_orbs_merge() {
        let near = |i: usize, j: usize| (i, j) == (0, 2) || (i, j) == (2, 0);
        assert_eq!(plan(&[1, 1, 1], near), vec![(0, vec![2])]);
        assert!(plan(&[1, 1], |_, _| false).is_empty());
    }

    #[test]
    fn worth_multiplies_and_never_overflows() {
        assert_eq!(worth(7, 3), 21);
        assert_eq!(worth(7, 0), 7);
        assert_eq!(worth(u32::MAX, 2), u32::MAX);
    }

    #[test]
    fn the_switch_works() {
        configure(false);
        assert!(!on());
        configure(true);
        assert!(on());
    }
''',
    helpers={
        "merge": '''// ExperienceOrb.scanForMerges, Clumps' way: any orbs that touch become one orb worth all of them.
fn clumps_merge(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<()> {
    use horizon_client::entities::lib_clumps as m;
    let orbs = &mut game.orbs;
    if !m::on() {
        return std::ops::ControlFlow::Continue(());
    }
    let ticks: Vec<u32> = orbs.iter().map(|o| o.ticks).collect();
    let areas: Vec<_> = orbs.iter().map(|o| o.bounds().inflate(Vec3::new(0.5, 0.5, 0.5))).collect();
    let boxes: Vec<_> = orbs.iter().map(ExperienceOrb::bounds).collect();
    let plan = m::plan(&ticks, |i, j| boxes[j].intersects(areas[i].clone()));
    let mut gone = vec![false; orbs.len()];
    for (i, taken) in plan {
        let mut total = m::worth(orbs[i].value, orbs[i].count);
        let mut age = orbs[i].age;
        for j in taken {
            total = total.saturating_add(m::worth(orbs[j].value, orbs[j].count));
            age = age.min(orbs[j].age);
            gone[j] = true;
        }
        orbs[i].value = total;
        orbs[i].count = 1;
        orbs[i].age = age;
    }
    let mut gone = gone.into_iter();
    orbs.retain(|_| !gone.next().unwrap_or(false));
    std::ops::ControlFlow::Break(())
}
''',
        "tick": '''// Once a tick: tell the orb merge what the player switched on.
fn clumps_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::entities::lib_clumps as m;
    if !game.native_window() {
        return;
    }
    m::configure(game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "sizes"));
}
''',
    },
))
