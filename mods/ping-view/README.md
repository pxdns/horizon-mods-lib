# Ping View (Horizon native mod)

Displays your and other players ping in the tab list, with color.


Ported from **Ping View (pingview) by karlssonsmp** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/ping_view/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `ping_view-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=ping-view), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Ping View. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/ping_view/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/ping_view/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_ping_view` and this crate's library |
| `mixins/ping_view/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/ping-view && cargo test          # the logic
python3 tools/mods.py check ping-view    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package ping-view  # target/ping_view-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-ping-view.yml` tests and packages the mod and publishes the release `ping-view-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
