/*!
Property Grid Component
Key-value property display
*/

use crate::ui::theme::*;
use eframe::egui;

pub struct PropertyGrid {
    properties: Vec<(String, String)>,
}

impl PropertyGrid {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }
    
    pub fn add(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.push((key.into(), value.into()));
        self
    }
    
    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Grid::new("property_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                for (key, value) in &self.properties {
                    ui.label(
                        egui::RichText::new(key)
                            .size(12.0)
                            .color(FG_SECONDARY)
                    );
                    ui.label(
                        egui::RichText::new(value)
                            .size(12.0)
                            .color(FG_PRIMARY)
                    );
                    ui.end_row();
                }
            });
    }
}

impl Default for PropertyGrid {
    fn default() -> Self {
        Self::new()
    }
}
