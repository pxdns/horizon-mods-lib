# Search Stats

![Search Stats icon](../../assets/icons/catalogue/searchstats.png)

Adds a search box to the statistics screen that filters the Items and Mobs lists as you type.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/search-stats) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=search-stats)

## Use it

Open Statistics and type: the Items and Mobs lists keep only the entries whose name contains every word you typed. Backspace deletes a letter, Delete clears the search, and it starts empty each time the screen opens. The search box sits left of the Done button.

| Option | Default |
|---|---|
| Show the search box | on |

Switch it on or off in **Options → Mods → Search Stats**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `stats_screen::stat_items`, `stats_screen::stat_mobs`, `stats_screen::draw_stats_footer`, `stats_screen::open_stats`, `native_key`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/search-stats/mixins/search_stats/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **SearchStats (searchstats)** (Apache-2.0); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
