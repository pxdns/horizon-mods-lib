# Build from source

You need Rust, Python 3.12+ and Git. The mods' own logic builds and tests with plain `cargo`; checking them against Horizon
needs Horizon's source and a few system libraries.

## Test a mod's logic

```sh
cd mods/zoomify
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

The Cargo library of each project **is** `mixins/<id>/runtime.rs`, the same file that horizon-mixins merges into the game, so
what you test is what ships.

## Apply and type-check on Horizon

`tools/mods.py` is adapted from the registry's own tool (`horizon-rust/horizon-mods`, `native/tools/native.py`, MIT):

```sh
python3 tools/mods.py list                     # every project
python3 tools/mods.py check zoomify            # apply on the pinned Horizon, cargo check the game
python3 tools/mods.py check all                # every mod together, as a player's set is
python3 tools/mods.py package zoomify          # target/zoomify-1.0.0.hnmod and its SHA-256
```

`check` downloads Horizon's source at the commit in `compatibility.json` (GitHub's archive, never a checkout), builds
horizon-mixins at its pinned commit, applies the mods and runs `cargo check` on the modded game with warnings denied. On Ubuntu
the game's crates need `libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev pkg-config`.

## Pins

`compatibility.json` in each project names the Horizon commit it was tested on and the horizon-mixins commit it was applied
with, and `"hooks": 1`: the hook-point API version it needs. See [Compatibility](compatibility.md).
