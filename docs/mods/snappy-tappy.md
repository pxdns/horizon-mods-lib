# Snappy Tappy

![Snappy Tappy icon](../../assets/icons/snappy-tappy.svg)

When you hold two opposite movement keys, the one pressed last wins, so you switch direction at once.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/snappy-tappy) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=snappy-tappy)

## Use it

Nothing to press: hold A and D (or W and S) together and the key you pressed last decides the direction; let go of it and the other carries on.

| Option | Default |
|---|---|
| A and D | on |
| W and S | on |

Switch it on or off in **Options → Mods → Snappy Tappy**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_key`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/snappy-tappy/mixins/snappy_tappy/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **SnappyTappy (SnappyTappy)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
