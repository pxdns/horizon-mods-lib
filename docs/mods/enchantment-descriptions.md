# Enchantment Descriptions

![Enchantment Descriptions icon](../../assets/icons/catalogue/enchantment-insights.png)

Adds a line to enchanted items' tooltips saying what each enchantment does.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/enchantment-descriptions) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=enchantment-descriptions)

## Use it

Nothing to press: hover an enchanted item or book and its tooltip gains a gray line for each enchantment saying what it does, such as `Sharpness: Increases melee damage.`

| Option | Default |
|---|---|
| On enchanted books too | on |
| On enchanted items | on |

Switch it on or off in **Options → Mods → Enchantment Descriptions**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `stations::stack_tooltip`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/enchantment-descriptions/mixins/enchantment_descriptions/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Enchantment Descriptions (enchdesc)** (see the upstream project; the descriptions are written for this mod); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
