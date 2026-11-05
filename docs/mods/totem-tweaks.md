# Totem Tweaks

![Totem Tweaks icon](../../assets/icons/catalogue/cpvp.png)

Customise the totem of undying pop animation: off, speed, size, spin, rocking and position.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/totem-tweaks) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=totem-tweaks)

## Use it

Nothing to press: the settings page (Options > Mods > Totem Tweaks) switches the totem pop animation off, picks its speed (0.5 to 3 times), its size (50 to 200 %), and switches the spinning, the rocking and the sideways swing off.



Switch it on or off in **Options → Mods → Totem Tweaks**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `item_activation::animation_pose`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/totem-tweaks/mixins/totem_tweaks/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Totem Tweaks (totem-tweaks)** (independent implementation from the mod's public description; no upstream code or assets); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
