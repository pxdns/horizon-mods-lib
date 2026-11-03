# Mod catalogue

Every mod in the Tempest project's cross-pack matrix ([`docs/MOD-MATRIX.md`](https://github.com/pxdns/patcher/blob/main/docs/MOD-MATRIX.md)
in `pxdns/patcher`): **186 mods**, **5** ported · **2** partly ported · **44** needs a hook point · **31** pack or asset · **31** engine or server · **28** library · **45** no 26.3 build.

Horizon implements Minecraft **26.3**, so the 26.3 column decides what can be ported at all. A native Horizon mod can
only use what Horizon's [native hook points](docs/hook-points.md) expose (frame, tick, HUD, key, wheel, command, zoom,
minimap, mods list, settings and the world lifecycle, plus the item panel and recipes), which is why most Fabric mods that
rewrite rendering, networking or inventories wait for more hook points. The status below says which case each mod is in;
nothing is marked ported unless a project in [`mods/`](mods/) builds and passes its tests.

## Status

| Status | Meaning |
|---|---|
| Ported | A project in `mods/` implements it for Horizon, independently of the upstream code |
| Partly ported | Only the part the hook points allow is in a project |
| Needs a hook point | Wanted, but Horizon does not expose what it needs yet |
| Pack or asset | Belongs in the registry as a resource, shader or data pack |
| Engine or server | Performance or server work that Horizon's own engine replaces |
| Library | A helper other Fabric mods load; native mods do not use it |
| No 26.3 build | The upstream mod has no build for Horizon's game version |

Upstream licences are copied from each project's Modrinth page. They are listed for credit and to show that this repository
copies no upstream code or art: every port is a clean re-implementation of the behaviour.

## Ported (5)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| fpsdisplay-5.1.0+26.2 | [fpsdisplay](https://modrinth.com/mod/fpsdisplay) | `GNU Lesser General Public License v3.0 only` | `fpsdisplay-5.2.0+26.3` | Ported ([`fps-display`](mods/fps-display/)) | FPS Display |
| KeyOverlay-26.2 | [keyoverlay](https://modrinth.com/mod/keyoverlay) | `see project` | `KeyOverlay-26.3` | Ported ([`key-overlay`](mods/key-overlay/)) | Key Overlay |
| simplezoom-fabric-26.2-1.3.0 | [zoom-simple](https://modrinth.com/mod/zoom-simple) | `MIT License` | `simplezoom-fabric-26.3-1.4.0` | Ported ([`simple-zoom`](mods/simple-zoom/)) | Simple Zoom |
| sprintindicator-fabric-2.0.0+26.2 | [sprint-indicator](https://modrinth.com/mod/sprint-indicator) | `MIT License` | `sprintindicator-fabric-2.1.0+26.3` | Ported ([`sprint-indicator`](mods/sprint-indicator/)) | Sprint Indicator |
| Zoomify (Zoom) | [zoomify](https://modrinth.com/mod/zoomify) | `GNU Lesser General Public License v3.0 only` | `zoomify-2.16.3+26.3` | Ported ([`zoomify`](mods/zoomify/)) | Zoomify |

## Partly ported (2)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| xaerominimap-fabric-1.21.11-25.3.10 | [xaeros-minimap](https://modrinth.com/mod/xaeros-minimap) | `see project` | `xaerominimap-fabric-26.3-26.5.3` | Partly ported ([`waypoints`](mods/waypoints/)) | Only the waypoints idea: /wp commands and a tracked-waypoint readout. Horizon ships its own minimap. |
| xaeroworldmap-fabric-1.21.11-1.40.11 | [xaeros-world-map](https://modrinth.com/mod/xaeros-world-map) | `see project` | `xaeroworldmap-fabric-26.3-1.46.4` | Partly ported ([`waypoints`](mods/waypoints/)) | Only the waypoints idea; there is no map screen hook yet. |

## Needs a hook point (44)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| [ETF] Entity Texture Features | [entitytexturefeatures](https://modrinth.com/mod/entitytexturefeatures) | `GNU Lesser General Public License v3.0 only` | `7.2.5-fabric-26.3` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| AppleSkin | [appleskin](https://modrinth.com/mod/appleskin) | `The Unlicense` | `appleskin-fabric-mc26.3-3.0.10` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| authme-fabric-9.3.0+26.2 | [auth-me](https://modrinth.com/mod/auth-me) | `MIT License` | `authme-fabric-9.3.0+26.3` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| Better Statistics Screen | [better-stats](https://modrinth.com/mod/better-stats) | `see project` | `betterstats-5.6.0-beta.1+fn-26.3` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| BetterGrassify | [bettergrassify](https://modrinth.com/mod/bettergrassify) | `Apache-2.0` | `1.8.8+fabric.26.3` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| bettermounthud-1.3.1 | [better-mount-hud](https://modrinth.com/mod/better-mount-hud) | `GNU General Public License v3.0 only` | `bettermounthud-1.2.6` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| Chat Animation [Smooth Chat] | [chatanimation](https://modrinth.com/mod/chatanimation) | `MIT License` | `chatanimation-fabric-1.3.3+mc26.3` | Needs a hook point | Needs incoming-chat or screen hooks that Horizon does not expose yet. |
| Chat Heads | [chat-heads](https://modrinth.com/mod/chat-heads) | `Mozilla Public License 2.0` | `chat_heads-1.2.9-fabric-26.3` | Needs a hook point | Needs incoming-chat or screen hooks that Horizon does not expose yet. |
| chatnotify-fabric-3.103.1+26.2 | [chatnotify](https://modrinth.com/mod/chatnotify) | `Apache License 2.0` | `chatnotify-fabric-3.104.0+26.3` | Needs a hook point | Needs incoming-chat or screen hooks that Horizon does not expose yet. |
| clientsort-fabric-3.103.1+26.2 | [clientsort](https://modrinth.com/mod/clientsort) | `Apache License 2.0` | `clientsort-fabric-3.104.0+26.3` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| Clumps | [clumps](https://modrinth.com/mod/clumps) | `MIT License` | `Clumps-fabric-26.3-26.3.2` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| Combat Hitboxes | [combat-hitboxes](https://modrinth.com/mod/combat-hitboxes) | `Apache License 2.0` | `combat-hitbox-1.0.11` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| Controlling | [controlling](https://modrinth.com/mod/controlling) | `MIT License` | `26.3.3` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| craftingtweaks-fabric-26.2-26.2.0.3 | [crafting-tweaks](https://modrinth.com/mod/crafting-tweaks) | `see project` | `craftingtweaks-fabric-26.3-26.3.0.1` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| crosshairindicator-1.0.3 | [crosshair-indicator](https://modrinth.com/mod/crosshair-indicator) | `Creative Commons Zero v1.0 Universal` | `crosshairindicator-1.0.3` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| Detail Armor Bar Reconstructed | [detail-armor-bar-reconstructed](https://modrinth.com/mod/detail-armor-bar-reconstructed) | `MIT License` | `DetailArmorBarReconstructed-5.3.2-26.3-fabric` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| Durability Tooltip | [durability-tooltip](https://modrinth.com/mod/durability-tooltip) | `see project` | `durabilitytooltip-1.2.0-fabric-mc26.3` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| EntityCrosshair-2.3.0-26.3+_fabric | [entitycrosshair](https://modrinth.com/mod/entitycrosshair) | `GNU General Public License v3.0 or later` | `EntityCrosshair-2.3.0-26.3+_fabric` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| Freecam | [freecam](https://modrinth.com/mod/freecam) | `MIT` | `1.5.0-alpha.2` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| freelook-1.4.3 | [freelook](https://modrinth.com/mod/freelook) | `GNU Affero General Public License v3.0 only` | `freelook-1.4.4` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| Gamma-Utils-Fabric-3.1.1+mc26.2 | [gamma-utils](https://modrinth.com/mod/gamma-utils) | `GNU Lesser General Public License v3.0 only` | `Gamma-Utils-Fabric-3.2.0+mc26.3` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| InvMove | [invmove](https://modrinth.com/mod/invmove) | `LGPL-3.0-only` | `0.9.6+26.3-fabric` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| Item Highlighter | [item-highlighter](https://modrinth.com/mod/item-highlighter) | `CC-BY-NC-ND-4.0` | `1.2.3` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| litematica-fabric-26.2-0.28.5 | [litematica](https://modrinth.com/mod/litematica) | `GNU Lesser General Public License v3.0 only` | `litematica-fabric-26.3-0.29.0` | Needs a hook point | Needs a screen or world-data hook point. |
| Marlow's Crystal Optimizer | [marlow-crystal-optimizer](https://modrinth.com/mod/marlow-crystal-optimizer) | `MIT License` | `Marlow Crystal Optimizer` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| modmenu-17.0.0 | [modmenu](https://modrinth.com/mod/modmenu) | `MIT License` | `modmenu-21.0.0` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| More Chat History | [morechathistory](https://modrinth.com/mod/morechathistory) | `Creative Commons Zero v1.0 Universal` | `2.0.0` | Needs a hook point | Needs incoming-chat or screen hooks that Horizon does not expose yet. |
| Mouse Tweaks | [mouse-tweaks](https://modrinth.com/mod/mouse-tweaks) | `BSD 3-Clause "New" or "Revised" License` | `MouseTweaks-fabric-mc26.3-2.31` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| multi-key-bindings-1.3.4+26.2 | [multi-key-bindings](https://modrinth.com/mod/multi-key-bindings) | `GNU General Public License v3.0 only` | `multi-key-bindings-1.3.5+26.3` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| NoChatReports-FABRIC-1.21.11-v2.18.0 | [no-chat-reports](https://modrinth.com/mod/no-chat-reports) | `Do What The F*ck You Want To Public License` | `NoChatReports-FABRIC-26.3-v2.21.0` | Needs a hook point | Needs incoming-chat or screen hooks that Horizon does not expose yet. |
| Pearl Optimizer | [pearl_optimizer](https://modrinth.com/mod/pearl_optimizer) | `see project` | `PearlOptimizer-26.1-26.3` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| Pick Up Notifier | [pick-up-notifier](https://modrinth.com/mod/pick-up-notifier) | `Mozilla Public License 2.0` | `pickupnotifier-v26.3.0-mc26.3.x+fabric` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| pingview-fabric-1.6 | [ping-view](https://modrinth.com/mod/ping-view) | `MIT License` | `pingview-fabric-1.6` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| potioncounter-fabric-1.12.0+mc26.2 | [potioncounter](https://modrinth.com/mod/potioncounter) | `MIT License` | `potioncounter-fabric-1.12.1+mc26.3` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| ScalableLux-0.1.6+fabric.c25518a-all | [scalablelux](https://modrinth.com/mod/scalablelux) | `GNU Lesser General Public License v3.0 only` | `ScalableLux-fabric-mc26.3-0.3.0-alpha.0.6-all` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| searchstats-fabric-1.103.0+26.2 | [searchstats](https://modrinth.com/mod/searchstats) | `Apache License 2.0` | `searchstats-fabric-1.103.0+26.2` | Needs a hook point | Needs a hook point Horizon does not have yet (the current hook points are listed in the docs). |
| Shulker Box Tooltip | [shulkerboxtooltip](https://modrinth.com/mod/shulkerboxtooltip) | `MIT License` | `shulkerboxtooltip-fabric-5.4.2+26.3` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| SnappyTappy | [snappytappy](https://modrinth.com/mod/snappytappy) | `MIT` | `26.3` | Needs a hook point | Needs input or camera hooks that Horizon does not expose yet. |
| status-effect-bars-1.0.10 | [status-effect-bars](https://modrinth.com/mod/status-effect-bars) | `GNU Lesser General Public License v3.0 only` | `status-effect-bars-1.0.12` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| statuseffecttimer-2.0.0+26.2 | [statuseffecttimer](https://modrinth.com/mod/statuseffecttimer) | `GNU Lesser General Public License v3.0 only` | `statuseffecttimer-2.0.0+26.3` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| tiers-0.8.3+26.2 | [tiers](https://modrinth.com/mod/tiers) | `GNU General Public License v3.0 only` | `tiers-0.9.0+26.3` | Needs a hook point | Needs a screen or world-data hook point. |
| totemcounter-fabric-1.13.0+mc26.2 | [totemcounter](https://modrinth.com/mod/totemcounter) | `MIT License` | `totemcounter-fabric-1.14.0+mc26.3` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| ukus-armor-hud-0.10.2+mc1.21.11 | [ukus-armor-hud](https://modrinth.com/mod/ukus-armor-hud) | `MIT License` | `ukus-armor-hud-fabric-0.13.0+mc26.3` | Needs a hook point | Needs inventory, effect or item data that the hook points do not give a mod yet. |
| VisualKeys-0.0.11+26.2 | [visualkeys](https://modrinth.com/mod/visualkeys) | `see project` | `VisualKeys-0.0.11+26.3` | Needs a hook point | Needs incoming-chat or screen hooks that Horizon does not expose yet. |

## Pack or asset (31)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| Anchor Optimizer | [anchoroptimizer](https://modrinth.com/mod/anchoroptimizer) | `see project` | `AnchorOptimizer-26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| betterhurtcam-fabric-1.14.0+mc26.2 | [betterhurtcam](https://modrinth.com/mod/betterhurtcam) | `MIT License` | `betterhurtcam-fabric-1.15.0+mc26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Block Outline Customizer | [block-outline-customizer](https://modrinth.com/mod/block-outline-customizer) | `MIT` | `1.5.0` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Capes | [capes](https://modrinth.com/mod/capes) | `GNU Lesser General Public License v2.1 only` | `capes-1.5.10+26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| centered-crosshair+1.21.11-1.4.0 | [centered-crosshair](https://modrinth.com/mod/centered-crosshair) | `MIT License` | `centered-crosshair+26.3-1.6.0` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| ClearWaterLava-Fabric-26.2 | [clear-waterlavapowdersnow](https://modrinth.com/mod/clear-waterlavapowdersnow) | `see project` | `ClearWaterLava-Fabric-26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Cobweb Outline | [cobweb-outline](https://modrinth.com/mod/cobweb-outline) | `MIT License` | `Cobweb Outline` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| ComplementaryUnbound_r5.8.1 | [complementary-unbound](https://modrinth.com/mod/complementary-unbound) | `see project` | `ComplementaryUnbound_r5.9.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| dark-loading-screen-1.6.19 | [dark-loading-screen](https://modrinth.com/mod/dark-loading-screen) | `GNU Lesser General Public License v3.0 only` | `dark-loading-screen-1.6.20` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Default Dark Mode: Expansion | [default-dark-mode-expansion](https://modrinth.com/mod/default-dark-mode-expansion) | `see project` | `Default-Dark-Mode-Expansion-1.19.0+-2026.9.1` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Default-Dark-Mode-26.2-2026.6.0 | [default-dark-mode](https://modrinth.com/mod/default-dark-mode) | `Creative Commons Attribution Non Commercial Share Alike 4.0 International` | `Default-Dark-Mode-26.3-2026.9.0` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Eating Animation[1.0.1] | [eating-animation-resourcepack](https://modrinth.com/mod/eating-animation-resourcepack) | `Creative Commons Attribution 4.0 International` | `Eating Animation[1.0.1]` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| EclipseUI-fabric-1.0.5+mc26.2-rc-2 | [eclipseui](https://modrinth.com/mod/eclipseui) | `MIT License` | `EclipseUI-fabric-1.0.5-fabric-26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Emissive Trims v1.5 | [emissive-trims](https://modrinth.com/mod/emissive-trims) | `Creative Commons Attribution 4.0 International` | `Emissive Trims v1.6` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| glow_trims-1.0 | [glow-trims](https://modrinth.com/mod/glow-trims) | `MIT License` | `glow_trims-1.0` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Low Fire | [low-fire-pack](https://modrinth.com/mod/low-fire-pack) | `see project` | `Low Fire` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Low Shield | [low-shield-pack](https://modrinth.com/mod/low-shield-pack) | `see project` | `Low Shield` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| miniature-shader-2.18.6 | [miniature-shader](https://modrinth.com/mod/miniature-shader) | `MIT License` | `miniature-shader-2.19` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| nofog-fabric-26.2 | [clear-fog](https://modrinth.com/mod/clear-fog) | `see project` | `nofog-fabric-26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Not Enough Animations | [not-enough-animations](https://modrinth.com/mod/not-enough-animations) | `see project` | `notenoughanimations-fabric-1.12.6-mc26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| real-arrow-tip-0.3.1+mc26.2 | [real-arrow-tip](https://modrinth.com/mod/real-arrow-tip) | `MIT License` | `real-arrow-tip-0.3.1+mc26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Red Trapped Chests v1.0 | [red-trapped-chests](https://modrinth.com/mod/red-trapped-chests) | `Creative Commons Attribution Non Commercial Share Alike 4.0 International` | `Red Trapped Chests v1.0` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Remove Reloading Screen | [rrls](https://modrinth.com/mod/rrls) | `Open Software License 3.0` | `5.2.8+mc26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| shieldfixes-2.0.4+26.2 | [complete-shield-fixes](https://modrinth.com/mod/complete-shield-fixes) | `MIT License` | `shieldfixes-2.0.4+26.2` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Short Swords | [short-swords-pack](https://modrinth.com/mod/short-swords-pack) | `Creative Commons Attribution 4.0 International` | `Short Swords` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| skinlayers3d-fabric-1.11.2-mc26.2 | [3dskinlayers](https://modrinth.com/mod/3dskinlayers) | `see project` | `skinlayers3d-fabric-1.11.3-mc26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| skyboxify-3.3.1+26.2-fabric | [skyboxify](https://modrinth.com/mod/skyboxify) | `GNU General Public License v3.0 only` | `skyboxify-3.4+26.3-fabric` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| Unique Dark - Lite - 1.20.2-26.x | [unique-dark](https://modrinth.com/mod/unique-dark) | `see project` | `Unique Dark - Lite - 1.20.2-26.x` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| visual_armor_trims_4.2 | [visual-armor-trims](https://modrinth.com/mod/visual-armor-trims) | `Creative Commons Attribution Share Alike 4.0 International` | `visual_armor_trims_4.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| visuality-0.7.15+26.3 | [visuality](https://modrinth.com/mod/visuality) | `MIT License` | `visuality-0.7.15+26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |
| waveycapes-fabric-1.10.2-mc26.2 | [wavey-capes](https://modrinth.com/mod/wavey-capes) | `see project` | `waveycapes-fabric-1.11.2-mc26.3` | Pack or asset | Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods. |

## Engine or server (31)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| BadOptimizations | [badoptimizations](https://modrinth.com/mod/badoptimizations) | `MIT License` | `BadOptimizations-2.4.1-26.3-fabric` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Concurrent Chunk Management Engine (Fabric) | [c2me-fabric](https://modrinth.com/mod/c2me-fabric) | `MIT License` | `c2me-fabric-mc26.3-0.4.2-alpha.0.88` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| continuity-3.0.1+26.2 | [continuity](https://modrinth.com/mod/continuity) | `GNU Lesser General Public License v3.0 only` | `continuity-3.0.1+26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| CrashAssistant-fabric-26.2-1.11.12 | [crash-assistant](https://modrinth.com/mod/crash-assistant) | `see project` | `CrashAssistant-fabric-26.2-26.3-1.11.14` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| debugify-26.2.0.0 | [debugify](https://modrinth.com/mod/debugify) | `GNU Lesser General Public License v3.0 only` | `debugify-26.3.0.0` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Dynamic FPS | [dynamic-fps](https://modrinth.com/mod/dynamic-fps) | `MIT License` | `dynamic-fps-3.11.10+minecraft-26.3.0-fabric` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| e4mc | [e4mc](https://modrinth.com/mod/e4mc) | `MIT License` | `6.2.3-fabric-modern` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Entity Culling | [entityculling](https://modrinth.com/mod/entityculling) | `see project` | `entityculling-fabric-1.11.2-mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Fast IP Ping | [fast-ip-ping](https://modrinth.com/mod/fast-ip-ping) | `GNU Lesser General Public License v3.0 only` | `fast-ip-ping-v1.0.12-mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| fastquit-3.1.5+mc26.2 | [fastquit](https://modrinth.com/mod/fastquit) | `MIT License` | `fastquit-3.1.5+mc26.2` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| FerriteCore | [ferrite-core](https://modrinth.com/mod/ferrite-core) | `MIT License` | `ferritecore-9.0.0-fabric` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| ImmediatelyFast | [immediatelyfast](https://modrinth.com/mod/immediatelyfast) | `GNU Lesser General Public License v3.0 or later` | `1.17.1+26.3-fabric` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| inventorytotem-26.2.0-3.4 | [inventory-totem](https://modrinth.com/mod/inventory-totem) | `see project` | `inventorytotem-26.3.0-3.4` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| iris-fabric-1.10.7+mc1.21.11 | [iris](https://modrinth.com/mod/iris) | `GNU Lesser General Public License v3.0 only` | `iris-fabric-1.11.6+mc26.3`; `iris-fabric-1.11.7+mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Krypton | [krypton](https://modrinth.com/mod/krypton) | `GNU Lesser General Public License v3.0 only` | `0.3.2` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Lithium | [lithium](https://modrinth.com/mod/lithium) | `GNU Lesser General Public License v3.0 only` | `lithium-fabric-0.26.1+mc26.3`; `lithium-fabric-0.26.2+mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| modernfix-5.27.19-build.1 | [modernfix-mvus](https://modrinth.com/mod/modernfix-mvus) | `GNU Lesser General Public License v3.0 only` | `modernfix-5.27.20-build.1` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| More Culling | [moreculling](https://modrinth.com/mod/moreculling) | `GNU General Public License v3.0 only` | `moreculling-fabric-26.3-1.9.0-beta.1` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| nvidium-0.4.4-beta4-26.2 | [nvidium](https://modrinth.com/mod/nvidium) | `GNU Lesser General Public License v3.0 only` | `nvidium-0.4.4-beta7-26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Packet Fixer | [packet-fixer](https://modrinth.com/mod/packet-fixer) | `MIT License` | `PacketFixer-fabric-3.3.6` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Reese's Sodium Options | [reeses-sodium-options](https://modrinth.com/mod/reeses-sodium-options) | `MIT License` | `reeses-sodium-options-fabric-2.2.5+mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| servercore-fabric-1.5.19+26.2 | [servercore](https://modrinth.com/mod/servercore) | `MIT License` | `servercore-fabric-1.5.20+26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Silicon-26.1.X | [silicons](https://modrinth.com/mod/silicons) | `MIT License` | `Silicon-1.21.X-26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Sodium | [sodium](https://modrinth.com/mod/sodium) | `see project` | `sodium-fabric-0.9.2+mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Sodium Shadowy Path Blocks (SSPB) | [sodium-shadowy-path-blocks](https://modrinth.com/mod/sodium-shadowy-path-blocks) | `GNU Lesser General Public License v3.0 only` | `sodium-shadowy-path-blocks-fabric-8.0.0` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| sodium-extra-fabric-0.8.3+mc1.21.11 | [sodium-extra](https://modrinth.com/mod/sodium-extra) | `GNU Lesser General Public License v3.0 only` | `sodium-extra-fabric-0.9.4+mc26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| SodiumTranslations | [translations-for-sodium](https://modrinth.com/mod/translations-for-sodium) | `Creative Commons Zero v1.0 Universal` | `SodiumTranslations` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| spark-1.10.173-fabric | [spark](https://modrinth.com/mod/spark) | `GNU General Public License v3.0 only` | `spark-1.10.187-fabric` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| Very Many Players (Fabric) | [vmp-fabric](https://modrinth.com/mod/vmp-fabric) | `MIT License` | `vmp-fabric-mc26.3-0.2.0+beta.7.238-all` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| voicechat-fabric-1.21.11-2.6.15 | [simple-voice-chat](https://modrinth.com/mod/simple-voice-chat) | `see project` | `voicechat-fabric-2.6.24+26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |
| zfastnoise-1.0.40+26.2 | [zfastnoise](https://modrinth.com/mod/zfastnoise) | `Mozilla Public License 2.0` | `zfastnoise-1.1.0-beta.5+26.3` | Engine or server | Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own. |

## Library (28)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| adventure-platform-fabric-7.2.0 | [adventure-platform-mod](https://modrinth.com/mod/adventure-platform-mod) | `MIT License` | `adventure-platform-fabric-7.2.0` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| balm-fabric-26.2-26.2.0.6 | [balm](https://modrinth.com/mod/balm) | `see project` | `balm-fabric-26.3-26.3.0.1` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Better Compatibility Checker | [better-compatibility-checker](https://modrinth.com/mod/better-compatibility-checker) | `see project` | `better-compatability-checker-fabric-26.3.0.1` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Bookshelf-fabric-MC26.3-26.3.0.1 | [bookshelf-lib](https://modrinth.com/mod/bookshelf-lib) | `GNU Lesser General Public License v2.1 only` | `Bookshelf-fabric-MC26.3-26.3.0.1` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| clienttweaks-fabric-26.2-26.2.0.6 | [client-tweaks](https://modrinth.com/mod/client-tweaks) | `see project` | `clienttweaks-fabric-26.3-26.3.0.1` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| cloth-config-21.11.153-fabric | [cloth-config](https://modrinth.com/mod/cloth-config) | `GNU Lesser General Public License v3.0 only` | `cloth-config-fabric-26.3.159` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| collective-26.2.0-8.39 | [collective](https://modrinth.com/mod/collective) | `see project` | `collective-26.3.0-8.40` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| compose-bundle-1.0.4+compose.1.12.0 | [compose-multiplatform](https://modrinth.com/mod/compose-multiplatform) | `Apache License 2.0` | `compose-bundle-1.0.4+compose.1.12.0` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Essential_1-4-1-1_fabric_26-2 | [essential](https://modrinth.com/mod/essential) | `see project` | `Essential_1-5-0-1_fabric_26-3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Fabric Language Kotlin | [fabric-language-kotlin](https://modrinth.com/mod/fabric-language-kotlin) | `Apache License 2.0` | `fabric-language-kotlin-1.14.1+kotlin.2.4.20` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| fabric-api-0.141.3+1.21.11 | [fabric-api](https://modrinth.com/mod/fabric-api) | `Apache License 2.0` | `fabric-api-0.161.0+26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Forge Config API Port | [forge-config-api-port](https://modrinth.com/mod/forge-config-api-port) | `Mozilla Public License 2.0` | `ForgeConfigAPIPort-v26.3.1-mc26.3.x-Fabric` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| fzzy_config-0.7.6+26.2 | [fzzy-config](https://modrinth.com/mod/fzzy-config) | `see project` | `fzzy_config-0.7.7+fix2+26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| konkrete_fabric_1.11.1_MC_26.2 | [konkrete](https://modrinth.com/mod/konkrete) | `Apache License 2.0` | `konkrete_fabric_1.11.1_MC_26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| libjf-26.2.2 | [libjf](https://modrinth.com/mod/libjf) | `GNU Lesser General Public License v3.0 or later` | `libjf-26.3.1` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| malilib-fabric-26.2-0.29.4 | [malilib](https://modrinth.com/mod/malilib) | `GNU Lesser General Public License v3.0 only` | `malilib-fabric-26.3-0.30.1` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| midnightlib-fabric-1.9.3+1.21.11 | [midnightlib](https://modrinth.com/mod/midnightlib) | `MIT License` | `midnightlib-fabric-1.9.3+26.3-rc-2` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| OneConfig-26.3-fabric-1.2.6 | [oneconfig](https://modrinth.com/mod/oneconfig) | `see project` | `OneConfig-26.3-fabric-1.2.6` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| PrickleMC-fabric-MC26.2-26.2.0.3 | [prickle](https://modrinth.com/mod/prickle) | `GNU Lesser General Public License v2.1 only` | `PrickleMC-fabric-MC26.3-26.3.0.2` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| PuzzlesLib-v26.2.3-mc26.2.x-Fabric | [puzzles-lib](https://modrinth.com/mod/puzzles-lib) | `Mozilla Public License 2.0` | `puzzleslib-v26.3.4-mc26.3.x+fabric` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Resourcify (26.2-fabric)-1.8.5 | [resourcify](https://modrinth.com/mod/resourcify) | `GNU Lesser General Public License v3.0 only` | `Resourcify (26.3-fabric)-1.8.6` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| SuperMartijn642's Config Lib | [supermartijn642s-config-lib](https://modrinth.com/mod/supermartijn642s-config-lib) | `see project` | `supermartijn642configlib-1.1.8-fabric-mc26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| tcdcommons-5.5.6+fn-26.2 | [tcdcommons](https://modrinth.com/mod/tcdcommons) | `see project` | `tcdcommons-5.6.0-beta.1+fn-26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| Text Placeholder API | [placeholder-api](https://modrinth.com/mod/placeholder-api) | `GNU Lesser General Public License v3.0 only` | `placeholder-api-3.2.0+26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| ukulib-1.10.2+1.21.11 | [ukulib](https://modrinth.com/mod/ukulib) | `Mozilla Public License 2.0` | `ukulib-fabric-2.2.0+26.3` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| WalksyLib-1.0.12+26.2 | [walksylib](https://modrinth.com/mod/walksylib) | `MIT License` | `WalksyLib-1.0.12+26.2` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| YetAnotherConfigLib (YACL) | [yacl](https://modrinth.com/mod/yacl) | `GNU Lesser General Public License v3.0 or later` | `yet_another_config_lib_v3-3.9.7+26.3-fabric` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |
| zconfig-1.0.0+26.x | [zconfig](https://modrinth.com/mod/zconfig) | `Mozilla Public License 2.0` | `zconfig-1.0.0+26.x` | Library | A library other Fabric mods load; Horizon's native mods do not use Fabric libraries. |

## No 26.3 build (45)

| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |
|---|---|---|---|---|---|
| [EMF] Entity Model Features | [entity-model-features](https://modrinth.com/mod/entity-model-features) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| alternate-current-mc1.21.11-1.9.0 | [alternate-current](https://modrinth.com/mod/alternate-current) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Better Hitboxes | [betterhitboxes](https://modrinth.com/mod/betterhitboxes) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| BetterF3-19.0.0-Fabric-26.2 | [betterf3](https://modrinth.com/mod/betterf3) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Bobby | [bobby](https://modrinth.com/mod/bobby) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Carpet | [carpet](https://modrinth.com/mod/carpet) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| chatpatches-8.0-alpha.11+26.2-fabric | [chatpatches](https://modrinth.com/mod/chatpatches) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| CIT Resewn | [cit-resewn](https://modrinth.com/mod/cit-resewn) | `MIT` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| ConsumableOptimizer-2.2.2-26.2 | [consumableoptimizer](https://modrinth.com/mod/consumableoptimizer) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| CrosshairAddons-2.1.4-26.2 | [crosshair-addons-public](https://modrinth.com/mod/crosshair-addons-public) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Crystal Speed | [crystal-speed](https://modrinth.com/mod/crystal-speed) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Custom Glint | [custom-glint](https://modrinth.com/mod/custom-glint) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Custom Shield Scale | [shieldscale](https://modrinth.com/mod/shieldscale) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| cwb-4.1.0+26.2 | [cubes-without-borders](https://modrinth.com/mod/cubes-without-borders) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Enchantment Descriptions | [enchantment-descriptions](https://modrinth.com/mod/enchantment-descriptions) | `GNU Lesser General Public License v2.1 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Enchantment Insights | [enchantment-insights](https://modrinth.com/mod/enchantment-insights) | `Mozilla Public License 2.0` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Enhanced Block Entities | [ebe](https://modrinth.com/mod/ebe) | `LGPL-3.0-only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| f5split-2.1.0+mc26.2 | [f5-split](https://modrinth.com/mod/f5-split) | `see project` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Fullbright | [fullbright](https://modrinth.com/mod/fullbright) | `GPL-3.0-only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| herobot-1.21.11-1.4.3+v260315 | [herobot](https://modrinth.com/mod/herobot) | `Creative Commons Attribution Non Commercial 4.0 International` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| hudmanager-4.0.3 | [hud-manager](https://modrinth.com/mod/hud-manager) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Item Scale | [itemscale](https://modrinth.com/mod/itemscale) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| lambdynamiclights-4.9.1+1.21.11 | [lambdynamiclights](https://modrinth.com/mod/lambdynamiclights) | `see project` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| language-reload-1.7.7+26.2 | [language-reload](https://modrinth.com/mod/language-reload) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Mace Optimizer | [mace_optimizer](https://modrinth.com/mod/mace_optimizer) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Memory Leak Fix | [memoryleakfix](https://modrinth.com/mod/memoryleakfix) | `LGPL-2.1-only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| minihud-fabric-1.21.11-0.38.2 | [minihud](https://modrinth.com/mod/minihud) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Model Gap Fix | [modelfix](https://modrinth.com/mod/modelfix) | `GPL-3.0-only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| No Death Animation | [no-death-animation](https://modrinth.com/mod/no-death-animation) | `MIT` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| noisium-fabric-2.8.5+mc26.2-pre-2 | [noisiumforked](https://modrinth.com/mod/noisiumforked) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| noxesium-fabric-3.2.4 | [noxesium](https://modrinth.com/mod/noxesium) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Particle Core | [particle-core](https://modrinth.com/mod/particle-core) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Particle Rain | [particle-rain](https://modrinth.com/mod/particle-rain) | `MIT` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| puzzle-fabric-2.3.1+26.2 | [puzzle](https://modrinth.com/mod/puzzle) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| reflex-fabric-1.0.4+26.2 | [reflex-antilag](https://modrinth.com/mod/reflex-antilag) | `see project` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Searchables-fabric-1.21.11-1.0.2 | [searchables](https://modrinth.com/mod/searchables) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| ShieldStatus-4.1.8+26.2 | [shield-statuses](https://modrinth.com/mod/shield-statuses) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| slyde-26.2.0 | [slyde](https://modrinth.com/mod/slyde) | `MIT License` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| ToolTipFix | [tooltipfix](https://modrinth.com/mod/tooltipfix) | `MIT` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| totemtweaks-26.2-1.2.0 | [cpvp](https://modrinth.com/mod/cpvp) | `see project` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| tweakeroo-fabric-1.21.11-0.27.2 | [tweakeroo](https://modrinth.com/mod/tweakeroo) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| voxy-0.2.8-alpha | [voxy](https://modrinth.com/mod/voxy) | `see project` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| VulkanMod | [vulkanmod](https://modrinth.com/mod/vulkanmod) | `LGPL-3.0-only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| Xp Bottle Optimizer | [xp-bottle-optimizer](https://modrinth.com/mod/xp-bottle-optimizer) | `LicenseRef-All-Rights-Reserved` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |
| yosbr-0.1.2 | [yosbr](https://modrinth.com/mod/yosbr) | `GNU Lesser General Public License v3.0 only` | `-` | No 26.3 build | No Fabric build for 26.3, the game version Horizon implements. |

