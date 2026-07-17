/*!
Status Bar — Professional bottom info bar
Shows: file, page, zoom, rotation, tool, search, cache, memory, tasks
All icons from icons.rs constants (no inline emoji).
*/

use super::icons;
use super::theme::*;
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
            .inner_margin(egui::Margin::symmetric(10, 2))
            .show(ui, |ui| {
                ui.set_height(STATUS_BAR_HEIGHT - 4.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = SP_SM;

                    // ═══ LEFT SIDE: Document Info ═════════════════════════

                    // File name
                    if let Some(path) = &app.doc_path {
                        let name = std::path::Path::new(path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(path.as_str());

                        ui.label(
                            RichText::new(format!("{} {}", icons::ICON_FILE, name))
                                .size(FONT_SIZE_CAPTION)
                                .color(FG_PRIMARY)
                        );
                        status_divider(ui);
                    }

                    // Page count
                    if let Some(doc) = &app.document {
                        ui.label(
                            RichText::new(format!(
                                "Page {} / {}",
                                app.current_page + 1,
                                doc.page_count()
                            ))
                            .size(FONT_SIZE_CAPTION)
                            .color(FG_SECONDARY)
                        );
                        status_divider(ui);
                    }

                    // Zoom level
                    if app.document.is_some() {
                        ui.label(
                            RichText::new(format!("{}%", app.zoom_level as i32))
                                .size(FONT_SIZE_CAPTION)
                                .color(FG_SECONDARY)
                        );
                    }

                    // Rotation indicator
                    if app.rotation != 0 {
                        status_divider(ui);
                        ui.label(
                            RichText::new(format!("{}°", app.rotation))
                                .size(FONT_SIZE_CAPTION)
                                .color(FG_SECONDARY)
                        );
                    }

                    // Active tool
                    if let Some(tool) = &app.current_tool {
                        status_divider(ui);
                        ui.label(
                            RichText::new(format!("{} {:?}", icons::ICON_TOOL, tool))
                                .size(FONT_SIZE_CAPTION)
                                .color(FG_ACCENT)
                        );
                    } else if app.document.is_some() {
                        status_divider(ui);
                        ui.label(
                            RichText::new(format!("{} Select", icons::ICON_SELECT))
                                .size(FONT_SIZE_CAPTION)
                                .color(FG_TERTIARY)
                        );
                    }

                    // Search results
                    let search_count = app.search_manager.result_count();
                    if search_count > 0 {
                        status_divider(ui);
                        ui.label(
                            RichText::new(format!(
                                "{} {} / {}",
                                icons::ICON_SEARCH_DOC,
                                app.search_manager.current_index() + 1,
                                search_count
                            ))
                            .size(FONT_SIZE_CAPTION)
                            .color(FG_ACCENT)
                        );
                    }

                    // ═══ RIGHT SIDE: Technical Info ═══════════════════════
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = SP_SM;

                            // Cache status
                            let cache_size = app.page_cache.len();
                            if cache_size > 0 {
                                ui.label(
                                    RichText::new(format!("{} {}", icons::ICON_CACHE, cache_size))
                                        .size(FONT_SIZE_TINY)
                                        .color(FG_TERTIARY)
                                ).on_hover_text("Cached pages");
                            }

                            // Rendering state
                            if app.render_worker.is_some() && app.document.is_some() {
                                status_divider(ui);
                                ui.label(
                                    RichText::new(icons::ICON_RENDERER)
                                        .size(FONT_SIZE_TINY)
                                        .color(FG_SUCCESS)
                                ).on_hover_text("Renderer active");
                            }

                            // Memory usage
                            if app.document.is_some() {
                                status_divider(ui);
                                let mem_mb = (cache_size * 2).max(10);
                                ui.label(
                                    RichText::new(format!("RAM: {}MB", mem_mb))
                                        .size(FONT_SIZE_TINY)
                                        .color(FG_TERTIARY)
                                );
                            }

                            // Transient status message with TTL fade
                            if let Some(msg) = &app.status_message {
                                if self.msg_ttl == 0 {
                                    self.msg_ttl = 180; // ~3s @ 60fps
                                }
                                status_divider(ui);
                                ui.label(
                                    RichText::new(msg.as_str())
                                        .size(FONT_SIZE_CAPTION)
                                        .color(FG_SUCCESS)
                                );
                            }

                            if self.msg_ttl > 0 {
                                self.msg_ttl -= 1;
                                if self.msg_ttl == 0 {
                                    app.status_message = None;
                                }
                                ui.ctx().request_repaint();
                            }
                        },
                    );
                });
            });
    }
}

/// Vertical divider for status bar
fn status_divider(ui: &mut egui::Ui) {
    ui.add(
        egui::Separator::default()
            .vertical()
            .spacing(SP_XS)
    );
}

impl Default for StatusBar {
    fn default() -> Self { Self::new() }
}
