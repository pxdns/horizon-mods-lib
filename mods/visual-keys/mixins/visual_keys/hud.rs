// The keyboard, in a corner: each key coloured by what it is bound to, bright while held.
fn visual_keys_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_visual_keys as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let bindings: Vec<(String, String)> = horizon_client::keybinds::MAPPINGS
        .iter()
        .map(|(action, _, category)| ((*category).to_owned(), game.settings.keys.get(action).to_owned()))
        .collect();
    let map = m::bind_map(&bindings);
    let cells = m::cells();
    let (w, h) = m::size(&cells);
    let (x0, y0) = corners.place(Corner::BottomRight, w + 4.0, h + 4.0);
    game.native_fill(batch, [x0, y0, w + 4.0, h + 4.0], m::BACKGROUND);
    for cell in &cells {
        let ink = m::colour(&cell.name, &map, m::pressed(&cell.name));
        let (x, y) = (x0 + 2.0 + cell.x, y0 + 2.0 + cell.y);
        game.native_fill(batch, [x, y, cell.w, cell.h], ink);
        if game.native_text_width(&cell.label) <= cell.w - 1.0 {
            let tx = x + ((cell.w - game.native_text_width(&cell.label)) / 2.0).floor();
            game.native_text(batch, &cell.label, [tx, y + 1.0], m::INK, false);
        }
    }
}
