# FPS Display

![FPS Display icon](../../assets/icons/fps-display.svg)

Frames per second in a corner of the screen, coloured by how smooth they are.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API 1** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/fps-display) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=fps-display)

## Use it

| Command | What it does |
|---|---|
| `/fps` | Hides or shows FPS Display |

| Option | Default |
|---|---|
| Shown | on |
| Slowest frame | off |
| 1% low | off |
| Colour by rate | on |

Switch it on or off in **Options → Mods → FPS Display**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/fps-display/mixins/fps_display/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **FPS - Display (fpsdisplay)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
