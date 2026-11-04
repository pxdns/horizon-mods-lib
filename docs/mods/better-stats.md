# Better Stats

![Better Stats icon](../../assets/icons/catalogue/better-stats.png)

Search, rank and export your statistics from chat: the biggest entries of each kind, a search across all of them, and a text or CSV file.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/better-stats) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=better-stats)

## Use it

| Command | What it does |
|---|---|
| `/statistics top <kind> [n]` | The n biggest entries of a kind (`mined`, `crafted`, `used`, `broken`, `picked_up`, `dropped`, `killed`, `killed_by`, `custom`), default 10 |
| `/statistics find <words>` | Every entry whose name has all the words, across all kinds |
| `/statistics export` | Writes `statistics.txt` (and `statistics.csv` if switched on) to the mod's folder |
| `/statistics` | Lists these |

| Option | Default |
|---|---|
| Also write a CSV when exporting | off |

Switch it on or off in **Options → Mods → Better Stats**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_command`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/better-stats/mixins/better_stats/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Better Statistics Screen (betterstats)** (independent implementation from the mod's public description; no upstream code or assets); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
