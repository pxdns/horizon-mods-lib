# Combat Hitboxes (Horizon native mod)

Makes entities slightly easier to hit by growing the box your crosshair is tested against.


Ported from **Combat Hitboxes (combat-hitbox)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/combat_hitboxes/`). It puts a cancellable HEAD hook on `ray_box`, the ray-versus-box test in Horizon's `game.rs`, which runs the same test again on a larger box. That function is not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. It grows the boxes of everything the crosshair is tested against, and only changes where you aim: it does not change who the server says you hit.

## Install

Download `combat_hitboxes-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=combat-hitboxes), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Combat Hitboxes. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/combat_hitboxes/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/combat_hitboxes/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_combat_hitboxes` and this crate's library |
| `mixins/combat_hitboxes/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/combat-hitboxes && cargo test          # the logic
python3 tools/mods.py check combat-hitboxes    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package combat-hitboxes  # target/combat_hitboxes-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-combat-hitboxes.yml` tests and packages the mod and publishes the release `combat-hitboxes-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
