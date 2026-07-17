/*!
Button Components — Design System
Reusable button widgets using theme tokens.
*/

use crate::ui::theme::*;
use eframe::egui::{self, Color32, Vec2};

// ═══════════════════════════════════════════════════════════════════════════════
// ICON BUTTON — square, icon-only, with optional active state
// ═══════════════════════════════════════════════════════════════════════════════

/// A compact, square icon button.
///
/// Uses Design System tokens for consistent sizing and color.
/// Renders custom background for hover/active states.
pub fn icon_button(
    ui: &mut egui::Ui,
    icon: &str,
    tooltip: &str,
    active: bool,
) -> egui::Response {
    let size = Vec2::splat(ICON_BTN_SIZE);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    let bg = if active {
        BG_ACTIVE
    } else if response.hovered() {
        BG_HOVER
    } else {
        Color32::TRANSPARENT
    };

    let fg = if active {
        Color32::WHITE
    } else if response.hovered() {
        FG_PRIMARY
    } else {
        FG_SECONDARY
    };

    // Background
    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, RADIUS_SM, bg);
    }

    // Icon text
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(FONT_SIZE_SECTION),
        fg,
    );

    response.on_hover_text(tooltip)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TEXT BUTTON — rectangular, with label
// ═══════════════════════════════════════════════════════════════════════════════

/// A styled text button with Design System tokens.
pub fn text_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(
        egui::RichText::new(text)
            .size(FONT_SIZE_BODY)
            .color(FG_PRIMARY)
    )
    .fill(BG_ELEVATED)
    .stroke(egui::Stroke::new(1.0, BORDER))
    .corner_radius(egui::CornerRadius::same(RADIUS_SM as u8))
    .min_size(egui::vec2(80.0, ICON_BTN_SIZE));

    ui.add(btn)
}

// ═══════════════════════════════════════════════════════════════════════════════
// LABELED ICON BUTTON — icon + text side by side
// ═══════════════════════════════════════════════════════════════════════════════

/// A button with icon and text label.
pub fn labeled_icon_button(
    ui: &mut egui::Ui,
    icon: &str,
    label: &str,
    tooltip: &str,
    active: bool,
) -> egui::Response {
    let (bg_color, text_color) = if active {
        (BG_ACTIVE, Color32::WHITE)
    } else {
        (Color32::TRANSPARENT, FG_PRIMARY)
    };

    let text = egui::RichText::new(format!("{icon}  {label}"))
        .size(FONT_SIZE_BODY)
        .color(text_color);
    let btn = egui::Button::new(text)
        .min_size(egui::vec2(108.0, 30.0))
        .corner_radius(egui::CornerRadius::same(RADIUS_SM as u8))
        .fill(bg_color)
        .stroke(egui::Stroke::NONE);
    ui.add(btn).on_hover_text(tooltip)
}
