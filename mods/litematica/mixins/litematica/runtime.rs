// Litematica, a native Horizon mod: Read .litematic schematics: info, and the material list of blocks you need.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_litematica`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "litematica";
pub const NAME: &str = "Litematica";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Read .litematic schematics: info, and the material list of blocks you need.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[("stacks", "Show stacks next to the counts", true)];

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

use std::collections::BTreeMap;

/// Slash command.
pub const COMMAND: &str = "schematic";
pub const HELP: [&str; 4] = [
    "/schematic list: the .litematic files in the schematics folder",
    "/schematic info <name>: what is in one",
    "/schematic materials <name> [n]: the n most-needed blocks",
    "/schematic export <name>: write the material list to a file",
];

/// One region of a schematic: its size, the block names of its palette and its packed block indices.
#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub name: String,
    pub size: [i64; 3],
    pub palette: Vec<String>,
    pub states: Vec<i64>,
}

/// What a `.litematic` file holds.
#[derive(Clone, Debug, PartialEq)]
pub struct Schematic {
    pub name: String,
    pub author: String,
    pub description: String,
    pub regions: Vec<Region>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Help,
    List,
    Info(String),
    Materials(String, usize),
    Export(String),
}

pub fn request(args: &[&str]) -> Request {
    match args {
        ["list"] => Request::List,
        ["info", name] => Request::Info((*name).to_owned()),
        ["materials", name] => Request::Materials((*name).to_owned(), 20),
        ["materials", name, n] => {
            Request::Materials((*name).to_owned(), n.parse().unwrap_or(20).clamp(1, 500))
        }
        ["export", name] => Request::Export((*name).to_owned()),
        _ => Request::Help,
    }
}

/// The file for a typed name: exact, or without the extension, ignoring case.
pub fn find_file(typed: &str, files: &[String]) -> Option<String> {
    let t = typed.to_lowercase();
    files
        .iter()
        .find(|f| f.to_lowercase() == t || f.to_lowercase() == format!("{t}.litematic"))
        .cloned()
}

pub fn list_lines(files: &[String], dir: &str) -> Vec<String> {
    let found: Vec<&String> = files
        .iter()
        .filter(|f| f.to_lowercase().ends_with(".litematic"))
        .collect();
    if found.is_empty() {
        return vec![format!("No schematics. Put .litematic files in {dir}")];
    }
    std::iter::once(format!("{} schematics:", found.len()))
        .chain(found.iter().map(|f| (*f).clone()))
        .collect()
}

/// Bits per block in a region's packed array: enough for the palette, never fewer than 2.
pub fn bits_for(palette_len: usize) -> usize {
    let mut bits = 2;
    while (1usize << bits) < palette_len {
        bits += 1;
    }
    bits
}

/// The index of block `i` in a Litematica bit array, where entries run across the 64-bit words without gaps.
pub fn index_at(longs: &[i64], bits: usize, i: usize) -> u64 {
    let mask = (1u64 << bits) - 1;
    let start_bit = i * bits;
    let (start, end) = (start_bit >> 6, ((i + 1) * bits - 1) >> 6);
    let offset = start_bit & 63;
    let word = |n: usize| longs.get(n).copied().unwrap_or(0) as u64;
    if start == end {
        (word(start) >> offset) & mask
    } else {
        ((word(start) >> offset) | (word(end) << (64 - offset))) & mask
    }
}

/// The number of blocks in a region: the product of its size, whatever the signs.
pub fn volume(size: [i64; 3]) -> u64 {
    size.iter().map(|v| v.unsigned_abs()).product()
}

/// Whether a block name is air.
pub fn is_air(name: &str) -> bool {
    matches!(
        name.strip_prefix("minecraft:").unwrap_or(name),
        "air" | "cave_air" | "void_air"
    )
}

/// How many of each block (not air) a schematic needs, by name.
pub fn materials(s: &Schematic) -> BTreeMap<String, u64> {
    let mut out = BTreeMap::new();
    for r in &s.regions {
        let bits = bits_for(r.palette.len());
        for i in 0..volume(r.size) as usize {
            let at = index_at(&r.states, bits, i) as usize;
            if let Some(name) = r.palette.get(at).filter(|n| !is_air(n)) {
                *out.entry(name.clone()).or_insert(0) += 1;
            }
        }
    }
    out
}

/// `minecraft:oak_planks` as `Oak Planks`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `3 stacks + 5` for a count, at 64 to a stack.
pub fn stacks(count: u64) -> String {
    match (count / 64, count % 64) {
        (0, r) => format!("{r}"),
        (s, 0) => format!("{s} stacks"),
        (s, r) => format!("{s} stacks + {r}"),
    }
}

/// The material list, biggest first, at most `top` lines.
pub fn material_lines(m: &BTreeMap<String, u64>, top: usize, with_stacks: bool) -> Vec<String> {
    let mut rows: Vec<(&String, &u64)> = m.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let total: u64 = m.values().sum();
    let mut out = vec![format!("{} kinds, {} blocks:", rows.len(), total)];
    out.extend(rows.iter().take(top).map(|(name, n)| {
        if with_stacks && **n >= 64 {
            format!("{}: {} ({})", title(name), n, stacks(**n))
        } else {
            format!("{}: {}", title(name), n)
        }
    }));
    if rows.len() > top {
        out.push(format!("... and {} more kinds", rows.len() - top));
    }
    out
}

pub fn info_lines(file: &str, s: &Schematic) -> Vec<String> {
    let blocks: u64 = materials(s).values().sum();
    let mut out = vec![format!(
        "{file}: {}{}",
        if s.name.is_empty() {
            "unnamed"
        } else {
            &s.name
        },
        if s.author.is_empty() {
            String::new()
        } else {
            format!(" by {}", s.author)
        }
    )];
    if !s.description.is_empty() {
        out.push(s.description.clone());
    }
    out.push(format!(
        "{} region(s), {blocks} blocks (not air)",
        s.regions.len()
    ));
    out.extend(s.regions.iter().take(8).map(|r| {
        format!(
            "{}: {} x {} x {}",
            r.name,
            r.size[0].abs(),
            r.size[1].abs(),
            r.size[2].abs()
        )
    }));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Packs indices into longs the way Litematica does: bit after bit across word boundaries.
    fn pack(indices: &[u64], bits: usize) -> Vec<i64> {
        let total = (indices.len() * bits).div_ceil(64);
        let mut out = vec![0u64; total];
        for (i, v) in indices.iter().enumerate() {
            let start = i * bits;
            let (word, offset) = (start >> 6, start & 63);
            out[word] |= v << offset;
            if offset + bits > 64 {
                out[word + 1] |= v >> (64 - offset);
            }
        }
        out.into_iter().map(|w| w as i64).collect()
    }

    fn sample() -> Schematic {
        let palette = vec![
            "minecraft:air".to_owned(),
            "minecraft:stone".to_owned(),
            "minecraft:oak_planks".to_owned(),
        ];
        // A 3 x 3 x 3 region: 27 entries at 2 bits.
        let mut indices = vec![0u64; 27];
        for (i, v) in indices.iter_mut().enumerate() {
            *v = [1, 2, 0][i % 3];
        }
        Schematic {
            name: "House".into(),
            author: "Me".into(),
            description: String::new(),
            regions: vec![Region {
                name: "main".into(),
                size: [3, 3, 3],
                palette,
                states: pack(&indices, 2),
            }],
        }
    }

    #[test]
    fn bits_cover_the_palette_with_a_floor_of_two() {
        assert_eq!(bits_for(1), 2);
        assert_eq!(bits_for(4), 2);
        assert_eq!(bits_for(5), 3);
        assert_eq!(bits_for(17), 5);
        assert_eq!(bits_for(256), 8);
    }

    #[test]
    fn unpacking_follows_the_word_boundaries() {
        for bits in [2usize, 3, 5, 7, 13] {
            let indices: Vec<u64> = (0..200)
                .map(|i| (i * 31 + 7) as u64 & ((1 << bits) - 1))
                .collect();
            let longs = pack(&indices, bits);
            for (i, v) in indices.iter().enumerate() {
                assert_eq!(index_at(&longs, bits, i), *v, "bits {bits} index {i}");
            }
        }
    }

    #[test]
    fn materials_count_every_block_but_air() {
        let m = materials(&sample());
        assert_eq!(m.get("minecraft:stone"), Some(&9));
        assert_eq!(m.get("minecraft:oak_planks"), Some(&9));
        assert!(!m.contains_key("minecraft:air"));
        assert_eq!(volume([3, -3, 3]), 27, "a negative size counts as its size");
    }

    #[test]
    fn the_material_list_and_info_read_well() {
        let lines = material_lines(&materials(&sample()), 1, true);
        assert_eq!(
            lines,
            [
                "2 kinds, 18 blocks:",
                "Oak Planks: 9",
                "... and 1 more kinds"
            ]
        );
        assert_eq!(stacks(64), "1 stacks");
        assert_eq!(stacks(200), "3 stacks + 8");
        assert_eq!(stacks(5), "5");
        let big: BTreeMap<String, u64> = [("minecraft:dirt".to_owned(), 130)].into();
        assert_eq!(material_lines(&big, 5, true)[1], "Dirt: 130 (2 stacks + 2)");
        let info = info_lines("house.litematic", &sample());
        assert_eq!(info[0], "house.litematic: House by Me");
        assert_eq!(info[1], "1 region(s), 18 blocks (not air)");
        assert_eq!(info[2], "main: 3 x 3 x 3");
    }

    #[test]
    fn requests_and_file_names() {
        assert_eq!(request(&[]), Request::Help);
        assert_eq!(request(&["list"]), Request::List);
        assert_eq!(
            request(&["materials", "house", "5"]),
            Request::Materials("house".into(), 5)
        );
        assert_eq!(
            request(&["materials", "house", "x"]),
            Request::Materials("house".into(), 20)
        );
        let files = vec!["House.litematic".to_owned(), "notes.txt".to_owned()];
        assert_eq!(
            find_file("house", &files),
            Some("House.litematic".to_owned())
        );
        assert_eq!(
            find_file("HOUSE.litematic", &files),
            Some("House.litematic".to_owned())
        );
        assert_eq!(find_file("castle", &files), None);
        assert_eq!(
            list_lines(&files, "dir"),
            ["1 schematics:", "House.litematic"]
        );
        assert!(list_lines(&[], "dir")[0].starts_with("No schematics"));
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
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
}
