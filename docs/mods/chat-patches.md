# Chat Patches

![Chat Patches icon](../../assets/icons/catalogue/chatpatches.png)

Timestamps in front of chat lines, and an option to drop immediate repeats.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/chat-patches) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-patches)

## Use it

Nothing to press: every chat line starts with a gray `[14:32]` (your local time). The settings page adds seconds, and can drop a message that repeats the previous one within five seconds.

| Option | Default |
|---|---|
| Timestamps | on |
| Seconds in the timestamp | off |
| Drop immediate repeats | off |

Switch it on or off in **Options → Mods → Chat Patches**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `chat::Chat::add_message`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/chat-patches/mixins/chat_patches/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Chat Patches (chatpatches)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
