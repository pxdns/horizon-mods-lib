// hud_layout::crosshair: the sprite Horizon draws is replaced by the mod's shape (the attack indicator after it stays).
fn entity_crosshair_crosshair(draws: Vec<GuiDraw>, w: i32, h: i32) -> Vec<GuiDraw> {
    let Some((shape, color)) = lib_entity_crosshair::current() else {
        return draws;
    };
    let (x0, y0) = ((w - 15) / 2, (h - 15) / 2);
    let mut out: Vec<GuiDraw> = draws.into_iter().skip(1).collect();
    for (x, y, rw, rh) in shape {
        out.push(fill(x0 + x, y0 + y, x0 + x + rw, y0 + y + rh, color));
    }
    out
}
