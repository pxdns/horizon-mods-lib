<p align="center"><img src="assets/icons/horizon-mods-lib.svg" width="72" alt="horizon-mods-lib"></p>

# horizon-mods-lib

Native mods for [Horizon](https://github.com/horizon-rust/horizon) (the Rust rewrite of Minecraft: Java 26.3), **ported from
popular Fabric mods**, one small tested project per mod, each released automatically as a `.hnmod`.

**Docs:** [docs/](docs/introduction.md) · [Catalogue of 186 mods](MODS.md) · [Modules](MODULES.md)

## The mods

| Project | What it does | Ported from |
|---|---|---|
| [`fps-display`](mods/fps-display) | FPS in a corner, coloured by smoothness | FPS - Display |
| [`sprint-indicator`](mods/sprint-indicator) | Sprinting, Walking or Sneaking on screen | Sprint Indicator |
| [`key-overlay`](mods/key-overlay) | W A S D, jump and mouse buttons, lit while held | Key Overlay |
| [`cps-counter`](mods/cps-counter) | Clicks per second | Tempest CPS module |
| [`info-hud`](mods/info-hud) | Speed, facing, dimension | MiniHUD (info lines) |
| [`custom-hud`](mods/custom-hud) | One box with the lines you pick | CustomHUD |
| [`zoomify`](mods/zoomify) | Hold to zoom, wheel to zoom further, smooth | Zoomify |
| [`simple-zoom`](mods/simple-zoom) | Press to zoom, press again to leave | Simple Zoom |
| [`waypoints`](mods/waypoints) | `/wp` named places and a tracked readout | Xaero's Minimap and World Map (waypoints idea) |
| [`armor-hud`](mods/armor-hud) | Worn armor and durability left (reads game state) | uku's Armor HUD, Detail Armor Bar |
| [`potion-counter`](mods/potion-counter) | Potions, splash and lingering carried (reads game state) | PotionCounter |
| [`totem-counter`](mods/totem-counter) | Totems carried, red at zero (reads game state) | TotemCounter |
| [`effect-timer`](mods/effect-timer) | Active effects with time left (reads game state) | Status Effect Timer |
| [`apple-skin`](mods/apple-skin) | Hunger, saturation, held-food preview as text (reads game state) | AppleSkin |
| [`pick-up-notifier`](mods/pick-up-notifier) | +3 Cobblestone on pickup (reads game state) | Pick Up Notifier |
| [`freelook`](mods/freelook) | Hold Left Alt and look around without turning (hooks mouse look and the camera) | Freelook |
| [`freecam`](mods/freecam) | Detach the camera and fly it (hooks mouse look, the camera and held keys) | Freecam |
| [`snappy-tappy`](mods/snappy-tappy) | The last of two opposite keys pressed wins | SnappyTappy |
| [`better-hurt-cam`](mods/better-hurt-cam) | Weaker hit shake, no death tilt | BetterHurtCam |

Each folder is a complete project (`runtime.rs` with unit tests, a `horizon-mixin.toml`, pins, licence, README) and has its own
workflow: bump its `version` on `main` and `release-<project>.yml` tests, packages and publishes `<project>-v<version>` with the `.hnmod`
and its SHA-256. [`ci.yml`](.github/workflows/ci.yml) applies every mod on the pinned Horizon and type-checks the game.

## Install, build, write one

- The site is committed in `public/` (Vercel serves it as is). Rebuild: `python3 -m venv .venv && .venv/bin/pip install markdown && .venv/bin/python site/build.py`.
- [Install a mod](docs/install.md) · [Build from source](docs/build.md) · [Hook points](docs/hook-points.md) · [Anatomy of a mod](docs/anatomy.md)
- `python3 tools/mods.py check all` applies every mod on Horizon and `cargo check`s the game; `package <slug>` makes the `.hnmod`.

## Status, honestly

All 19 mods are unit-tested, formatted, clippy-clean and apply on Horizon `ba6344a` with horizon-mixins `d9c83d7`, where the modded
game type-checks with warnings denied. CI cannot play the game through a window, so none has been exercised in a live session by CI.
30 of the 186 catalogued mods are still waiting on hook points Horizon does not have yet; 3 more are built into Horizon already. See [the hook-point notes](docs/hook-points.md). Ten of the 19 read or hook Horizon's own internals and are pinned to one Horizon commit.

## Credits, licensing, takedowns

[Credits](docs/credits.md) · [Licensing](docs/licensing.md) · [DMCA and takedowns](docs/dmca.md) · [Trademarks and disclaimer](docs/trademarks.md) ·
[Security](SECURITY.md) · [Contributing](CONTRIBUTING.md)

MIT licensed. An unofficial community project: not made or endorsed by the Horizon project, Mojang or Microsoft. Minecraft is a
trademark of Mojang Synergies AB.
