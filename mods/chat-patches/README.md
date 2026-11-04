# Chat Patches (Horizon native mod)

Timestamps in front of chat lines, and an option to drop immediate repeats.


Ported from **Chat Patches (chatpatches)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/chat_patches/`). It lives in Horizon's chat module (`crates/horizon-client-render/src/chat.rs`): a cancellable HEAD hook on `Chat::add_message` that runs before the other chat mods' (priority 250), adds the stamp and calls `add_message` again, so Chat Notify and More Chat History see each line once. It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. It covers timestamps and repeats only; the original's other features (copying, hiding, compact counters) are not included.

## Install

Download `chat_patches-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=chat-patches), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Chat Patches. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/chat_patches/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/chat_patches/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_chat_patches` and this crate's library |
| `mixins/chat_patches/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/chat-patches && cargo test          # the logic
python3 tools/mods.py check chat-patches    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package chat-patches  # target/chat_patches-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-chat-patches.yml` tests and packages the mod and publishes the release `chat-patches-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
