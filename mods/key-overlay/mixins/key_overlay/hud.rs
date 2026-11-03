// The box in the bottom-right corner; the debug screen takes the top corners.
fn key_overlay_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_key_overlay as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    let (shown, mouse, space) = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        (m::is(get, "shown"), m::is(get, "mouse"), m::is(get, "space"))
    };
    if !shown {
        return;
    }
    let cells = m::cells(mouse, space);
    let mut state = Vec::new();
    for cell in &cells {
        let (down, label) = game.native_mapping(cell.mapping).unwrap_or((false, String::new()));
        let text = cell.fixed.map_or_else(|| m::short(&label), str::to_owned);
        state.push((down, text));
    }
    let game = &*game;
    let (w, h) = m::bounds(&cells);
    let (left, top) = corners.place(Corner::BottomRight, w, h);
    for (cell, (down, text)) in cells.iter().zip(state) {
        let (background, ink) = if down {
            (m::HELD_BACKGROUND, m::HELD_INK)
        } else {
            (m::IDLE_BACKGROUND, m::IDLE_INK)
        };
        let (x, y) = (left + cell.x, top + cell.y);
        game.native_fill(batch, [x, y, cell.w, cell.h], background);
        let tx = x + (cell.w - game.native_text_width(&text)) / 2.0;
        game.native_text(batch, &text, [tx, y + (cell.h - 8.0) / 2.0], ink, !down);
    }
}
