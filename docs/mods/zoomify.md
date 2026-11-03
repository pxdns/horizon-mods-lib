# Zoomify

![Zoomify icon](../../assets/icons/zoomify.svg)

Hold a key to zoom in, turn the wheel to zoom further, with smooth easing and slower turning.

**Version** 1.0.0 · **Game** 26.3 · **Hook-point API 1** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/zoomify) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=zoomify)

## Use it

| Key | Default | What it does |
|---|---|---|
| Zoom | `C` | Hold to zoom; the wheel changes it while it is on |

| Option | Default |
|---|---|
| Starting zoom (2, 3, 4, 6, 8, 12 or 16×) | 4× |
| Smooth zoom | on |
| Slower turning | on |
| Wheel zoom | on |

Switch it on or off in **Options → Mods → Zoomify**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_key`, `native_wheel`, `native_zoom`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/zoomify/mixins/zoomify/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`, `hooks: 1`.

## Credit

Ported from **Zoomify (zoomify)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
