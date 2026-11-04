# Chat Heads

![Chat Heads icon](../../assets/icons/catalogue/chat-heads.png)

Shows the sender's face in front of each chat line.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/chat-heads) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-heads)

## Use it

Nothing to press: a chat line from a player starts with that player's face, the way a skin shows in the player list. It finds the sender in the usual `<Name> message` form, and in `Name: message` or `Name » message` forms for players who are online. The settings page switches the hat layer and the other forms off.

| Option | Default |
|---|---|
| Show the hat layer | on |
| Also find senders in Name: and Name » forms | on |

Switch it on or off in **Options → Mods → Chat Heads**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `chat::Chat::add_message`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/chat-heads/mixins/chat_heads/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Chat Heads (chat_heads)** (MPL-2.0); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
