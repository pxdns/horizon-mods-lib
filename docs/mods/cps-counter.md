# CPS Counter

![CPS Counter icon](../../assets/icons/cps-counter.svg)

Your clicks per second in a corner of the screen.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API** 1 · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/cps-counter) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=cps-counter)

## Use it

| Command | What it does |
|---|---|
| `/cps` | Hides or shows CPS Counter |

| Option | Default |
|---|---|
| Shown | on |
| Right clicks | off |

Switch it on or off in **Options → Mods → CPS Counter**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/cps-counter/mixins/cps_counter/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **Tempest CPS module (own work)** (MIT); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
