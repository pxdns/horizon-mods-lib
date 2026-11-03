# Waypoints (Horizon native mod)

Name places with /wp, list them, and track one: its distance and direction show on screen.

- `/wp add <name>` names the place you stand, `/wp list` lists them, `/wp del <name>` removes one.
- `/wp track <name>` puts its distance and direction in the bottom-right corner; `/wp untrack` stops.

Ported from **Xaero's Minimap and World Map (waypoints idea)** (independent implementation, no upstream code) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/waypoints/`) that hooks only Horizon's native hook points, so the launcher builds it on any Horizon with
`NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `waypoints-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=waypoints), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Waypoints. The [install guide](https://github.com/pxdns/horizon-mods-lib/blob/main/docs/install.md) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/waypoints/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/waypoints/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_waypoints` and this crate's library |
| `mixins/waypoints/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/waypoints && cargo test          # the logic
python3 tools/mods.py check waypoints    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package waypoints  # target/waypoints-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-waypoints.yml` tests and packages the mod and publishes the release `waypoints-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
