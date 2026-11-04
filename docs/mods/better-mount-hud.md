# Better Mount HUD

![Better Mount HUD icon](../../assets/icons/better-mount-hud.svg)

Numbers for the mount you ride: health, speed in blocks per second, jump height and jump charge.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/better-mount-hud) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=better-mount-hud)

## Use it

Nothing to press: while you ride, a box in the bottom-left corner shows the mount's health as numbers (red when low), how fast it moves in blocks per second, and for a saddled horse, donkey or similar how high it jumps and how charged your jump is. `/mount` hides or shows it.

| Option | Default |
|---|---|
| Shown | on |
| Speed | on |
| Jump height and charge | on |

Switch it on or off in **Options → Mods → Better Mount HUD**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/better-mount-hud/mixins/better_mount_hud/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Better Mount HUD (bettermounthud)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
