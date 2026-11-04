// `/schematic list|info|materials|export`.
fn litematica_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_litematica as m;
    use horizon_world::nbt::{Limits, Tag, read_named};
    use std::io::Read;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    let dir = game.native_data_dir(m::ID).join("schematics");
    let request = m::request(&rest);
    let say = |game: &mut crate::game::Game<'_>, lines: Vec<String>| {
        for line in lines {
            game.native_chat(&line);
        }
    };
    let name = match &request {
        m::Request::Info(n) | m::Request::Materials(n, _) | m::Request::Export(n) => n.clone(),
        m::Request::List => {
            let mut files: Vec<String> = std::fs::read_dir(&dir)
                .map(|d| d.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).collect())
                .unwrap_or_default();
            files.sort();
            let lines = m::list_lines(&files, &dir.display().to_string());
            return say(game, lines);
        }
        m::Request::Help => return say(game, m::HELP.map(String::from).to_vec()),
    };
    let Some(file) = m::find_file(&name, &std::fs::read_dir(&dir).map(|d| d.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).collect::<Vec<_>>()).unwrap_or_default()) else {
        return say(game, vec![format!("No schematic called `{name}` in {}", dir.display())]);
    };
    let bytes = match std::fs::read(dir.join(&file)) {
        Ok(b) => b,
        Err(e) => return say(game, vec![format!("Could not read {file}: {e}")]),
    };
    let mut raw = Vec::new();
    if flate2::read::GzDecoder::new(&bytes[..]).read_to_end(&mut raw).is_err() {
        return say(game, vec![format!("{file} is not a gzip file; a .litematic is")]);
    }
    let limits = Limits { max_bytes: 512 * 1024 * 1024, max_nodes: 50_000_000, max_depth: 512 };
    let root = match read_named(&raw, limits) {
        Ok((named, _)) => named.tag,
        Err(e) => return say(game, vec![format!("Could not read {file}: {e:?}")]),
    };
    let text = |t: Option<&Tag>| match t {
        Some(Tag::String(s)) => s.to_string_lossy(),
        _ => String::new(),
    };
    let int3 = |t: Option<&Tag>| -> [i64; 3] {
        let get = |k: &str| match t.and_then(|t| t.get(k)) {
            Some(Tag::Int(v)) => i64::from(*v),
            _ => 0,
        };
        [get("x"), get("y"), get("z")]
    };
    let mut regions = Vec::new();
    if let Some(Tag::Compound(map)) = root.get("Regions") {
        for (region_name, region) in map.iter() {
            let palette: Vec<String> = match region.get("BlockStatePalette") {
                Some(Tag::List(list)) => list.iter().map(|e| text(e.get("Name"))).collect(),
                _ => Vec::new(),
            };
            let states: Vec<i64> = match region.get("BlockStates") {
                Some(Tag::LongArray(v)) => v.clone(),
                _ => Vec::new(),
            };
            regions.push(m::Region { name: region_name.to_string_lossy(), size: int3(region.get("Size")), palette, states });
        }
    }
    let meta = root.get("Metadata");
    let schematic = m::Schematic { name: text(meta.and_then(|m| m.get("Name"))), author: text(meta.and_then(|m| m.get("Author"))), description: text(meta.and_then(|m| m.get("Description"))), regions };
    match request {
        m::Request::Info(_) => say(game, m::info_lines(&file, &schematic)),
        m::Request::Materials(_, top) => {
            let stacks = m::is(|k| game.native_setting(m::ID, k), "stacks");
            say(game, m::material_lines(&m::materials(&schematic), top, stacks));
        }
        m::Request::Export(_) => {
            let out = game.native_data_dir(m::ID).join(format!("{}-materials.txt", file.trim_end_matches(".litematic")));
            let body = m::material_lines(&m::materials(&schematic), usize::MAX, true).join("\n");
            match std::fs::write(&out, body) {
                Ok(()) => say(game, vec![format!("Material list written to {}", out.display())]),
                Err(e) => say(game, vec![format!("Could not write the material list: {e}")]),
            }
        }
        _ => {}
    }
}
