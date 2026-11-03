// Chat::layout, rebuilt: the same draw list with each line that was just added still sliding up into its place.
fn chat_animation_layout(chat: &Chat, height: i32, ticks: u64, focused: bool) -> std::ops::ControlFlow<Vec<GuiDraw>> {
        let total = chat.lines.len();
        if total == 0 {
            return std::ops::ControlFlow::Break(Vec::new());
        }
        let o = &chat.options;
        let scale = o.scale as f32;
        let max_width = hud_layout::ceil(o.width_px() as f32 / scale);
        let chat_bottom = hud_layout::floor((height - 40) as f32 / scale);
        let text_opacity = o.opacity as f32 * 0.9 + 0.1;
        let background_opacity = o.background_opacity as f32;
        let entry_height = o.line_height();
        let bottom_to_text =
            (8.0 * (o.line_spacing + 1.0) - 4.0 * o.line_spacing + 0.5).floor() as i32;
        let alpha_of = |line: &Line| {
            if focused {
                1.0
            } else {
                time_alpha(ticks, line.added)
            }
        };
        let visible = total
            .saturating_sub(chat.scroll)
            .min(chat.lines_per_page(focused));
        let lines: Vec<(usize, &Line, f32)> = (0..visible)
            .rev()
            .map(|i| (i, &chat.lines[i + chat.scroll]))
            .map(|(i, line)| (i, line, alpha_of(line)))
            .filter(|(_, _, alpha)| *alpha > 1.0e-5)
            .collect();
        let mut out = vec![GuiDraw::Push(vec![
            PoseOp::Scale(scale, scale),
            PoseOp::Translate(4.0, 0.0),
        ])];
        for &(i, line, alpha) in &lines {
            let bottom = chat_bottom - i as i32 * entry_height + lib_chat_animation::slide(ticks, line.added, entry_height);
            out.push(hud_layout::fill(
                -4,
                bottom - entry_height,
                max_width + 4 + 4,
                bottom,
                hud_layout::as8(alpha * background_opacity) << 24,
            ));
        }
        for &(i, line, alpha) in &lines {
            let bottom = chat_bottom - i as i32 * entry_height + lib_chat_animation::slide(ticks, line.added, entry_height);
            let opacity = alpha * text_opacity;
            let color = hud_layout::white(opacity);
            if color >> 24 != 0 {
                out.push(GuiDraw::Text {
                    text: line.content.clone(),
                    x: 0,
                    y: bottom - bottom_to_text,
                    color,
                    shadow: true,
                });
            }
            if let Some(indicator) = line.indicator {
                let color = hud_layout::as8(opacity) << 24 | indicator & 0xFF_FFFF;
                out.push(hud_layout::fill(
                    -4,
                    bottom - entry_height,
                    -2,
                    bottom,
                    color,
                ));
            }
        }
        if focused {
            let count = lines.len() as i32;
            let chat_height = count * entry_height;
            let virtual_height = total as i32 * entry_height;
            let y = chat.scroll as i32 * chat_height / total as i32 - chat_bottom;
            let height = chat_height * chat_height / virtual_height;
            if virtual_height != chat_height {
                let alpha: u32 = if y > 0 { 170 } else { 96 };
                let color = if chat.new_since_scroll {
                    0xCC3333
                } else {
                    0x3333AA
                };
                let x = max_width + 4;
                out.push(hud_layout::fill(
                    x,
                    -y,
                    x + 2,
                    -y - height,
                    alpha << 24 | color,
                ));
                out.push(hud_layout::fill(
                    x + 2,
                    -y,
                    x + 1,
                    -y - height,
                    alpha << 24 | 0xCCCCCC,
                ));
            }
        }
        out.push(GuiDraw::Pop);
        std::ops::ControlFlow::Break(out)
}
