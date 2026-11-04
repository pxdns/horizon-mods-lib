# The generators

Everything under `mods/`, the release workflows (`.github/workflows/release-*.yml`, `_release.yml`, `ci.yml`), `docs/`, `MODS.md`, `MODULES.md`,
`catalogue/` and `site/data/mods.json` is **generated** from the files in this folder. Edit the generator, not the output.

| File | What it holds |
|---|---|
| `mods*.py` | Each mod's logic, tests, hook helpers and text: `mods.py` (stable hook points), `mods_raw.py` (game state), `mods_cam.py` (camera and input), `mods_chat.py` (chat, tooltips), `mods_draw.py` (crosshair), `mods_hit.py` |
| `common.py` | Shared Rust and TOML text, the release and CI workflows |
| `gen.py` | Writes `mods/<slug>/` for every mod above |
| `docs.py` | Writes the documentation pages |
| `catalog.py` | Writes the catalogue from `inputs/` |
| `icons.py` | Refreshes `assets/icons/catalogue/` from Modrinth (network; run by hand) |
| `inputs/` | Snapshots of the Tempest project's mod matrix, licence notes, module list and metadata: the only outside data, committed here |
| `regen.py` | Runs all of it, in order |

```sh
pip install markdown        # for the site build
python3 tools/gen/regen.py  # downloads the pinned Horizon source, regenerates, formats, rebuilds public/
git status                  # clean: the repository reproduces itself
```

Nothing reads a path outside the repository. The pinned Horizon source is downloaded by `tools/mods.py fetch`, and the generator that rebuilds Horizon's chat layout
(`chat-animation`) reads it from `target/horizon/<commit>/` (or from `HORIZON_SRC`).
