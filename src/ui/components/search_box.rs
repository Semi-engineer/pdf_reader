/*!
Search Box Component
*/

use crate::ui::theme::*;
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
            ui.spacing_mut().item_spacing.x = 4.0;
            
            // Search icon
            ui.label(egui::RichText::new("🔍").size(12.0).color(FG_SECONDARY));
            
            // Text input
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.text)
                    .hint_text(&self.hint)
                    .desired_width(ui.available_width() - 60.0)
                    .frame(true)
            );
            
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submitted = Some(self.text.clone());
            }
            
            // Clear button
            if !self.text.is_empty() {
                if ui.small_button("✖").clicked() {
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
