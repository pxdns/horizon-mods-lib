// The box in the bottom-right corner; the debug screen takes the top corners.
fn effect_timer_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_effect_timer as m;
    use horizon_client::extras::Corner;
    if game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "bars") {
        let icons: Vec<m::Icon> = game
            .potions
            .effects
            .list
            .iter()
            .filter(|e| e.visible)
            .map(|e| {
                let def = horizon_client_core::potions::effect(e.id);
                m::Icon {
                    id: e.id.to_owned(),
                    duration: e.duration,
                    ambient: e.ambient,
                    beneficial: def.is_some_and(|d| d.category == horizon_client_core::potions::Category::Beneficial),
                    color: def.map_or(0xFF_FFFF, |d| d.color as u32),
                }
            })
            .collect();
        for (rect, ink) in m::bars(batch.width as i32, &icons) {
            game.native_fill(batch, rect, ink);
        }
    }
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomRight) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let ambient = m::is(|k| game.native_setting(m::ID, k), "ambient");
    let active: Vec<m::Active> = game
        .potions
        .effects
        .list
        .iter()
        .filter(|e| e.visible)
        .map(|e| m::Active { id: e.id.to_owned(), duration: e.duration, amplifier: e.amplifier, ambient: e.ambient })
        .collect();
    let lines = m::lines(&active, ambient);
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::BottomRight, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (line, ink)) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], *ink, true);
    }
}
