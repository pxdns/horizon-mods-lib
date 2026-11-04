# Multi Key Bindings (Horizon native mod)

More than one key for the same action.


Ported from **Multi Key Bindings (multi-key-bindings)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/multi_key_bindings/`). It lives in the input crate's `keybinds.rs`: cancellable HEAD hooks on `KeyBinds::translate_all` and `translate_mouse_all`, the functions that turn a pressed key or button into the default key of the action it is bound to, rebuilt with the extra keys added. They are not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. Extra keys are made with a command, not buttons on the Key Binds page (Horizon draws it in a layout function that cannot be extended), and modifier combinations such as Shift+K are not supported. It was written from the original's public description, not its source, which is GPL-3.0.

## Install

Download `multi_key_bindings-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=multi-key-bindings), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Multi Key Bindings. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/multi_key_bindings/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/multi_key_bindings/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_multi_key_bindings` and this crate's library |
| `mixins/multi_key_bindings/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/multi-key-bindings && cargo test          # the logic
python3 tools/mods.py check multi-key-bindings    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package multi-key-bindings  # target/multi_key_bindings-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-multi-key-bindings.yml` tests and packages the mod and publishes the release `multi-key-bindings-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
