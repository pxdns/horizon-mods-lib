# Freelook

![Freelook icon](../../assets/icons/freelook.svg)

Hold a key and move the mouse to look around without turning your player.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/freelook) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=freelook)

## Use it

| Key | Default | What it does |
|---|---|---|
| Freelook | `Left Alt` | Hold (or press, with *Toggle*) and move the mouse to look around; your player keeps facing the same way |

| Option | Default |
|---|---|
| Toggle instead of hold | off |
| Invert pitch | off |

Switch it on or off in **Options → Mods → Freelook**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_key`, `perspective::turn_player`, `perspective::camera_view`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/freelook/mixins/freelook/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Freelook (freelook)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
