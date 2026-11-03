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

Each folder is a complete project (`runtime.rs` with unit tests, a `horizon-mixin.toml`, pins, licence, README) and has its own
workflow: bump its `version` on `main` and `release-<project>.yml` tests, packages and publishes `<project>-v<version>` with the `.hnmod`
and its SHA-256. [`ci.yml`](.github/workflows/ci.yml) applies every mod on the pinned Horizon and type-checks the game.

## Install, build, write one

- [Install a mod](docs/install.md) · [Build from source](docs/build.md) · [Hook points](docs/hook-points.md) · [Anatomy of a mod](docs/anatomy.md)
- `python3 tools/mods.py check all` applies every mod on Horizon and `cargo check`s the game; `package <slug>` makes the `.hnmod`.

## Status, honestly

All nine mods are unit-tested, formatted, clippy-clean and apply on Horizon `ba6344a` with horizon-mixins `d9c83d7`, where the modded
game type-checks with warnings denied. CI cannot play the game through a window, so none has been exercised in a live session by CI.
44 of the 186 catalogued mods are waiting on hook points Horizon does not have yet; see the [roadmap](docs/hook-points.md).

## Credits, licensing, takedowns

[Credits](docs/credits.md) · [Licensing](docs/licensing.md) · [DMCA and takedowns](docs/dmca.md) · [Trademarks and disclaimer](docs/trademarks.md) ·
[Security](SECURITY.md) · [Contributing](CONTRIBUTING.md)

MIT licensed. An unofficial community project: not made or endorsed by the Horizon project, Mojang or Microsoft. Minecraft is a
trademark of Mojang Synergies AB.
