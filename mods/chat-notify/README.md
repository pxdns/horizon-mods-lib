# Chat Notify (Horizon native mod)

Plays a sound when someone says your name in chat.


Ported from **Chat Notify (chatnotify)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/chat_notify/`). It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`) and hooks `Chat::add_message` (before the original, so it also runs when another mod replaces it), then plays the sound from the game's tick, using `Game.audio` and `Options::player_name`. None of that is part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `chat_notify-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-notify), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Chat Notify. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/chat_notify/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/chat_notify/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_chat_notify` and this crate's library |
| `mixins/chat_notify/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/chat-notify && cargo test          # the logic
python3 tools/mods.py check chat-notify    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package chat-notify  # target/chat_notify-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-chat-notify.yml` tests and packages the mod and publishes the release `chat-notify-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
