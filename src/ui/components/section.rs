/*!
Section Component — Design System
Collapsible section with header for inspector panels.
*/

use crate::ui::theme::*;
use eframe::egui;

pub struct Section {
    title: String,
    icon: Option<String>,
    default_open: bool,
}

impl Section {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            default_open: true,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
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

        let header_text = if let Some(icon) = &self.icon {
            format!("{icon}  {}", self.title)
        } else {
            self.title.clone()
        };

        let response = egui::CollapsingHeader::new(
            egui::RichText::new(&header_text)
                .size(FONT_SIZE_SECTION)
                .color(FG_PRIMARY)
                .strong()
        )
        .id_salt(id)
        .default_open(self.default_open)
        .show(ui, |ui| {
            ui.add_space(SP_XS);
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
