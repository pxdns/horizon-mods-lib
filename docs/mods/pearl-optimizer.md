# Pearl Optimizer

![Pearl Optimizer icon](../../assets/icons/horizon-mods-lib.svg)

Shows your thrown ender pearl immediately on a server, before the server's pearl arrives.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/pearl-optimizer) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=pearl-optimizer)

## Use it

Nothing to press: on a server, throwing an ender pearl shows a pearl flying at once instead of after the server's reply, and it is swapped for the server's pearl when that arrives (or removed after two seconds if the server never sends one). The server still decides everything; this only changes what you see.

| Option | Default |
|---|---|
| Show my pearl at once | on |

Switch it on or off in **Options → Mods → Pearl Optimizer**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `remote::Game::send`, `creatures::Game::add_projectile`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/pearl-optimizer/mixins/pearl_optimizer/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Pearl Optimizer (pearloptimizer)** (independent implementation from the mod's public description; no upstream code or assets); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
