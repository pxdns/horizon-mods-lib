# F5 Split

![F5 Split icon](../../assets/icons/horizon-mods-lib.svg)

Separate keys for the back and the front third-person view, as a toggle or while held.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/f5-split) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=f5-split)

## Use it

| Key | Default | What it does |
|---|---|---|
| Back view | `F6` | Press for the view from behind, press again to return to first person |
| Front view | `F7` | The same for the view facing you |

With the setting "Hold the key instead of toggling" on, the view lasts only while the key is held. F5 keeps cycling as before.

| Option | Default |
|---|---|
| Hold the key instead of toggling | off |

Switch it on or off in **Options → Mods → F5 Split**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_key`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/f5-split/mixins/f5_split/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **F5 Split (f5split)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
