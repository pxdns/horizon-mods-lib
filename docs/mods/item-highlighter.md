# Item Highlighter

![Item Highlighter icon](../../assets/icons/horizon-mods-lib.svg)

Marks newly picked-up items with a twinkling star in your inventory.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/item-highlighter) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=item-highlighter)

## Use it

Nothing to press: pick something up, open your inventory, and every slot holding that kind of item shows a small twinkling gold star. The stars go away when you close the inventory (a setting keeps them instead until you pick up nothing new).

| Option | Default |
|---|---|
| Clear the stars when the inventory closes | on |

Switch it on or off in **Options → Mods → Item Highlighter**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `game::Game::draw_slot_item`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/item-highlighter/mixins/item_highlighter/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Item Highlighter (highlighter)** (independent implementation from the mod's public description; no upstream code or assets); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
