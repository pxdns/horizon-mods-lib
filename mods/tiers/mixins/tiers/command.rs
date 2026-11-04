// `/tier <name>`, `/tier url <template>`, `/tier me`.
fn tiers_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_tiers as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    fn to_json(value: &serde_json::Value) -> crate::game::lib_tiers::Json {
        use crate::game::lib_tiers::Json;
        match value {
            serde_json::Value::Null => Json::Null,
            serde_json::Value::Bool(b) => Json::Bool(*b),
            serde_json::Value::Number(n) => Json::Num(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => Json::Str(s.clone()),
            serde_json::Value::Array(a) => Json::Arr(a.iter().map(to_json).collect()),
            serde_json::Value::Object(o) => Json::Obj(o.iter().map(|(k, v)| (k.clone(), to_json(v))).collect()),
        }
    }
    let rest: Vec<&str> = words.collect();
    let url = game.native_setting(m::ID, "url").unwrap_or_else(|| m::DEFAULT_URL.to_owned());
    let lookups = m::is(|k| game.native_setting(m::ID, k), "lookups");
    let me = game.options.player_name().to_owned();
    match m::request(&rest, &me) {
        m::Request::Help => {
            for line in m::HELP {
                game.native_chat(line);
            }
            game.native_chat(&format!("Lookups go to {} ({})", m::host_of(&url), if lookups { "on" } else { "off: turn them on in Options > Mods > Tiers" }));
        }
        m::Request::SetUrl(template) => match m::check_url(&template) {
            Ok(()) => {
                game.native_set_setting(m::ID, "url", &template);
                game.native_chat(&format!("Tiers will ask {}", m::host_of(&template)));
            }
            Err(why) => game.native_chat(&why),
        },
        m::Request::Lookup(name) => {
            if !lookups {
                game.native_chat(&format!("Lookups are off. A lookup sends the name `{name}` to {}; turn them on in Options > Mods > Tiers.", m::host_of(&url)));
                return;
            }
            let now = game.ticks as u64 / 20;
            if let Some(cached) = m::cached(&name, now) {
                for line in cached {
                    game.native_chat(&line);
                }
                return;
            }
            let target = m::fill(&url, &name);
            let worker_name = name.clone();
            game.native_chat(&format!("Asking {} about {name}...", m::host_of(&url)));
            std::thread::spawn(move || {
                let result = ureq::get(&target)
                    .set("User-Agent", "horizon-mods-lib tiers (https://github.com/pxdns/horizon-mods-lib)")
                    .timeout(std::time::Duration::from_secs(8))
                    .call()
                    .map_err(|e| e.to_string())
                    .and_then(|response| response.into_string().map_err(|e| e.to_string()))
                    .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).map_err(|e| format!("the reply was not JSON: {e}")))
                    .map(|value| to_json(&value));
                m::deliver(worker_name, result);
            });
        }
    }
}
