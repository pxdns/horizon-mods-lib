# Releases

Every project releases itself.

1. Change the mod, and bump `version` in `mods/<slug>/mixins/<id>/horizon-mixin.toml` (and `Cargo.toml`).
2. Push to `main`.
3. `.github/workflows/release-<slug>.yml` calls the shared `_release.yml`, which runs `cargo fmt --check`, clippy with warnings
   denied and `cargo test`, packages the `.hnmod`, uploads it as a workflow artifact and, **if the tag `<slug>-v<version>` does
   not exist yet**, creates that GitHub release with the `.hnmod` and a `.sha256` file.

A push that does not change a mod's folder does not trigger its workflow, and a push that does not change the version
releases nothing. To test and package without releasing, run a workflow by hand from the Actions tab and leave *publish* off.

`ci.yml` runs on every push and pull request: each mod's tests in a matrix, and a job that applies **all** mods on the pinned
Horizon and type-checks the game, so a hook that no longer matches Horizon fails there and not on a player's machine.

## Versioning

Semantic versions, per mod. A change that needs a newer hook-point API also bumps `hooks` in `compatibility.json`. A new Horizon
commit is tested by changing `horizon_revision` (in `compatibility.json` and `horizon-mixin.toml`) and letting CI apply the set.
