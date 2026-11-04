# Entity Crosshair

![Entity Crosshair icon](../../assets/icons/catalogue/entitycrosshair.png)

Replaces the crosshair with a style and colour of your choice, red over living entities.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/entity-crosshair) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=entity-crosshair)

## Use it

Nothing to press: the settings page picks the crosshair's style (plus, gap plus, small plus, dot, ring) and colour, and whether it turns red over a living entity.



Switch it on or off in **Options → Mods → Entity Crosshair**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_frame`, `hud_layout::crosshair`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/entity-crosshair/mixins/entity_crosshair/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Entity Crosshair (EntityCrosshair)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
