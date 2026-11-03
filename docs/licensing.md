# Licensing

## Code

Everything in this repository (the mods, the tools, the site, the documentation) is MIT, see [`LICENSE`](https://github.com/pxdns/horizon-mods-lib/blob/main/LICENSE):
Copyright (c) 2026 pxdns and the horizon-mods-lib contributors. Each project folder carries its own copy, and the `.hnmod`
includes it as `mixins/<id>/LICENSE`, so the notice travels with the sources.

## Upstream mods and clean-room ports

The ports are written from behaviour, not from upstream source. Nobody here has copied or translated another mod's code, art,
text, or data files; the reference was what the mod does when you use it, and its public description. Several upstream mods are
LGPL, GPL or all-rights-reserved; none of that applies to a program that contains none of their code. A port can still share a
name or idea with its original, and the [credits](credits.md) say whose idea it was.

If you contribute a port: **do not paste upstream code into a pull request**, and do not translate it line by line. Describe the
behaviour, write the Rust, test it.

## Horizon and Minecraft

Horizon is MIT and independent of Mojang and Microsoft. These mods run in Horizon, which reads worlds and resources from a
Minecraft installation the player owns; **no Minecraft assets, code or names beyond plain descriptive references are included
here.** Minecraft is a trademark of Mojang Synergies AB. See [Trademarks](trademarks.md).

## Third-party material

| What | Licence |
|---|---|
| `tools/mods.py` (adapted from horizon-mods) | MIT, Ludovic Marie and the Horizon contributors |
| Project layout and `horizon-mixin.toml` shape (from the template) | MIT, levzzz5154 and the Horizon contributors |
| Python `markdown` (site build only, not shipped) | BSD-3-Clause |
| Upstream mod icons in `assets/icons/catalogue/` | Their authors' terms, not MIT; shown for identification, removed on request |

Bundled fonts: none. The Rust crates in the mod projects: none; every `Cargo.toml` has no dependencies.
