# More Chat History

![More Chat History icon](../../assets/icons/more-chat-history.svg)

Chat remembers up to 2000 lines instead of 100, so you can scroll back through a long conversation.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/more-chat-history) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=more-chat-history)

## Use it

Nothing to press: chat keeps more of what was said and you scroll back through it. The first button picks how many lines (100, 250, 500, 1000 or 2000).



Switch it on or off in **Options → Mods → More Chat History**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `chat::Chat::add_message`, `chat::Chat::add_lines`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/more-chat-history/mixins/more_chat_history/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **More Chat History (morechathistory)** (see the upstream project); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
