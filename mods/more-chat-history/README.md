# More Chat History (Horizon native mod)

Increases the maximum length of the chat history, from the vanilla 100 messages to 16384.


Ported from **More Chat History (morechathistory) by JackFred** (CC0-1.0) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/more_chat_history/`). It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::push_message` (every message passes through it, a signed player message too) that does what the original does with a longer limit, including the line wrapping (Horizon's `add_lines` has a closure now, so its limit cannot be changed with a constant). It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `more_chat_history-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=more-chat-history), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → More Chat History. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/more_chat_history/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/more_chat_history/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_more_chat_history` and this crate's library |
| `mixins/more_chat_history/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/more-chat-history && cargo test          # the logic
python3 tools/mods.py check more-chat-history    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package more-chat-history  # target/more_chat_history-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-more-chat-history.yml` tests and packages the mod and publishes the release `more-chat-history-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
