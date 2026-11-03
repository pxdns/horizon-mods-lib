# Credits

## Horizon

- **Horizon** is made by Ludovic Marie and the Horizon contributors, MIT:
  [horizon-rust/horizon](https://github.com/horizon-rust/horizon).
- **horizon-mixins**, the tool that turns these bundles into a game, is by levzzz5154, MIT:
  [horizon-rust/horizon-mixins](https://github.com/horizon-rust/horizon-mixins).
- The structure of every project follows
  [horizon-native-mod-template](https://github.com/horizon-rust/horizon-native-mod-template), itself based on levzzz5154's
  Veinminer port, MIT.
- `tools/mods.py` is adapted from `native/tools/native.py` in [horizon-rust/horizon-mods](https://github.com/horizon-rust/horizon-mods), MIT.
- The site's look follows the Horizon website, [horizon-rust/horizon-site](https://github.com/horizon-rust/horizon-site), MIT: the
  same dark violet palette, card shapes and spacing, written again as plain CSS. No file from that site is copied.

## The mods these are ported from

Each project re-implements the behaviour of an existing mod. No code, art or text is copied from them, and they are not endorsed
by their authors. The authors are credited here because the idea, the name and the design are theirs.

| Project | Ported from | Upstream licence |
|---|---|---|
| [`fps-display`](mods/fps-display.md) | FPS - Display (fpsdisplay) | see the upstream project |
| [`sprint-indicator`](mods/sprint-indicator.md) | Sprint Indicator (sprintindicator) | see the upstream project |
| [`key-overlay`](mods/key-overlay.md) | Key Overlay (KeyOverlay) | see the upstream project |
| [`cps-counter`](mods/cps-counter.md) | Tempest CPS module (own work) | MIT |
| [`info-hud`](mods/info-hud.md) | MiniHUD (info lines) | see the upstream project |
| [`custom-hud`](mods/custom-hud.md) | CustomHUD (profile idea) | MIT |
| [`zoomify`](mods/zoomify.md) | Zoomify (zoomify) | see the upstream project |
| [`simple-zoom`](mods/simple-zoom.md) | Simple Zoom (simplezoom) | see the upstream project |
| [`waypoints`](mods/waypoints.md) | Xaero's Minimap and World Map (waypoints idea) | independent implementation, no upstream code |
| [`armor-hud`](mods/armor-hud.md) | uku's Armor HUD and Detail Armor Bar Reconstructed | see the upstream projects |
| [`potion-counter`](mods/potion-counter.md) | PotionCounter (potioncounter) | see the upstream project |
| [`totem-counter`](mods/totem-counter.md) | TotemCounter (totemcounter) | see the upstream project |
| [`effect-timer`](mods/effect-timer.md) | Status Effect Timer and Status Effect Bars | see the upstream projects |
| [`apple-skin`](mods/apple-skin.md) | AppleSkin (appleskin) | The Unlicense |
| [`pick-up-notifier`](mods/pick-up-notifier.md) | Pick Up Notifier (pickupnotifier) | see the upstream project |
| [`freelook`](mods/freelook.md) | Freelook (freelook) | see the upstream project |
| [`freecam`](mods/freecam.md) | Freecam (freecam) | see the upstream project |
| [`snappy-tappy`](mods/snappy-tappy.md) | SnappyTappy (SnappyTappy) | see the upstream project |
| [`better-hurt-cam`](mods/better-hurt-cam.md) | BetterHurtCam (betterhurtcam), and the death-tilt part of No Death Animation | see the upstream projects |

The complete list of upstream projects with their authors' licences, as shown on Modrinth, is in the
[catalogue](../MODS.md). A Modrinth page is the place to find who made a mod and to support them.

## Tempest

The ports and the module list come out of the Tempest client project, [pxdns/patcher](https://github.com/pxdns/patcher): its
cross-pack matrix, its licence notes (`docs/MOD-LICENSES.md`) and its 1.16.5 port status.

## Icons

- **The nine projects' own icons** (`assets/icons/<project>.svg`) are original artwork made for this library, MIT.
- **The catalogue's icons** (`assets/icons/catalogue/`) are each upstream mod's **own project icon, downloaded from Modrinth**, shown only to identify the mod
  it belongs to. They are the work of those mods' authors (named on every catalogue page, from Modrinth's team listing) and are **not** covered by this repository's
  MIT licence: they stay under whatever terms the authors gave them. They are shrunk to 96 px and otherwise unaltered. Three projects have no icon on Modrinth and
  show a placeholder tile instead.
- If you made one of these icons and want it gone, see [DMCA and takedowns](dmca.md); it is removed without argument.

## Fonts

None are bundled. The site uses the visitor's own system fonts, so it loads no third-party font, script, analytics or tracker.
