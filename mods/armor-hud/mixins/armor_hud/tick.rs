// Once a tick: switch the armor widget on with these settings (off with the mod).
fn armor_hud_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_armor_hud as m;
    use horizon_client::armor_hud as ah;
    if !game.native_on(m::ID) {
        game.native_armor_hud(None);
        return;
    }
    let get = |k: &str| game.native_setting(m::ID, k);
    let v = |key: &str| m::value(&get, m::ROWS.iter().find(|r| r.key == key).expect("a known row"));
    let int = |key: &str| v(key).parse::<i32>().unwrap_or(0);
    let config = ah::Config {
        enabled: v("enabled") == "true",
        anchor: match v("anchor").as_str() {
            "top_center" => ah::Anchor::TopCenter,
            "top" => ah::Anchor::Top,
            "vertical_center" => ah::Anchor::VertCenter,
            "bottom" => ah::Anchor::Bottom,
            _ => ah::Anchor::Hotbar,
        },
        side: if v("side") == "right" { ah::Side::Right } else { ah::Side::Left },
        offset_x: int("offset_x"),
        offset_y: int("offset_y"),
        style: match v("style").as_str() {
            "rounded_corners" => ah::Style::RoundedCorners,
            "rounded" => ah::Style::Rounded,
            _ => ah::Style::Hotbar,
        },
        orientation: if v("orientation") == "vertical" { ah::Orientation::Vertical } else { ah::Orientation::Horizontal },
        widget_shown: match v("widget_shown").as_str() {
            "always" => ah::WidgetShown::Always,
            "if_any_present" => ah::WidgetShown::IfAnyPresent,
            "damaged_pieces" => ah::WidgetShown::DamagedPieces,
            _ => ah::WidgetShown::NotEmpty,
        },
        offhand_slot: match v("offhand_slot").as_str() {
            "always_ignore" => ah::OffhandSlot::AlwaysIgnore,
            "always_leave_space" => ah::OffhandSlot::AlwaysLeaveSpace,
            _ => ah::OffhandSlot::Adhere,
        },
        durability_display: match v("durability").as_str() {
            "numeric" => ah::Display::Numeric,
            "percentage" => ah::Display::Percentage,
            _ => ah::Display::Bar,
        },
        offhand_durability: v("offhand_durability") == "true",
        mainhand_durability: v("mainhand_durability") == "true",
        reversed: v("reversed") == "true",
        icons_shown: v("icons") == "true",
        warning_shown: v("warning") == "true",
        min_durability_value: int("min_value"),
        min_durability_percentage: v("min_percent").parse::<f64>().unwrap_or(0.1),
        warning_bob_intensity: int("bob"),
    };
    game.native_armor_hud(Some(config));
}
