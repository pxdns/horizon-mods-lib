# Pick Up Notifier

![Pick Up Notifier icon](../../assets/icons/catalogue/pick-up-notifier.png)

+3 Cobblestone: a short list of what you just picked up (and, if you like, dropped).

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/pick-up-notifier) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=pick-up-notifier)

## Use it

| Command | What it does |
|---|---|
| `/pickups` | Hides or shows Pick Up Notifier |

| Option | Default |
|---|---|
| Shown | on |
| Show losses | off |

Switch it on or off in **Options → Mods → Pick Up Notifier**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_tick`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/pick-up-notifier/mixins/pick_up_notifier/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Pick Up Notifier (pickupnotifier)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
