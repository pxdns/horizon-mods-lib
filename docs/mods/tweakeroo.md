# Tweakeroo

![Tweakeroo icon](../../assets/icons/catalogue/tweakeroo.png)

Four of Tweakeroo's tweaks: hand restock, hotbar cycle, fast right click and a tool break warning.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/tweakeroo) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=tweakeroo)

## Use it

| Tweak | What it does |
|---|---|
| Hand restock | When the stack in your hand runs out, a stack of the same item from your inventory takes its place |
| Hotbar cycle (`;`) | Rotates the hotbar and the three inventory rows: the hotbar goes to row 1, each row down one, the bottom row comes to the hotbar |
| Fast right click | Placing and using repeat as fast as the game allows while the button is held |
| Tool break warning | A chat warning when the tool in your hand is about to break |

Each has a switch on the settings page. Restock and the hotbar cycle send slot clicks to the server in a burst, which some servers' anti-cheat flags: ask before using them online.

| Option | Default |
|---|---|
| Hand restock | on |
| Hotbar cycle key | on |
| Fast right click | off |
| Tool break warning | on |

Switch it on or off in **Options → Mods → Tweakeroo**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_key`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/tweakeroo/mixins/tweakeroo/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Tweakeroo (tweakeroo)** (independent implementation from the mod's public description; no upstream code or assets (the original is LGPL-3.0)); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
