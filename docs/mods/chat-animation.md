# Chat Animation

![Chat Animation icon](../../assets/icons/catalogue/chatanimation.png)

New chat lines slide up into place instead of appearing at once.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/chat-animation) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-animation)

## Use it

Nothing to press: new chat lines slide up into place. The settings page switches the slide on or off.

| Option | Default |
|---|---|
| Slide in | on |

Switch it on or off in **Options → Mods → Chat Animation**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `chat::Chat::layout`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/chat-animation/mixins/chat_animation/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Chat Animation [Smooth Chat] (chatanimation)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
