# Crosshair Indicator

![Crosshair Indicator icon](../../assets/icons/catalogue/crosshair-indicator.png)

Shows corner brackets around your crosshair when you're aiming at a player.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API 1** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/crosshair-indicator) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crosshair-indicator)

## Use it

Nothing to press and nothing to set: switch it on in Options > Mods, and corner brackets show round your crosshair whenever you aim at another player.



Switch it on or off in **Options → Mods → Crosshair Indicator**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_mods`.
- Logic and tests: `mods/crosshair-indicator/mixins/crosshair_indicator/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **Crosshair Indicator (crosshairindicator) by SwimmingDog1 and MGC8** (CC0-1.0); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
