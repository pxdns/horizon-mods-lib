# Chat Notify

![Chat Notify icon](../../assets/icons/chat-notify.svg)

Plays a sound when someone says your name in chat.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/chat-notify) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-notify)

## Use it

Nothing to press: a short sound plays when a message mentions your name. The settings page picks which messages count (players, the server) and whether your own count.

| Option | Default |
|---|---|
| Player messages | on |
| Server messages | off |
| My own messages | off |

Switch it on or off in **Options → Mods → Chat Notify**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `chat::Chat::add_message`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/chat-notify/mixins/chat_notify/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Chat Notify (chatnotify)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
