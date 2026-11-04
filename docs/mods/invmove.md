# InvMove

![InvMove icon](../../assets/icons/invmove.svg)

Keep walking, jumping and sneaking while your inventory is open.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/invmove) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=invmove)

## Use it

Open your inventory and keep holding W A S D: you keep walking. Space and Shift also jump and sneak unless you switch that off in the settings. Clicking in the inventory never attacks or uses an item.

| Option | Default |
|---|---|
| Jump and sneak too | on |

Switch it on or off in **Options → Mods → InvMove**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `game::Game::tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/invmove/mixins/invmove/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **InvMove (invmove)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
