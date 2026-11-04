#!/usr/bin/env python3
"""Regenerates everything that is generated, from this repository alone:

    python3 tools/gen/regen.py

1. downloads the pinned Horizon source (tools/mods.py fetch; GitHub's archive of the commit, never a local checkout),
2. writes mods/<slug>/ and the release workflows (gen.py),
3. writes docs/*.md and docs/mods/*.md (docs.py),
4. writes MODS.md, MODULES.md, catalogue/*.md and site/data/mods.json (catalog.py, from the snapshots in inputs/),
5. formats each mod and writes its Cargo.lock,
6. builds the site into public/ (site/build.py; needs `pip install markdown`).

Hand-written files (README.md, tools/mods.py, site/, vercel.json, the other workflows) are never touched. Run it twice and
`git status` stays clean: that is the check that nothing comes from outside the repository.
Refreshing the Modrinth icons is separate and needs the network on purpose: python3 tools/gen/icons.py
"""
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def run(*args, cwd=ROOT):
    print("+", " ".join(str(a) for a in args), flush=True)
    subprocess.run([str(a) for a in args], cwd=cwd, check=True)


def main():
    py = sys.executable
    run(py, ROOT / "tools/mods.py", "fetch")
    run(py, HERE / "gen.py")
    run(py, HERE / "docs.py")
    run(py, HERE / "catalog.py")
    for mod in sorted((ROOT / "mods").iterdir()):
        if (mod / "Cargo.toml").is_file():
            run("cargo", "generate-lockfile", "--offline", "-q", cwd=mod)
            run("cargo", "fmt", cwd=mod)
    run(py, ROOT / "site/build.py")


main()
