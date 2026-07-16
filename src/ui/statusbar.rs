/*!
Status Bar — slim info bar at the bottom.
*/

use super::theme::{BG_SURFACE, BORDER, FG_ACCENT, FG_PRIMARY, FG_SECONDARY, FG_SUCCESS};
use crate::app::DocLensApp;
use eframe::egui::{self, RichText, Stroke};

pub struct StatusBar {
    /// Frames remaining to show the current status_message
    msg_ttl: u8,
}

impl StatusBar {
    pub fn new() -> Self { Self { msg_ttl: 0 } }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut DocLensApp) {
        egui::Frame::new()
            .fill(BG_SURFACE)
            .stroke(Stroke::new(1.0, BORDER))
            .inner_margin(egui::Margin { left: 10, right: 10, top: 3, bottom: 3 })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;

                    // ── Left: file name ───────────────────────────────────
                    if let Some(path) = &app.doc_path {
                        let name = std::path::Path::new(path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(path.as_str());
                        ui.label(
                            RichText::new(format!("»  {name}"))
                                .size(12.5).color(FG_PRIMARY)
                        );
                        vdiv(ui);
                    }

                    // ── Page ─────────────────────────────────────────────
                    if let Some(doc) = &app.document {
                        ui.label(
                            RichText::new(format!("p. {}  /  {}", app.current_page + 1, doc.page_count()))
                                .size(12.0).color(FG_SECONDARY)
                        );
                        vdiv(ui);
                    }

                    // ── Zoom ─────────────────────────────────────────────
                    ui.label(
                        RichText::new(format!("{:.0}%", app.zoom_level))
                            .size(12.0).color(FG_SECONDARY)
                    );

                    // ── Rotation ─────────────────────────────────────────
                    if app.rotation != 0 {
                        vdiv(ui);
                        ui.label(
                            RichText::new(format!("{}°", app.rotation))
                                .size(12.0).color(FG_SECONDARY)
                        );
                    }

                    // ── Active tool ───────────────────────────────────────
                    if let Some(tool) = &app.current_tool {
                        vdiv(ui);
                        ui.label(
                            RichText::new(format!("{:?}", tool))
                                .size(12.0).color(FG_ACCENT)
                        );
                    } else if app.document.is_some() {
                        vdiv(ui);
                        ui.label(
                            RichText::new("Select").size(12.0).color(FG_SECONDARY)
                        );
                    }

                    // ── Search count ─────────────────────────────────────
                    let sc = app.search_manager.result_count();
                    if sc > 0 {
                        vdiv(ui);
                        ui.label(
                            RichText::new(format!("~  {} / {}", app.search_manager.current_index() + 1, sc))
                                .size(12.0).color(FG_ACCENT)
                        );
                    }

                    // ── Right: status message + cache ─────────────────────
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Cache counter (small, dim)
                        ui.label(
                            RichText::new(format!("# {}", app.page_cache.len()))
                                .size(10.5).color(BORDER)
                        );

                        // Transient status message with TTL fade
                        if let Some(msg) = &app.status_message {
                            if self.msg_ttl == 0 { self.msg_ttl = 180; } // ~3 s @ 60 fps
                            vdiv(ui);
                            ui.label(
                                RichText::new(msg.as_str())
                                    .size(12.0).color(FG_SUCCESS)
                            );
                        }

                        if self.msg_ttl > 0 {
                            self.msg_ttl -= 1;
                            if self.msg_ttl == 0 {
                                app.status_message = None;
                            }
                            ui.ctx().request_repaint();
                        }
                    });
                });
            });
    }
}

fn vdiv(ui: &mut egui::Ui) {
    ui.add(egui::Separator::default().vertical().spacing(6.0));
}

impl Default for StatusBar {
    fn default() -> Self { Self::new() }
}
