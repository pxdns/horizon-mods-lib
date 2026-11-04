# Crosshair Indicator

![Crosshair Indicator icon](../../assets/icons/crosshair-indicator.svg)

Turns your crosshair red when it is over a living creature.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/crosshair-indicator) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crosshair-indicator)

## Use it

Nothing to press: when your crosshair is over a living, alive entity, a red mark is drawn over it. The settings page can swap the cross for a dot.

| Option | Default |
|---|---|
| Shown | on |
| Dot instead of cross | off |

Switch it on or off in **Options → Mods → Crosshair Indicator**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/crosshair-indicator/mixins/crosshair_indicator/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Crosshair Indicator (crosshairindicator)** (CC0-1.0); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
