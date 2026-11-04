# Password Hider

![Password Hider icon](../../assets/icons/horizon-mods-lib.svg)

Shows the password in /login, /register and /changepassword as asterisks while you type, and keeps it out of the chat history and its file.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/password-hider) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=password-hider)

## Use it

Nothing to press. Type `/login hunter2` or `/register hunter2 hunter2` and the password shows as `*******` in the chat box. Pressing the up arrow never brings it back, and it is not written to Horizon's saved command history. The commands covered are `/login`, `/l`, `/log`, `/register`, `/reg`, `/changepassword`, `/cp`, `/unregister` and `/unreg`.

| Option | Default |
|---|---|
| Mask the password while typing | on |
| Keep it out of the history and its file | on |

Switch it on or off in **Options → Mods → Password Hider**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `chat::ChatScreen::format`, `chat::Chat::add_recent`, `native_mods`, `native_settings`, `native_settings_click`, `native_tick`.
- Logic and tests: `mods/password-hider/mixins/password_hider/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **the password-hiding in login plugins' clients (an idea, not a port)** (independent implementation, no upstream code); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
