# BetterGrassify

![BetterGrassify icon](../../assets/icons/horizon-mods-lib.svg)

Connected grass: a grass block's side shows the grass top where the neighbouring step down is grass too.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/bettergrassify) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=bettergrassify)

## Use it

Nothing to press: where a grass block's side is exposed next to a step down onto more grass, the side shows the grass top instead of the dirt strip with a grass fringe, so a grassy slope looks continuous (OptiFine's "Better Grass: Fancy").

| Option | Default |
|---|---|
| Better grass on grass block sides | on |

Switch it on or off in **Options → Mods → BetterGrassify**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `mesher::lighting::ambient_occlusion`, `mesher::Mesher::quad`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/bettergrassify/mixins/bettergrassify/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **BetterGrassify (bettergrass)** (Apache-2.0 for the idea; no upstream code is used (its sprite code is LGPL from another project)); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
