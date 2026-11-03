# Custom HUD

![Custom HUD icon](../../assets/icons/custom-hud.svg)

One box with the lines you pick: FPS, coordinates, facing and clicks per second, on the side you choose.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API** 1 · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/custom-hud) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=custom-hud)

## Use it

| Command | What it does |
|---|---|
| `/hud` | Hides or shows Custom HUD |

| Option | Default |
|---|---|
| Shown | on |
| Right side | off |
| FPS | on |
| Coordinates | on |
| Facing | on |
| Clicks per second | off |

Switch it on or off in **Options → Mods → Custom HUD**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/custom-hud/mixins/custom_hud/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **CustomHUD (profile idea)** (MIT); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
