# Entity Texture Features

![Entity Texture Features icon](../../assets/icons/catalogue/entitytexturefeatures.png)

Random texture variants and emissive layers for mobs, from the textures a resource pack provides.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/entity-texture-features) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=entity-texture-features)

## Use it

Nothing to press, but it needs a resource pack that adds textures: a mob whose texture is `entity/zombie/zombie` picks, by its entity id, one of `zombie`, `zombie2`, `zombie3`... that the pack provides (the same one every time), and a texture `zombie_e` is drawn over the body at full brightness (glowing eyes). Without such textures it changes nothing.

| Option | Default |
|---|---|
| Random texture variants | on |
| Emissive layers | on |

Switch it on or off in **Options → Mods → Entity Texture Features**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `mobs::render::body`, `mobs::render::layers`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/entity-texture-features/mixins/entity_texture_features/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Entity Texture Features (etf)** (independent implementation from the mod's public description; no upstream code or assets); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
