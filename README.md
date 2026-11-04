<p align="center"><img src="assets/icons/horizon-mods-lib.svg" width="72" alt="horizon-mods-lib"></p>

# horizon-mods-lib

Native mods for [Horizon](https://github.com/horizon-rust/horizon) (the Rust rewrite of Minecraft: Java 26.3), **ported from
popular Fabric mods**, one small tested project per mod, each released automatically as a `.hnmod`.

**Docs:** [docs/](docs/introduction.md) · [Catalogue of 186 mods](MODS.md) · [Modules](MODULES.md)

## Download everything

One zip with **all 27 mods**, unpacked, plus an install script: [`horizon-mods-all.zip`](https://github.com/pxdns/horizon-mods-lib/releases/download/all-mods/horizon-mods-all.zip) (rebuilt on every change). Unzip it and run `./install.sh MyInstance`
(or `install.ps1` on Windows); [the install guide](docs/install.md) has the details and the two pairs that overlap.

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
| [`armor-hud`](mods/armor-hud) | Worn armor and durability left, armor value on the bar (reads game state) | uku's Armor HUD, Detail Armor Bar |
| [`potion-counter`](mods/potion-counter) | Potions, splash and lingering carried (reads game state) | PotionCounter |
| [`totem-counter`](mods/totem-counter) | Totems carried, red at zero (reads game state) | TotemCounter |
| [`effect-timer`](mods/effect-timer) | Active effects with time left and duration bars under the icons (reads game state) | Status Effect Timer |
| [`apple-skin`](mods/apple-skin) | Hunger, saturation, held-food preview, drawn on the food bar (reads game state) | AppleSkin |
| [`pick-up-notifier`](mods/pick-up-notifier) | +3 Cobblestone on pickup (reads game state) | Pick Up Notifier |
| [`freelook`](mods/freelook) | Hold Left Alt and look around without turning (hooks mouse look and the camera) | Freelook |
| [`freecam`](mods/freecam) | Detach the camera and fly it (hooks mouse look, the camera and held keys) | Freecam |
| [`snappy-tappy`](mods/snappy-tappy) | The last of two opposite keys pressed wins | SnappyTappy |
| [`better-hurt-cam`](mods/better-hurt-cam) | Weaker hit shake, no death tilt | BetterHurtCam |
| [`more-chat-history`](mods/more-chat-history) | Chat keeps 100 to 2000 lines (hooks Horizon's chat) | More Chat History |
| [`chat-notify`](mods/chat-notify) | A sound when a message says your name (hooks Horizon's chat) | Chat Notify |
| [`chat-animation`](mods/chat-animation) | New chat lines slide up into place (rebuilds the chat layout) | Chat Animation |
| [`durability-tooltip`](mods/durability-tooltip) | A Durability line on tools, weapons and armor tooltips | Durability Tooltip |
| [`crosshair-indicator`](mods/crosshair-indicator) | Red mark over the crosshair when it is over a living entity | Crosshair Indicator |
| [`entity-crosshair`](mods/entity-crosshair) | Crosshair style and colour of your choice, red over living entities | Entity Crosshair |
| [`shulker-box-tooltip`](mods/shulker-box-tooltip) | What is inside a shulker box: totals and a 9 by 3 grid of the items | Shulker Box Tooltip |
| [`combat-hitboxes`](mods/combat-hitboxes) | Entities a little easier to hit (grows the aiming boxes) | Combat Hitboxes |
| [`ping-view`](mods/ping-view) | Your connection's ping, with average and range (hooks Horizon's ping logger) | Ping View |
| [`invmove`](mods/invmove) | Keep walking with the inventory open (re-runs the tick with no screen) | InvMove |
| [`better-mount-hud`](mods/better-mount-hud) | Health, speed and jump of the mount you ride (reads game state) | Better Mount HUD |
| [`password-hider`](mods/password-hider) | Masks the password in `/login`, `/register` and the like, and keeps it out of the history file (hooks Horizon's chat) | Written for this library |
| [`enchantment-descriptions`](mods/enchantment-descriptions) | A line saying what each enchantment on an item does, under its tooltip (reads game state) | Enchantment Descriptions |
| [`f5-split`](mods/f5-split) | Separate keys for the back and front third-person view, toggle or hold (sets the game's camera) | F5 Split |
| [`minimap-plus`](mods/minimap-plus) | Corner, size, zoom keys and a big map for Horizon's minimap | Xaero's Minimap and World Map (the map display) |

Each folder is a complete project (`runtime.rs` with unit tests, a `horizon-mixin.toml`, pins, licence, README) and has its own
workflow: bump its `version` on `main` and `release-<project>.yml` tests, packages and publishes `<project>-v<version>` with the `.hnmod`
and its SHA-256. [`ci.yml`](.github/workflows/ci.yml) applies every mod on the pinned Horizon and type-checks the game.

## Install, build, write one

- The site is committed in `public/` (Vercel serves it as is). Rebuild: `python3 -m venv .venv && .venv/bin/pip install markdown && .venv/bin/python site/build.py`.
- [Install a mod](docs/install.md) · [Build from source](docs/build.md) · [Hook points](docs/hook-points.md) · [Anatomy of a mod](docs/anatomy.md)
- `python3 tools/mods.py check all` applies every mod on Horizon and `cargo check`s the game; `package <slug>` makes the `.hnmod`.

## Status, honestly

All 34 mods are unit-tested, formatted, clippy-clean and apply on Horizon `ba6344a` with horizon-mixins `d9c83d7`, where the modded
game type-checks with warnings denied. CI cannot play the game through a window, so none has been exercised in a live session by CI.
Of the 186 catalogued mods, 31 are ported, 5 partly, 8 are built into Horizon already and 15 are waiting on hook points Horizon does not have yet (each catalogue page says why; the proposals for [screen, slot and map hooks](proposals/native-screen-hooks.md) and [transport and packet hooks](proposals/native-network-hooks.md) say what would unlock them). See [the hook-point notes](docs/hook-points.md). 24 of the 34 read or hook Horizon's own internals and are pinned to one Horizon commit.

## Credits, licensing, takedowns

[Credits](docs/credits.md) · [Licensing](docs/licensing.md) · [DMCA and takedowns](docs/dmca.md) · [Trademarks and disclaimer](docs/trademarks.md) ·
[Security](SECURITY.md) · [Contributing](CONTRIBUTING.md)

MIT licensed. An unofficial community project: not made or endorsed by the Horizon project, Mojang or Microsoft. Minecraft is a
trademark of Mojang Synergies AB.

## Regenerating

The mods, docs, catalogue and release workflows are generated from [`tools/gen/`](tools/gen/README.md). `python3 tools/gen/regen.py` rebuilds them from this repository alone (it downloads the pinned Horizon source itself); a second run changes nothing.
