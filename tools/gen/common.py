"""Shared Rust/TOML/README/workflow text for the horizon-mods-lib generator."""

HORIZON_REV = "ba6344a59cb9e755b730dcf02285fc0d33b97d3c"
MIXINS_REV = "d9c83d7096b7ea5b83678da2efdfd6161f74997e"
GAME = "26.3"

OPTIONS_RS = r'''
use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
@OPTIONS@
];

thread_local! {
    /// Read once through the game's settings files, then kept.
    static CHOSEN: RefCell<Option<Vec<bool>>> = const { RefCell::new(None) };
}

/// A stored value: `true` or `false`, else the default.
pub fn parse(stored: Option<String>, default: bool) -> bool {
    stored.map_or(default, |v| match v.trim() {
        "true" => true,
        "false" => false,
        _ => default,
    })
}

fn chosen(get: impl Fn(&str) -> Option<String>) -> Vec<bool> {
    CHOSEN.with_borrow_mut(|c| {
        c.get_or_insert_with(|| {
            OPTIONS
                .iter()
                .map(|(key, _, default)| parse(get(key), *default))
                .collect()
        })
        .clone()
    })
}

/// Whether option `key` is on.
pub fn is(get: impl Fn(&str) -> Option<String>, key: &str) -> bool {
    OPTIONS
        .iter()
        .position(|o| o.0 == key)
        .is_some_and(|i| chosen(get)[i])
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let on = chosen(get);
    OPTIONS
        .iter()
        .zip(on)
        .map(|((_, label, _), on)| format!("{label}: {}", if on { "ON" } else { "OFF" }))
        .collect()
}

/// A click on button `i`: flips it and returns what to store (key, value).
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut on = chosen(get);
    let (key, _, _) = *OPTIONS.get(i)?;
    on[i] = !on[i];
    let value = on[i].to_string();
    CHOSEN.with_borrow_mut(|c| *c = Some(on));
    Some((key, value))
}

/// Forgets what was read (tests only).
#[cfg(test)]
pub fn forget_options() {
    CHOSEN.with_borrow_mut(|c| *c = None);
}
'''

OPTIONS_TESTS_RS = r'''
    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(
            labels(none).len(),
            OPTIONS.len(),
            "one button per option"
        );
        assert_eq!(is(none, OPTIONS[0].0), OPTIONS[0].2);
        forget_options();
        let stored = |k: &str| (k == OPTIONS[0].0).then(|| "false".to_owned());
        assert!(!is(stored, OPTIONS[0].0));
        forget_options();
        assert!(parse(Some("junk".into()), true));
        assert!(!parse(Some(" false\n".into()), true));
    }

    #[test]
    fn a_click_flips_and_reports_what_to_store() {
        forget_options();
        let none = |_: &str| None;
        let (key, value) = click(none, 0).expect("button 0 exists");
        assert_eq!(key, OPTIONS[0].0);
        assert_eq!(value, (!OPTIONS[0].2).to_string());
        assert_eq!(is(none, key), !OPTIONS[0].2);
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
'''

# --- helper files -------------------------------------------------------------------------------

LIST_RS = '''// The mod's row in Options > Mods.
fn @ID@_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::@MOD@ as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: @KEYS@,
    });
}
'''

SETTINGS_RS = '''// The mod's settings page: one button per option.
fn @ID@_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::@MOD@ as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::labels(|k| game.native_setting(m::ID, k)));
}
'''

CLICK_RS = '''// A click on the settings page: flip the option and keep it.
fn @ID@_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::@MOD@ as m;
    if mod_id != m::ID {
        return;
    }
    let stored = {
        let game = &*game;
        m::click(|k| game.native_setting(m::ID, k), button)
    };
    if let Some((key, value)) = stored {
        game.native_set_setting(m::ID, key, &value);
    }
}
'''

HUD_TARGET = "horizon_client::game::native_hooks::Game<'_>::native_hud"
HOOK_TARGETS = {
    "hud": ("native_hud", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("batch", "&mut horizon_client::hud::HudBatch", "&mut *batch"),
        ("corners", "&mut horizon_client::extras_hud::Corners", "&mut *corners"),
    ]),
    "command": ("native_command", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("command", "&str", "command"),
        ("ran", "&mut bool", "&mut *ran"),
    ]),
    "list": ("native_mods", [
        ("mods", "&mut Vec<crate::game::native_hooks::NativeMod>", "&mut *mods"),
    ]),
    "settings": ("native_settings", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("mod_id", "&str", "mod_id"),
        ("buttons", "&mut Vec<String>", "&mut *buttons"),
    ]),
    "click": ("native_settings_click", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("mod_id", "&str", "mod_id"),
        ("button", "usize", "button"),
    ]),
    "key": ("native_key", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("key", "&str", "key"),
        ("pressed", "bool", "pressed"),
    ]),
    "wheel": ("native_wheel", [
        ("game", "&crate::game::Game<'_>", "&*self"),
        ("steps", "f32", "steps"),
        ("claimed", "&mut bool", "&mut *claimed"),
    ]),
    "zoom": ("native_zoom", [
        ("game", "&crate::game::Game<'_>", "&*self"),
        ("zoom", "&mut Option<crate::game::native_hooks::NativeZoom>", "&mut *zoom"),
    ]),
    "minimap": ("native_minimap", [
        ("game", "&crate::game::Game<'_>", "&*self"),
        ("minimap", "&mut Option<crate::game::native_hooks::NativeMinimap>", "&mut *minimap"),
    ]),
    "tick": ("native_tick", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
    ]),
    "frame": ("native_frame", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("elapsed", "std::time::Duration", "elapsed"),
    ]),
    # Raw targets inside Horizon: full function ids, cancellable HEAD.
    "turn": ("horizon_client::game::perspective::Game<'_>::turn_player", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("xo", "f64", "xo"),
        ("yo", "f64", "yo"),
    ], True),
    "camera": ("horizon_client::game::perspective::Game<'_>::camera_view", [
        ("game", "&mut crate::game::Game<'_>", "&mut *self"),
        ("partial", "f32", "partial"),
    ], True),
    "bob": ("horizon_client::game::fov::Game<'_>::bob_matrix", [
        ("game", "&crate::game::Game<'_>", "&*self"),
        ("partial", "f32", "partial"),
    ], True),
}


def toml(mod, hooks, name, version):
    out = f'''[mod]
id = "{mod}"
version = "{version}"
priority = 100

# The commit it was tested on. It uses only the native hook points (hooks 1 in
# compatibility.json): the launcher builds it on any Horizon that has them.
[compatibility]
horizon_revision = "{HORIZON_REV}"

[[module]]
id = "runtime"
file = "crates/horizon-client/src/game.rs"
name = "{name}"
source = "runtime.rs"
'''
    for h in hooks:
        spec = HOOK_TARGETS[h]
        fn, bindings = spec[0], spec[1]
        target = fn if "::" in fn else f"horizon_client::game::native_hooks::Game<'_>::{fn}"
        cancel = "cancellable = true\n" if len(spec) > 2 and spec[2] else ""
        b = ",\n".join(f'  {{ name = "{n}", ty = "{t}", value = "{v}" }}' for n, t, v in bindings)
        out += f'''
[[injection]]
id = "{h}"
kind = "HEAD"
{cancel}target = "{target}"
helper = "{h}.rs"
bindings = [
{b},
]
'''
    return out


def cargo_toml(slug, version, description):
    return f'''[package]
name = "horizon-mod-{slug}"
version = "{version}"
edition = "2024"
license = "MIT"
publish = false
description = "{description}"

# The mod's logic and its tests: the same file the bundle adds to the game.
[lib]
path = "mixins/{{ID}}/runtime.rs"

[workspace]
'''


def compat_json(raw=False, api=1):
    hooks = "" if raw else f'  "hooks": {api},\n'
    return f'''{{
  "game": "{GAME}",
{hooks}  "horizon_repository": "https://github.com/horizon-rust/horizon",
  "horizon_revision": "{HORIZON_REV}",
  "horizon_mixins_repository": "https://github.com/horizon-rust/horizon-mixins",
  "horizon_mixins_revision": "{MIXINS_REV}"
}}
'''


LICENSE = '''MIT License

Copyright (c) 2026 pxdns and the horizon-mods-lib contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
'''

RELEASE_YML = '''# Releases the @NAME@ native mod (@SLUG@/) as @ID@-<version>.hnmod.
#
# Bump `version` in mods/@SLUG@/mixins/@ID@/horizon-mixin.toml and push to main: the mod is tested, packaged, and a
# GitHub release `@SLUG@-v<version>` is created with the .hnmod and its SHA-256. Nothing is released when that tag already
# exists, so ordinary pushes are free. Run it by hand from the Actions tab to test and package without releasing.
name: release @SLUG@

on:
  push:
    branches: [main]
    paths:
      - "mods/@SLUG@/**"
      - "tools/mods.py"
      - ".github/workflows/_release.yml"
      - ".github/workflows/release-@SLUG@.yml"
  workflow_dispatch:
    inputs:
      publish:
        description: "Create the release even from a manual run"
        type: boolean
        default: false

permissions:
  contents: write

jobs:
  release:
    uses: ./.github/workflows/_release.yml
    with:
      slug: @SLUG@
      publish: ${{ github.event_name == 'push' || inputs.publish }}
'''

REUSABLE_YML = '''# Shared by every release-<mod>.yml: test the mod, package its .hnmod, release it when its version is new.
name: release a mod

on:
  workflow_call:
    inputs:
      slug:
        description: "Folder of the mod under mods/"
        required: true
        type: string
      publish:
        description: "Create the GitHub release when the tag is new"
        required: false
        type: boolean
        default: true

permissions:
  contents: write

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"

      - name: Install Rust
        run: rustup toolchain install stable --profile minimal --component rustfmt --component clippy

      - name: Format, lint and test the mod's logic
        working-directory: mods/${{ inputs.slug }}
        run: |
          cargo fmt --check
          cargo clippy --all-targets --locked -- -D warnings
          cargo test --locked

      - name: Package the .hnmod
        id: package
        run: |
          python3 tools/mods.py package "${{ inputs.slug }}" | tee package.json
          echo "file=$(python3 -c 'import json; print(json.load(open("package.json"))["file"])')" >> "$GITHUB_OUTPUT"
          echo "sha=$(python3 -c 'import json; print(json.load(open("package.json"))["sha256"])')" >> "$GITHUB_OUTPUT"
          echo "version=$(python3 tools/mods.py version "${{ inputs.slug }}")" >> "$GITHUB_OUTPUT"

      - name: Upload the .hnmod as a workflow artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ inputs.slug }}-${{ steps.package.outputs.version }}
          path: ${{ steps.package.outputs.file }}

      - name: Release when this version is new
        if: ${{ inputs.publish }}
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          tag="${{ inputs.slug }}-v${{ steps.package.outputs.version }}"
          if gh release view "$tag" >/dev/null 2>&1; then
            echo "$tag is already released; bump the version to release again."
            exit 0
          fi
          printf '%s\\n' "${{ steps.package.outputs.sha }}  $(basename '${{ steps.package.outputs.file }}')" > "${{ steps.package.outputs.file }}.sha256"
          gh release create "$tag" "${{ steps.package.outputs.file }}" "${{ steps.package.outputs.file }}.sha256" \\
            --title "${{ inputs.slug }} ${{ steps.package.outputs.version }}" \\
            --target "${{ github.sha }}" \\
            --notes "Native Horizon mod \\`${{ inputs.slug }}\\` ${{ steps.package.outputs.version }}, built for Horizon ${{ '@GAME@' }} on the hook points (hooks 1). Unzip it and build it into a game with horizon-launcher native-build; SHA-256 \\`${{ steps.package.outputs.sha }}\\`."
'''

CI_YML = '''name: ci

on:
  push:
    branches: [main, "claude/**"]
  pull_request:

permissions:
  contents: read

jobs:
  logic:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        mod: [@MODS@]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install Rust
        run: rustup toolchain install stable --profile minimal --component rustfmt --component clippy
      - name: Format, lint and test the mod's logic
        working-directory: mods/${{ matrix.mod }}
        run: |
          cargo fmt --check
          cargo clippy --all-targets --locked -- -D warnings
          cargo test --locked
      - name: The bundle packages
        run: python3 tools/mods.py package "${{ matrix.mod }}"

  on-horizon:
    # Applies every mod together on the pinned Horizon and type-checks the game, as the launcher builds a player's set.
    runs-on: ubuntu-latest
    needs: logic
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install Rust
        run: rustup toolchain install stable --profile minimal
      - name: System libraries the game's crates link
        run: sudo apt-get update && sudo apt-get install -y --no-install-recommends libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev pkg-config
      - name: Apply all mods on Horizon and cargo check
        run: python3 tools/mods.py check all
      - name: The all-in-one bundle builds
        run: python3 tools/mods.py bundle all

  reproducible:
    # Regenerates every generated file from this repository alone and fails if anything differs.
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install Rust and the site builder's one dependency
        run: |
          rustup toolchain install stable --profile minimal --component rustfmt
          pip install markdown
      - run: python3 tools/gen/regen.py
      - name: Nothing changed
        run: |
          if ! git diff --exit-code --stat; then
            echo "::error::Generated files are out of date, run python3 tools/gen/regen.py and commit"
            exit 1
          fi
'''
