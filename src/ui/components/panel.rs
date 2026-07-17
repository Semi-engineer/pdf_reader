/*!
Panel Component
Container with header and collapsible content
*/

use crate::ui::theme::*;
use eframe::egui;

pub struct Panel {
    title: String,
    collapsible: bool,
    show_separator: bool,
}

impl Panel {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            collapsible: false,
            show_separator: true,
        }
    }
    
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
    
    pub fn separator(mut self, show: bool) -> Self {
        self.show_separator = show;
        self
    }
    
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<R> {
        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&self.title)
                        .size(12.0)
                        .color(FG_SECONDARY)
                        .strong()
                );
            });
            
            if self.show_separator {
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);
            } else {
                ui.add_space(8.0);
            }
            
            // Content
            content(ui)
        })
    }
}
