// `/statistics top|find|export`: reads the statistics Horizon keeps.
fn better_stats_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::lib_better_stats as m;
    let mut words = command.split_whitespace();
    if !game.native_on(m::ID) || words.next() != Some(m::COMMAND) {
        return;
    }
    *ran = true;
    let rest: Vec<&str> = words.collect();
    game.load_stats();
    let entries = m::entries(&game.stats.values);
    match m::request(&rest) {
        m::Request::Help => {
            for line in m::HELP {
                game.native_chat(line);
            }
        }
        m::Request::Top(kind, n) => {
            for line in m::top(&entries, &kind, n) {
                game.native_chat(&line);
            }
        }
        m::Request::Find(terms) => {
            for line in m::find(&entries, &terms) {
                game.native_chat(&line);
            }
        }
        m::Request::Export => {
            let dir = game.native_data_dir(m::ID);
            let csv = m::is(|k| game.native_setting(m::ID, k), "csv");
            let result = std::fs::create_dir_all(&dir)
                .and_then(|()| std::fs::write(dir.join("statistics.txt"), m::export_text(&entries)))
                .and_then(|()| if csv { std::fs::write(dir.join("statistics.csv"), m::export_csv(&entries)) } else { Ok(()) });
            match result {
                Ok(()) => game.native_chat(&format!("Statistics written to {}", dir.join("statistics.txt").display())),
                Err(e) => game.native_chat(&format!("Could not write the statistics: {e}")),
            }
        }
    }
}
