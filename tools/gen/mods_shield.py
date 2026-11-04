"""Shield Statuses: your shield's state, ready, blocking or disabled with the seconds left, from Horizon's own shield state. Pinned to one Horizon commit."""
from mods import BOX_HEAD, HUD_BOX, hud_guard
from mods_raw import DRAW_INK

SHIELD = []

SHIELD.append(dict(
    slug="shield-statuses", id="shield_statuses", name="Shield Statuses", title="Shield Statuses", raw=True,
    usage="Nothing to press: while you carry a shield a box in the top-right corner says `Shield: ready` (green), `Shield: blocking` (aqua) or `Shield: disabled 3.5 s` (red) after an axe disables it. `/shield` hides or shows it.",
    description="Your shield's state as a coloured line: ready, blocking, or disabled with the seconds left.",
    upstream="Shield Statuses (shieldstatus)", upstream_licence="see the upstream project",
    command="shield", hooks=["hud", "command", "list", "settings", "click"],
    options=[("shown", "Shown", True), ("hide", "Hide when ready", False)],
    raw_note=("It reads Horizon's own shield state (`Game::shield_cooldown`, the item in use and `Game::blocks_attacks`), which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. "
              "It shows your own shield only: the original colours other players' shields, which needs their state and an entity drawing hook."),
    runtime_extra=BOX_HEAD + r'''
/// Slash command that hides or shows the box.
pub const COMMAND: &str = "shield";

/// A shield's state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    Blocking,
    /// Disabled by an axe: ticks left.
    Disabled(u32),
}

/// The state: a cooldown wins over blocking, which wins over ready.
pub fn status(blocking: bool, cooldown_ticks: u32) -> Status {
    if cooldown_ticks > 0 {
        Status::Disabled(cooldown_ticks)
    } else if blocking {
        Status::Blocking
    } else {
        Status::Ready
    }
}

/// The line and its colour; none when `hide_ready` and the shield is ready.
pub fn line(status: Status, hide_ready: bool) -> Option<(String, u32)> {
    match status {
        Status::Ready if hide_ready => None,
        Status::Ready => Some(("Shield: ready".to_owned(), 0x55_FF55)),
        Status::Blocking => Some(("Shield: blocking".to_owned(), 0x55_FFFF)),
        Status::Disabled(ticks) => Some((format!("Shield: disabled {:.1} s", f64::from(ticks) / 20.0), 0xFF_5555)),
    }
}
''',
    tests=r'''
    #[test]
    fn the_state_follows_cooldown_then_blocking() {
        assert_eq!(status(false, 0), Status::Ready);
        assert_eq!(status(true, 0), Status::Blocking);
        assert_eq!(status(false, 70), Status::Disabled(70));
        assert_eq!(status(true, 70), Status::Disabled(70), "a disabled shield is not blocking");
    }

    #[test]
    fn lines_and_colours() {
        assert_eq!(line(Status::Ready, false), Some(("Shield: ready".into(), 0x55_FF55)));
        assert_eq!(line(Status::Ready, true), None);
        assert_eq!(line(Status::Blocking, true), Some(("Shield: blocking".into(), 0x55_FFFF)));
        assert_eq!(line(Status::Disabled(70), false), Some(("Shield: disabled 3.5 s".into(), 0xFF_5555)));
        assert_eq!(line(Status::Disabled(1), false).unwrap().0, "Shield: disabled 0.1 s");
    }
''',
    hud=lambda: HUD_BOX.replace("@CORNERDOC@", "the top-right corner").replace("@DATA@", hud_guard("Corner::TopRight", '''    let get = |k: &str| game.native_setting(m::ID, k);
    let hide = m::is(get, "hide");
    let shield = |stack: Option<&horizon_client::items::ItemStack>| stack.is_some_and(|s| game.blocks_attacks(s).is_some());
    let carrying = shield(game.stack_in(game.selected)) || shield(game.offhand.as_ref());
    if !carrying {
        return;
    }
    let blocking = game.using.is_some_and(|(slot, _)| shield(game.stack_in(slot)));
    let lines: Vec<(String, u32)> = m::line(m::status(blocking, game.shield_cooldown), hide).into_iter().collect();
''') + DRAW_INK.replace("@CORNER@", "Corner::TopRight")),
))
