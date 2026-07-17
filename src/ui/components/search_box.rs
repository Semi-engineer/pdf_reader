/*!
Search Box Component — Design System
*/

use crate::ui::theme::*;
use crate::ui::icons;
use eframe::egui;

pub struct SearchBox {
    text: String,
    hint: String,
}

impl SearchBox {
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            text: String::new(),
            hint: hint.into(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut submitted = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = SP_XS;

            // Search icon
            ui.label(
                egui::RichText::new(icons::ICON_SEARCH_DOC)
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_TERTIARY)
            );

            // Text input
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.text)
                    .hint_text(&self.hint)
                    .desired_width(ui.available_width() - 60.0)
                    .frame(true)
                    .font(egui::FontId::proportional(FONT_SIZE_BODY))
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submitted = Some(self.text.clone());
            }

            // Clear button
            if !self.text.is_empty() {
                if ui.small_button(icons::ICON_CLEAR).clicked() {
                    self.text.clear();
                    submitted = Some(String::new());
                }
            }
        });

        submitted
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Default for SearchBox {
    fn default() -> Self {
        Self::new("Search...")
    }
}
