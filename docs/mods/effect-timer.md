# Effect Timer

![Effect Timer icon](../../assets/icons/catalogue/statuseffecttimer.png)

Your active potion effects and how long each has left, red in the last ten seconds, with a duration bar under each effect icon.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/effect-timer) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=effect-timer)

## Use it

| Command | What it does |
|---|---|
| `/effects` | Hides or shows Effect Timer |

| Option | Default |
|---|---|
| Shown | on |
| Show beacon effects | on |
| Duration bars under the icons | on |

Switch it on or off in **Options → Mods → Effect Timer**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/effect-timer/mixins/effect_timer/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Status Effect Timer and Status Effect Bars** (see the upstream projects); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
