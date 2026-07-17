/*!
Property Grid Component — Design System
Key-value display for inspector panels.
*/

use crate::ui::theme::*;
use eframe::egui;

pub struct PropertyGrid {
    id: String,
    properties: Vec<(String, String)>,
}

impl PropertyGrid {
    pub fn new() -> Self {
        Self {
            id: "property_grid".into(),
            properties: Vec::new(),
        }
    }

    /// Set a unique ID (needed when multiple grids appear in the same panel).
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn add(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.push((key.into(), value.into()));
        self
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Grid::new(&self.id)
            .num_columns(2)
            .spacing([SP_SM, SP_XS])
            .striped(false)
            .show(ui, |ui| {
                for (key, value) in &self.properties {
                    ui.label(
                        egui::RichText::new(key)
                            .size(FONT_SIZE_CAPTION)
                            .color(FG_SECONDARY)
                    );
                    ui.label(
                        egui::RichText::new(value)
                            .size(FONT_SIZE_CAPTION)
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
