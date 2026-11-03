# Sprint Indicator

![Sprint Indicator icon](../../assets/icons/sprint-indicator.svg)

Shows Sprinting, Walking or Sneaking in a corner of the screen while you move.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API** 1 · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/sprint-indicator) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=sprint-indicator)

## Use it

| Command | What it does |
|---|---|
| `/sprint` | Hides or shows Sprint Indicator |

| Option | Default |
|---|---|
| Shown | on |
| Show walking | on |
| Show standing still | off |

Switch it on or off in **Options → Mods → Sprint Indicator**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/sprint-indicator/mixins/sprint_indicator/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **Sprint Indicator (sprintindicator)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
