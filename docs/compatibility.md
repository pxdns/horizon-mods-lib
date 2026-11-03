# Compatibility

| | |
|---|---|
| Game | Minecraft Java 26.3, as implemented by Horizon |
| Horizon (tested on) | `ba6344a59cb9e755b730dcf02285fc0d33b97d3c` (Nightly 2026-10-30) |
| horizon-mixins | `d9c83d7096b7ea5b83678da2efdfd6161f74997e` (0.6.0, plan schema 6) |
| Hook points | `hooks: 1`: builds on any Horizon whose `NATIVE_HOOKS_API` is 1 or later |
| Rust | Whatever the launcher brings (it downloads its own toolchain); CI uses stable |

The nine mods that use only the stable hook points are described below; the six that read game state (see [Hook points](hook-points.md)) are pinned to the exact commit and have no `hooks` entry.

Because the first nine hook only the stable hook points, the Horizon commit is the one they were *tested on*, not a requirement:
the launcher builds them on the commit the other mods in the build pin, else on the commit of the installed version. A hook
point that is removed or changed incompatibly would break them, and Horizon's contract is that it does not do that.

Native mods are not built for dedicated servers yet; on a server you join, only client-side hooks do anything and the protocol is
unchanged. In single player and on a LAN host the mods run in the host's game.
