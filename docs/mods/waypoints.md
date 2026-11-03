# Waypoints

![Waypoints icon](../../assets/icons/waypoints.svg)

Name places with /wp, list them, and track one: its distance and direction show on screen.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API** 1 · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/waypoints) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=waypoints)

## Use it

| Command | What it does |
|---|---|
| `/wp add <name>` | Names the block you stand on |
| `/wp list` | Lists them, with distances in your dimension |
| `/wp del <name>` | Deletes one |
| `/wp track <name>` | Shows its distance and direction on screen |
| `/wp untrack` | Stops tracking |

| Option | Default |
|---|---|
| Show tracked waypoint | on |

Switch it on or off in **Options → Mods → Waypoints**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/waypoints/mixins/waypoints/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **Xaero's Minimap and World Map (waypoints idea)** (independent implementation, no upstream code); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
