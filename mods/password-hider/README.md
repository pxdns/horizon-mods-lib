# Password Hider (Horizon native mod)

Shows the password in /login, /register and /changepassword as asterisks while you type, and keeps it out of the chat history and its file.


Ported from **the password-hiding in login plugins' clients (an idea, not a port)** (independent implementation, no upstream code) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/password_hider/`). It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `ChatScreen::format`, the text formatter of the chat box, and one on `Chat::add_recent`, which stores a sent line in the history. Neither is part of the stable hook-point contract, so it is **pinned to one Horizon commit**. It only changes what is shown and saved on your computer: the server still receives the real command, as it must. `HORIZON_NET_DEBUG=1` logs every packet the game sends, passwords included; leave it off when you type one.

## Install

Download `password_hider-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=password-hider), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Password Hider. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/password_hider/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/password_hider/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_password_hider` and this crate's library |
| `mixins/password_hider/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/password-hider && cargo test          # the logic
python3 tools/mods.py check password-hider    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package password-hider  # target/password_hider-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-password-hider.yml` tests and packages the mod and publishes the release `password-hider-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
