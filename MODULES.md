# Modules

## Tempest's modules

The Tempest client (`pxdns/patcher`) groups its features into modules. This is the full list from the 26.2 module docs
and the 1.16.5 port table, with where each one stands on Horizon. The 1.16.5 browser port keeps three modules working
(Fullbright, InvMove, No Death Animation) plus the item-scale, Clean F3 and hit-crosshair hooks; see
[`docs/1.16.5/PORT-STATUS.md`](https://github.com/pxdns/patcher/blob/main/docs/1.16.5/PORT-STATUS.md).

| Tempest module | Horizon |
|---|---|
| `apple-skin` | Needs a hook point |
| `appleskin` | Needs a hook point |
| `armor-hud` | Needs a hook point |
| `better-grass` | Needs a hook point |
| `better-stats` | Needs a hook point |
| `bettergrassify` | Needs a hook point |
| `bobby` | Needs a hook point |
| `capes` | Needs a hook point |
| `chat-heads` | Needs a hook point |
| `chatanimation` | Needs a hook point |
| `cit-resewn` | Needs a hook point |
| `cleanf3` | Needs a hook point |
| `controlling` | Needs a hook point |
| `crosshair-plus` | Needs a hook point |
| `crystal-speed` | Needs a hook point |
| `custom-crosshair` | Needs a hook point |
| `customhud` | Ported: [`custom-hud`](mods/custom-hud/) |
| `dynamic-fps` | Needs a hook point |
| `effect-timers` | Needs a hook point |
| `enhanced-block-entities` | Needs a hook point |
| `entity-model-features` | Needs a hook point |
| `entity-texture-features` | Needs a hook point |
| `entitytexturefeatures` | Needs a hook point |
| `freecam` | Needs a hook point |
| `fullbright` | Horizon has its own fullbright setting; no mod needed |
| `held-item` | Needs a hook point |
| `hud-editor` | Needs a hook point |
| `invmove` | Needs a hook point |
| `item-highlighter` | Needs a hook point |
| `modmenu-settings` | Needs a hook point |
| `more-chat-history` | Needs a hook point |
| `morechathistory` | Needs a hook point |
| `mouse-tweaks` | Needs a hook point |
| `no-death-animation` | Needs a hook point |
| `no-particles` | Needs a hook point |
| `no-weather` | Needs a hook point |
| `red-crosshair-indicator` | Needs a hook point |
| `shulkerboxtooltip` | Needs a hook point |
| `sprint-display` | Ported: [`sprint-indicator`](mods/sprint-indicator/) |
| `tempest-info` | Needs a hook point |
| `tooltip-fix` | Needs a hook point |
| `tooltipfix` | Needs a hook point |
| `totem-counter` | Needs a hook point |
| `transparent-shield` | Needs a hook point |
| `ukus-armor-hud` | Needs a hook point |
| `xp-bottle-optimizer` | Needs a hook point |
| `zoomify` | Ported: [`zoomify`](mods/zoomify/) |

Two projects here have no Tempest module behind them: `cps-counter` and `info-hud` are small utilities written for this
library (`info-hud` follows MiniHUD's info lines, `cps-counter` the usual PvP readout).

## The Horizon organisation

[`github.com/horizon-rust`](https://github.com/horizon-rust) holds eight public repositories:

| Repository | What it is |
|---|---|
| [`horizon`](https://github.com/horizon-rust/horizon) | The game, launcher, server and tools: a rewrite of Minecraft Java 26.3 in Rust (MIT) |
| [`horizon-mods`](https://github.com/horizon-rust/horizon-mods) | The registry of reviewed mods, modpacks and resource and shader packs the launcher lists; also Horizon's own native mods |
| [`horizon-site`](https://github.com/horizon-rust/horizon-site) | The website at playhorizon.dev |
| [`horizon-mixins`](https://github.com/horizon-rust/horizon-mixins) | Compile-time source mixins for Horizon, by levzzz5154 (MIT); the tool every native mod is built with |
| [`horizon-native-mod-template`](https://github.com/horizon-rust/horizon-native-mod-template) | The template these projects follow |
| [`horizon-server`](https://github.com/horizon-rust/horizon-server) | A Minecraft Java 26.3 server written in Rust |
| [`kira-pumpkin`](https://github.com/horizon-rust/kira-pumpkin) | Kira, the Discord bot, answering in a Pumpkin server's chat |
| [`.github`](https://github.com/horizon-rust/.github) | The organisation profile |

Horizon's own native mods in `horizon-mods/native/` are `coords-hud`, `fps-overlay`, `keystrokes`, `minimap`, `zoom`,
`recipes`, `smp` and `voicechat-server`. This library adds independent mods beside them; none of them replaces those.

Horizon is an unofficial project. It is not approved by or associated with Mojang or Microsoft.
