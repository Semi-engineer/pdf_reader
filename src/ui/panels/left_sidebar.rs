/*!
Left Sidebar
Navigation panels (Thumbnails, Outline, Search, Bookmarks, Annotations)
Uses Design System tokens and shared components.
*/

use crate::app::DocLensApp;
use crate::ui::components::*;
use crate::ui::icons;
use crate::ui::theme::*;
use crate::workspace::PanelId;
use eframe::egui;

pub struct LeftSidebar;

impl LeftSidebar {
    pub fn show(ui: &mut egui::Ui, app: &mut DocLensApp) {
        // Panel header with tabs
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            let tabs: &[(PanelId, &str)] = &[
                (PanelId::Thumbnails, icons::ICON_THUMBNAILS),
                (PanelId::Outline,    icons::ICON_OUTLINE),
                (PanelId::Search,     icons::ICON_SEARCH),
                (PanelId::Bookmarks,  icons::ICON_BOOKMARKS),
                (PanelId::Annotations, icons::ICON_ANNOTATIONS),
            ];

            for (panel, icon) in tabs {
                let active = app.workspace.active_left_panel == *panel;
                if ui.selectable_label(active, *icon).on_hover_text(panel.name()).clicked() {
                    app.workspace.set_active_left_panel(*panel);
                }
            }
        });

        ui.separator();

        // Panel content
        egui::ScrollArea::vertical().show(ui, |ui| {
            match app.workspace.active_left_panel {
                PanelId::Thumbnails => show_thumbnails_panel(ui, app),
                PanelId::Outline => show_outline_panel(ui, app),
                PanelId::Search => show_search_panel(ui, app),
                PanelId::Bookmarks => show_bookmarks_panel(ui, app),
                PanelId::Annotations => show_annotations_panel(ui, app),
                _ => {}
            }
        });
    }
}

fn show_thumbnails_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    Panel::new("Thumbnails").icon(icons::ICON_THUMBNAILS).show(ui, |ui| {
        if let Some(doc) = &app.document {
            let page_count = doc.page_count();

            for page in 0..page_count {
                let selected = page == app.current_page;

                let frame_rect = egui::Rect::from_min_size(
                    ui.cursor().min,
                    egui::vec2(ui.available_width(), 140.0)
                );

                let response = ui.allocate_rect(frame_rect, egui::Sense::click());

                if response.clicked() {
                    app.goto_page(page);
                }

                let bg_color = if selected {
                    BG_ACTIVE
                } else if response.hovered() {
                    BG_HOVER
                } else {
                    BG_ELEVATED
                };

                ui.painter().rect_filled(frame_rect, RADIUS_SM, bg_color);

                // Thumbnail image
                if let Some(image) = app.thumbnail_manager.get_thumbnail(page) {
                    let img_width = image.width() as f32;
                    let img_height = image.height() as f32;
                    let aspect = img_width / img_height;
                    let thumb_width = frame_rect.width() * 0.8;
                    let thumb_height = thumb_width / aspect;

                    let thumb_rect = egui::Rect::from_center_size(
                        frame_rect.center(),
                        egui::vec2(thumb_width, thumb_height)
                    );

                    let texture: egui::TextureHandle = ui.ctx().load_texture(
                        format!("thumb_{}", page),
                        image.as_ref().clone(),
                        egui::TextureOptions::LINEAR
                    );

                    ui.painter().image(
                        texture.id(),
                        thumb_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }

                // Page number
                let text_color = if selected { egui::Color32::WHITE } else { FG_SECONDARY };
                ui.painter().text(
                    egui::pos2(frame_rect.center().x, frame_rect.bottom() - 20.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{}", page + 1),
                    egui::FontId::proportional(FONT_SIZE_CAPTION),
                    text_color,
                );

                ui.add_space(SP_SM);
            }
        } else {
            ui.label(egui::RichText::new("No document open").color(FG_SECONDARY));
        }
    });
}

fn show_outline_panel(ui: &mut egui::Ui, _app: &mut DocLensApp) {
    Panel::new("Outline").icon(icons::ICON_OUTLINE).show(ui, |ui| {
        // Placeholder — outline requires PDF bookmark/outline API extraction
        ui.add_space(SP_LG);
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(icons::ICON_OUTLINE)
                    .size(SP_XXL)
                    .color(FG_TERTIARY)
            );
            ui.add_space(SP_SM);
            ui.label(
                egui::RichText::new("No outline available")
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_SECONDARY)
            );
            ui.label(
                egui::RichText::new("This document does not contain a table of contents.")
                    .size(FONT_SIZE_TINY)
                    .color(FG_TERTIARY)
            );
        });
    });
}

fn show_search_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    Panel::new("Search").icon(icons::ICON_SEARCH).show(ui, |ui| {
        let mut search_box = SearchBox::new("Search in document...");

        if let Some(query) = search_box.show(ui) {
            if !query.is_empty() {
                app.perform_search(query);
            } else {
                app.search_manager.clear();
            }
        }

        ui.add_space(SP_SM);

        // Results
        let result_count = app.search_manager.result_count();
        if result_count > 0 {
            ui.label(
                egui::RichText::new(format!("{} results", result_count))
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_SECONDARY)
            );

            ui.add_space(SP_XS);

            // Clone data before mutable borrow
            let results: Vec<(usize, String)> = app.search_manager
                .results()
                .iter()
                .map(|r| (r.page, r.text.clone()))
                .collect();

            for (page, text) in results {
                if ui.selectable_label(
                    false,
                    format!("Page {} — {}", page + 1, text)
                ).clicked() {
                    app.goto_page(page);
                }
            }
        }
    });
}

fn show_bookmarks_panel(ui: &mut egui::Ui, _app: &mut DocLensApp) {
    Panel::new("Bookmarks").icon(icons::ICON_BOOKMARKS).show(ui, |ui| {
        ui.add_space(SP_LG);
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(icons::ICON_BOOKMARKS)
                    .size(SP_XXL)
                    .color(FG_TERTIARY)
            );
            ui.add_space(SP_SM);
            ui.label(
                egui::RichText::new("No bookmarks")
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_SECONDARY)
            );
            ui.label(
                egui::RichText::new("Bookmarks will appear here.")
                    .size(FONT_SIZE_TINY)
                    .color(FG_TERTIARY)
            );
        });
    });
}

fn show_annotations_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    Panel::new("Annotations").icon(icons::ICON_ANNOTATIONS).show(ui, |ui| {
        let annotations = app.annotation_manager.get_all_annotations();

        if annotations.is_empty() {
            ui.add_space(SP_LG);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(icons::ICON_ANNOTATIONS)
                        .size(SP_XXL)
                        .color(FG_TERTIARY)
                );
                ui.add_space(SP_SM);
                ui.label(
                    egui::RichText::new("No annotations")
                        .size(FONT_SIZE_CAPTION)
                        .color(FG_SECONDARY)
                );
            });
        } else {
            for ann in annotations {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("Page {}", ann.page + 1))
                            .size(FONT_SIZE_CAPTION)
                            .color(FG_PRIMARY)
                    );
                    ui.label(
                        egui::RichText::new(format!("{:?}", ann.annotation_type))
                            .size(FONT_SIZE_CAPTION)
                            .color(FG_SECONDARY)
                    );
                });
            }
        }
    });
}
