# Clumps

![Clumps icon](../../assets/icons/catalogue/clumps.png)

Groups nearby experience orbs into a single orb, which cuts the entity count and is collected at once.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/clumps) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=clumps)

## Use it

Nothing to press: experience orbs that lie close together merge into one orb worth all of them, whatever their sizes, and you collect it the moment you touch it. The first button switches merging of different sizes off.

| Option | Default |
|---|---|
| Merge orbs of different sizes | on |

Switch it on or off in **Options → Mods → Clumps**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `entities::merge_orbs`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/clumps/mixins/clumps/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Clumps (clumps)** (MIT); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
