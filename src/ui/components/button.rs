/*!
Button Components
*/

use crate::ui::theme::*;
use eframe::egui;

pub fn icon_button(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
    let btn = egui::Button::new(
        egui::RichText::new(icon)
            .size(14.0)
            .color(FG_SECONDARY)
    )
    .min_size(egui::vec2(32.0, 32.0))
    .frame(false);
    
    let response = ui.add(btn);
    
    if response.hovered() {
        ui.painter().rect_filled(
            response.rect,
            3.0,
            BG_HOVER,
        );
    }
    
    response.on_hover_text(tooltip)
}

pub fn toggle_button(
    ui: &mut egui::Ui,
    icon: &str,
    tooltip: &str,
    active: bool,
) -> egui::Response {
    let (bg_color, text_color) = if active {
        (BG_ACTIVE, egui::Color32::WHITE)
    } else {
        (egui::Color32::TRANSPARENT, FG_SECONDARY)
    };
    
    let btn = egui::Button::new(
        egui::RichText::new(icon)
            .size(14.0)
            .color(text_color)
    )
    .min_size(egui::vec2(32.0, 32.0))
    .fill(bg_color)
    .frame(active);
    
    let response = ui.add(btn);
    
    if !active && response.hovered() {
        ui.painter().rect_filled(
            response.rect,
            3.0,
            BG_HOVER,
        );
    }
    
    response.on_hover_text(tooltip)
}

pub fn text_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(
        egui::RichText::new(text)
            .size(13.0)
            .color(FG_PRIMARY)
    )
    .fill(BG_ELEVATED)
    .stroke(egui::Stroke::new(1.0, BORDER))
    .min_size(egui::vec2(80.0, 28.0));
    
    ui.add(btn)
}
