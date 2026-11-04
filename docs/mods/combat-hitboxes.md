# Combat Hitboxes

![Combat Hitboxes icon](../../assets/icons/combat-hitboxes.svg)

Makes entities slightly easier to hit by growing the box your crosshair is tested against.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/combat-hitboxes) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=combat-hitboxes)

## Use it

Nothing to press: entities are a little easier to hit, because the box your crosshair is tested against is bigger. The first button picks how much (0, 0.05, 0.1, 0.2 or 0.3 blocks).



Switch it on or off in **Options → Mods → Combat Hitboxes**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `game::ray_box`, `native_frame`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/combat-hitboxes/mixins/combat_hitboxes/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Combat Hitboxes (combat-hitbox)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
