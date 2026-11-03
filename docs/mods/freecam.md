# Freecam

![Freecam icon](../../assets/icons/freecam.svg)

Detach the camera and fly it around while your player stands still.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/freecam) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=freecam)

## Use it

| Key | Default | What it does |
|---|---|---|
| Freecam | `F8` | Press to detach the camera, press again to return. W A S D fly in the direction you look, Space goes up, Shift down, Ctrl faster |

| Option | Default |
|---|---|
| Fast flight (3x) | off |

Switch it on or off in **Options → Mods → Freecam**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_key`, `native_frame`, `native_tick`, `perspective::turn_player`, `perspective::camera_view`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/freecam/mixins/freecam/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Freecam (freecam)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
