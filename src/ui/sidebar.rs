/*!
Sidebar — thumbnail panel with current-page highlight.
*/

use super::theme::{BG_ACTIVE, BG_ACTIVE_DIM, BG_ELEVATED, FG_ACCENT, FG_SECONDARY};
use crate::app::DocLensApp;
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};

pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> Self { Self {} }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        let page_count = match &app.document {
            Some(d) => d.page_count(),
            None => {
                self.empty_state(ui);
                return;
            }
        };
        let current_page = app.current_page;
        let mut goto: Option<usize> = None;

        // Header
        egui::Frame::new()
            .fill(super::theme::BG_SURFACE)
            .inner_margin(egui::Margin { left: 10, right: 10, top: 8, bottom: 6 })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Pages").size(12.5).color(FG_SECONDARY).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{page_count}"))
                                .size(11.5).color(FG_SECONDARY)
                        );
                    });
                });
            });

        ui.add(egui::Separator::default().spacing(0.0));

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(6.0);
                for page in 0..page_count {
                    let is_current = page == current_page;
                    self.thumb_card(ui, app, page, is_current, &mut goto);
                    ui.add_space(4.0);
                }
                ui.add_space(6.0);
            });

        if let Some(p) = goto { app.goto_page(p); }
    }

    fn thumb_card(
        &self,
        ui: &mut egui::Ui,
        app: &DocLensApp,
        page: usize,
        is_current: bool,
        goto: &mut Option<usize>,
    ) {
        let margin = egui::Margin::symmetric(8, 4);
        let fill = if is_current { *BG_ACTIVE_DIM } else { Color32::TRANSPARENT };
        let stroke = if is_current {
            Stroke::new(1.5, BG_ACTIVE)
        } else {
            Stroke::NONE
        };

        let resp = egui::Frame::new()
            .fill(fill)
            .stroke(stroke)
            .corner_radius(egui::CornerRadius::same(6))
            .inner_margin(margin)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                // Thumbnail image or placeholder
                if let Some(thumb) = app.thumbnail_manager.get_thumbnail(page) {
                    let tex = ui.ctx().load_texture(
                        format!("thumb_{page}"),
                        thumb.as_ref().clone(),
                        egui::TextureOptions::LINEAR,
                    );
                    let avail = ui.available_width() - 4.0;
                    let aspect = tex.size()[1] as f32 / tex.size()[0] as f32;
                    let h = (avail * aspect).min(160.0);
                    let size = Vec2::new(avail, h);
                    ui.add(
                        egui::Image::new(&tex)
                            .fit_to_exact_size(size)
                            .corner_radius(egui::CornerRadius::same(3))
                    );
                } else {
                    // Skeleton placeholder
                    let (rect, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width() - 4.0, 110.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(rect, 3.0, BG_ELEVATED);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "…",
                        egui::FontId::proportional(18.0),
                        FG_SECONDARY,
                    );
                }

                ui.add_space(4.0);

                // Page label
                let label_color = if is_current { FG_ACCENT } else { FG_SECONDARY };
                ui.label(
                    RichText::new(format!("{}", page + 1))
                        .size(11.0)
                        .color(label_color),
                );
            })
            .response;

        // Make the whole card clickable
        let card_resp = resp.interact(egui::Sense::click());
        if is_current {
            // Subtle glow on current page label
        }
        if card_resp.hovered() && !is_current {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if card_resp.clicked() {
            *goto = Some(page);
        }
    }

    fn empty_state(&self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("▣").size(32.0));
            ui.add_space(6.0);
            ui.label(RichText::new("No document").size(12.0).color(FG_SECONDARY));
        });
    }
}

impl Default for Sidebar {
    fn default() -> Self { Self::new() }
}
