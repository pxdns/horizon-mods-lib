# Chat Heads (Horizon native mod)

Shows the sender's face in front of each chat line.


Ported from **Chat Heads (chat_heads)** (MPL-2.0) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/chat_heads/`). It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::add_message` that runs after Chat Patches (priority 240) and adds Horizon's own inline player head (the text component `{"object":"player"}` Horizon already draws in the player list and in text) in front of the line, then calls `add_message` again. It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. The head is only as good as the skin Horizon has for that name: it uses the default skin until the player list has loaded the real one. The sender is found from the text, as the original does for servers that do not sign chat.

## Install

Download `chat_heads-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-heads), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Chat Heads. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/chat_heads/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/chat_heads/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_chat_heads` and this crate's library |
| `mixins/chat_heads/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/chat-heads && cargo test          # the logic
python3 tools/mods.py check chat-heads    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package chat-heads  # target/chat_heads-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-chat-heads.yml` tests and packages the mod and publishes the release `chat-heads-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
