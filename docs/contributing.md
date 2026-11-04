# Contributing

Ports are welcome. A good one is small, tested, and honest about what the hook points let it do.

1. **Pick a mod that fits the hook points.** The [catalogue](../MODS.md) marks what needs a hook point and what does not; if a mod
   needs one, the useful contribution is a pull request to [Horizon](https://github.com/horizon-rust/horizon) that adds the hook point.
2. **Copy a project folder** under `mods/` and rename it (`slug` for the folder, `snake_case` for the id). Keep `runtime.rs` pure,
   put the tests in it, keep each helper a few lines.
3. **Write it from behaviour.** Do not copy or translate upstream code. See [Licensing](licensing.md).
4. **Run** `cargo fmt`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, and
   `python3 tools/mods.py check <slug>`.
5. **Add** a `release-<slug>.yml` (copy an existing one and change the slug), a row in `docs/credits.md`, and an icon in `assets/icons/`.
6. Open the pull request. CI applies all mods on the pinned Horizon.

Everything under `mods/`, `docs/`, `catalogue/` and the release workflows is generated from `tools/gen/`: edit the generator and run `python3 tools/gen/regen.py` (it downloads the pinned Horizon source itself, and a second run changes nothing).
See `tools/gen/README.md`.

Be accurate in README claims: say what the mod does and what it does not, and name the upstream mod and its licence.
