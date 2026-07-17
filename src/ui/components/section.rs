/*!
Section Component
Collapsible section with header
*/

use crate::ui::theme::*;
use eframe::egui;

pub struct Section {
    title: String,
    default_open: bool,
}

impl Section {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            default_open: true,
        }
    }
    
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<egui::InnerResponse<R>> {
        let id = ui.make_persistent_id(&self.title);
        let response = egui::CollapsingHeader::new(
            egui::RichText::new(&self.title)
                .size(12.5)
                .color(FG_PRIMARY)
                .strong()
        )
        .id_salt(id)
        .default_open(self.default_open)
        .show(ui, |ui| {
            ui.add_space(4.0);
            content(ui)
        });
        
        response.body_response.and_then(|body| {
            response.body_returned.map(|inner| egui::InnerResponse {
                inner,
                response: body,
            })
        })
    }
}
