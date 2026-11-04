# Shulker Box Tooltip

![Shulker Box Tooltip icon](../../assets/icons/catalogue/shulkerboxtooltip.png)

Shows what is inside a shulker box in its tooltip: totals by item, and a graphical 9 by 3 grid of the items.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/shulker-box-tooltip) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=shulker-box-tooltip)

## Use it

Nothing to press: hover a shulker box and its tooltip lists what is inside, totalled by item, with a 3 by 9 grid of the real item icons and counts under it.

| Option | Default |
|---|---|
| Totals by item | on |
| Slot grid with item icons | on |
| Other containers too | off |

Switch it on or off in **Options → Mods → Shulker Box Tooltip**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `stations::stack_tooltip`, `bundle::tooltip_layout`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/shulker-box-tooltip/mixins/shulker_box_tooltip/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Shulker Box Tooltip (shulkerboxtooltip)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
