# Litematica

![Litematica icon](../../assets/icons/catalogue/litematica.png)

Read .litematic schematics: info, and the material list of blocks you need.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/litematica) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=litematica)

## Use it

| Command | What it does |
|---|---|
| `/schematic list` | The `.litematic` files in the mod's `schematics` folder |
| `/schematic info <name>` | Name, author, regions and sizes, block count |
| `/schematic materials <name> [n]` | The n most-needed blocks (default 20), with stacks |
| `/schematic export <name>` | Writes the whole material list to a text file |

Put your `.litematic` files in `horizon-mods/.horizon-mod-data/litematica/schematics/`. This is the schematic reading and material list of Litematica; it does not show a schematic in the world or place it.

| Option | Default |
|---|---|
| Show stacks next to the counts | on |

Switch it on or off in **Options → Mods → Litematica**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/litematica/mixins/litematica/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Litematica (litematica)** (independent implementation of the public .litematic file layout; no upstream code or assets (the original is LGPL-3.0)); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
