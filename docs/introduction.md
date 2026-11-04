# Introduction

**horizon-mods-lib** is a library of native mods for [Horizon](https://github.com/horizon-rust/horizon), the free,
open-source rewrite of Minecraft: Java Edition 26.3 in Rust. Every mod is its own small project with its own tests, its own
release and its own `.hnmod` file. They are ports: each one re-implements something people already use in Fabric
(a FPS readout, a zoom, a key overlay, waypoints) on Horizon's native hook points.

## What a native mod is

A Fabric mod is Java that gets loaded into the JVM. A Horizon native mod is Rust **source** that
[horizon-mixins](https://github.com/horizon-rust/horizon-mixins) merges into the game's own source before it is compiled. The
launcher builds a game for the instance the mod is installed in, so the mod is not a plugin loaded at run time: it is part of
the executable you play.

That has consequences, and this library is built around them:

- A mod can only do what the game's source lets it hook. Horizon keeps a short list of **stable hook points**
  (see [Hook points](hook-points.md)); mods written against them build on any Horizon whose hook-point version is new enough.
- A native mod runs with the player's rights, so the registry reads the source of every native mod before it lists it. These
  projects are small on purpose: a mod's logic is one `runtime.rs` file with unit tests, and each hook is a helper of a few lines.
- Everything is MIT, nothing is minified, and the `.hnmod` contains exactly the `.rs` and `.toml` files you can read in the repository.

## What is here

| | |
|---|---|
| [Mods](mods/fps-display.md) | 50 projects in `mods/`, one folder each |
| [Catalogue](../MODS.md) | The 186 mods in the Tempest cross-pack matrix and where each stands on Horizon |
| [Modules](../MODULES.md) | Tempest's own module list, and an overview of the Horizon organisation |
| [Credits](credits.md), [Licensing](licensing.md), [DMCA](dmca.md) | Who made what, under which terms, and how to ask for something to come down |

## Where to go next

1. [Install a mod](install.md), or all of them in one go, if you want to play with them.
2. [Build from source](build.md) if you want to change one.
3. [Anatomy of a mod](anatomy.md) and [Hook points](hook-points.md) if you want to write the next one.
