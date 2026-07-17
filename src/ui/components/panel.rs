/*!
Panel Component — Design System
Container with header and content area for sidebar panels.
*/

use crate::ui::theme::*;
use eframe::egui;

pub struct Panel {
    title: String,
    icon: Option<String>,
    show_separator: bool,
}

impl Panel {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            show_separator: true,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
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
                ui.add_space(SP_SM);
                if let Some(icon) = &self.icon {
                    ui.label(
                        egui::RichText::new(icon)
                            .size(FONT_SIZE_CAPTION)
                            .color(FG_TERTIARY)
                    );
                    ui.add_space(SP_XS);
                }
                ui.label(
                    egui::RichText::new(&self.title)
                        .size(FONT_SIZE_CAPTION)
                        .color(FG_SECONDARY)
                        .strong()
                );
            });

            if self.show_separator {
                ui.add_space(SP_XS);
                ui.separator();
                ui.add_space(SP_XS);
            } else {
                ui.add_space(SP_SM);
            }

            // Content
            content(ui)
        })
    }
}
