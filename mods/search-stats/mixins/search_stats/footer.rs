// The footer: the search box, left of the Done button.
fn search_stats_footer(game: &crate::game::Game<'_>, batch: &mut HudBatch, w: f32, h: f32) {
    use lib_search_stats as m;
    if !m::shown() {
        return;
    }
    let atlas = &game.gui.atlas;
    let width = (w / 2.0 - 100.0 - 12.0).clamp(60.0, 160.0);
    let (x, y) = (((w / 2.0 - 100.0 - 8.0) - width).max(4.0).floor(), h - FOOTER + 6.0);
    batch.fill(atlas, x - 1.0, y - 1.0, width + 2.0, 22.0, 0xFFA0_A0A0);
    batch.fill(atlas, x, y, width, 20.0, 0xFF00_0000);
    let query = m::query();
    if query.is_empty() {
        batch.text(atlas, "Search...", x + 4.0, y + 6.0, 0xFF70_7070, false);
    } else {
        // Keep the end of a long search in view.
        let mut shown: String = query.clone();
        while HudBatch::text_width(atlas, &shown) > width - 14.0 && !shown.is_empty() {
            shown.remove(0);
        }
        batch.text(atlas, &shown, x + 4.0, y + 6.0, 0xFFFF_FFFF, true);
        let caret = x + 4.0 + HudBatch::text_width(atlas, &shown);
        if (game.started.elapsed().as_millis() / 500) % 2 == 0 {
            batch.fill(atlas, caret + 1.0, y + 5.0, 1.0, 10.0, 0xFFFF_FFFF);
        }
    }
}
