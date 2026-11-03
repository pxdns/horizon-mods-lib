# Key Overlay

![Key Overlay icon](../../assets/icons/key-overlay.svg)

Shows W, A, S, D, jump and the mouse buttons on screen and lights the ones you hold.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API 1** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/key-overlay) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=key-overlay)

## Use it

| Command | What it does |
|---|---|
| `/keys` | Hides or shows Key Overlay |

| Option | Default |
|---|---|
| Shown | on |
| Mouse buttons | on |
| Jump bar | on |

Switch it on or off in **Options → Mods → Key Overlay**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_hud`, `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/key-overlay/mixins/key_overlay/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **Key Overlay (KeyOverlay)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
