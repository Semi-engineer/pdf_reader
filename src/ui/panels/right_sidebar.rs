/*!
Right Sidebar
Inspector panels (Properties, Metadata, Page Info, Annotation Inspector)
Uses Design System tokens and shared components.
*/

use crate::app::DocLensApp;
use crate::ui::components::{PropertyGrid, Section};
use crate::ui::icons;
use crate::ui::theme::*;
use crate::workspace::PanelId;
use eframe::egui;

pub struct RightSidebar;

impl RightSidebar {
    pub fn show(ui: &mut egui::Ui, app: &mut DocLensApp) {
        // Panel tabs
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            let tabs: &[(PanelId, &str)] = &[
                (PanelId::Properties, icons::ICON_PROPERTIES),
                (PanelId::PageInfo,   icons::ICON_PAGE_INFO),
                (PanelId::Metadata,   icons::ICON_METADATA),
                (PanelId::AnnotationInspector, icons::ICON_ANNOTATION_INSPECT),
            ];

            for (panel, icon) in tabs {
                let active = app.workspace.active_right_panel == *panel;
                if ui.selectable_label(active, *icon).on_hover_text(panel.name()).clicked() {
                    app.workspace.set_active_right_panel(*panel);
                }
            }
        });

        ui.separator();

        // Panel content
        egui::ScrollArea::vertical().show(ui, |ui| {
            match app.workspace.active_right_panel {
                PanelId::Properties => show_properties_panel(ui, app),
                PanelId::PageInfo => show_page_info_panel(ui, app),
                PanelId::Metadata => show_metadata_panel(ui, app),
                PanelId::AnnotationInspector => show_annotation_inspector_panel(ui, app),
                _ => {}
            }
        });
    }
}

fn show_properties_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    Section::new("View Properties").icon(icons::ICON_PROPERTIES).show(ui, |ui| {
        PropertyGrid::new().id("view_props")
            .add("Zoom", format!("{:.0}%", app.zoom_level))
            .add("Rotation", format!("{}°", app.rotation))
            .add("Two-Page", if app.two_page_mode { "Yes" } else { "No" })
            .show(ui);
    });

    ui.add_space(SP_MD);

    if let Some(doc) = &app.document {
        Section::new("Document").icon(icons::ICON_FILE).show(ui, |ui| {
            PropertyGrid::new().id("doc_props")
                .add("Pages", format!("{}", doc.page_count()))
                .add("Current", format!("{}", app.current_page + 1))
                .show(ui);
        });
    }
}

fn show_page_info_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    if app.document.is_some() {
        Section::new("Current Page").icon(icons::ICON_PAGE_INFO).show(ui, |ui| {
            PropertyGrid::new().id("page_info")
                .add("Page", format!("{}", app.current_page + 1))
                .add("Zoom", format!("{:.0}%", app.zoom_level))
                .add("Rotation", format!("{}°", app.rotation))
                .show(ui);
        });
    } else {
        ui.add_space(SP_LG);
        ui.label(egui::RichText::new("No document open").color(FG_SECONDARY));
    }
}

fn show_metadata_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    if let Some(_doc) = &app.document {
        Section::new("Document Metadata").icon(icons::ICON_METADATA).show(ui, |ui| {
            ui.label(
                egui::RichText::new("Metadata extraction not yet implemented")
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_SECONDARY)
                    .italics()
            );
        });
    } else {
        ui.add_space(SP_LG);
        ui.label(egui::RichText::new("No document open").color(FG_SECONDARY));
    }
}

fn show_annotation_inspector_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    Section::new("Selected Annotation").icon(icons::ICON_ANNOTATION_INSPECT).show(ui, |ui| {
        let annotations = app.annotation_manager.get_all_annotations();

        if annotations.is_empty() {
            ui.label(
                egui::RichText::new("No annotations")
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_SECONDARY)
            );
        } else {
            ui.label(
                egui::RichText::new("Select an annotation to inspect")
                    .size(FONT_SIZE_CAPTION)
                    .color(FG_SECONDARY)
            );
        }
    });
}
