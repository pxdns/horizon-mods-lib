# Crystal Optimizer

![Crystal Optimizer icon](../../assets/icons/catalogue/marlow-crystal-optimizer.png)

Makes End crystals you hit disappear at once on a server, and brings one back if the server did not break it.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/crystal-optimizer) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crystal-optimizer)

## Use it

Nothing to press: on a server, an End crystal you hit vanishes the moment you swing instead of when the server's answer arrives. If the server does not remove it within 1.5 seconds it comes back. The server still decides everything; this only changes what you see.

| Option | Default |
|---|---|
| Hide a hit crystal at once | on |

Switch it on or off in **Options → Mods → Crystal Optimizer**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `remote::Game::send`, `remote_projectiles::remove_remote_projectile`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/crystal-optimizer/mixins/crystal_optimizer/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Marlow's Crystal Optimizer (marlowcrystal)** (MIT); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
