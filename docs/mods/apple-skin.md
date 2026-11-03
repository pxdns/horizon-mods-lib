# Apple Skin

![Apple Skin icon](../../assets/icons/apple-skin.svg)

Your hunger and saturation as numbers, and what the food in your hand would restore.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/apple-skin) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=apple-skin)

## Use it

| Command | What it does |
|---|---|
| `/food` | Hides or shows Apple Skin |

| Option | Default |
|---|---|
| Shown | on |
| Held food preview | on |
| Show saturation | on |

Switch it on or off in **Options → Mods → Apple Skin**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/apple-skin/mixins/apple_skin/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **AppleSkin (appleskin)** (The Unlicense); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
