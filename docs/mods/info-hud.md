# Info HUD

![Info HUD icon](../../assets/icons/info-hud.svg)

Your speed, the way you face and your dimension in a corner of the screen, in the spirit of MiniHUD's info lines.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API** 1 · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/info-hud) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=info-hud)

## Use it

| Command | What it does |
|---|---|
| `/info` | Hides or shows Info HUD |

| Option | Default |
|---|---|
| Shown | on |
| Speed | on |
| Facing | on |
| Dimension | off |
| Yaw and pitch | off |

Switch it on or off in **Options → Mods → Info HUD**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_tick`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/info-hud/mixins/info_hud/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **MiniHUD (info lines)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
