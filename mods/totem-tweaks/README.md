# Totem Tweaks (Horizon native mod)

Customise the totem of undying pop animation: off, speed, size, spin, rocking and position.


Ported from **Totem Tweaks (totem-tweaks)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/totem_tweaks/`). It replaces `item_activation::animation_pose`, the function that poses the totem for each frame of the pop, with a cancellable HEAD hook that computes the same curve with the player's choices. It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. Left out from the original: the handheld totem's size and the equip animation, which need hooks in the hand drawing. I could not look at the animation on screen; with every setting at its default the pose is the same curve as Horizon's, and the logic is tested.

## Install

Download `totem_tweaks-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=totem-tweaks), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Totem Tweaks. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/totem_tweaks/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/totem_tweaks/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_totem_tweaks` and this crate's library |
| `mixins/totem_tweaks/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/totem-tweaks && cargo test          # the logic
python3 tools/mods.py check totem-tweaks    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package totem-tweaks  # target/totem_tweaks-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-totem-tweaks.yml` tests and packages the mod and publishes the release `totem-tweaks-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
