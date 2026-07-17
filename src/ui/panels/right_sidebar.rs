/*!
Right Sidebar
Inspector panels (Properties, Metadata, Page Info)
*/

use crate::app::DocLensApp;
use crate::ui::components::{PropertyGrid, Section};
use crate::ui::theme::*;
use crate::workspace::PanelId;
use eframe::egui;

pub struct RightSidebar;

impl RightSidebar {
    pub fn show(ui: &mut egui::Ui, app: &mut DocLensApp) {
        // Panel tabs
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            
            for panel in &[
                PanelId::Properties,
                PanelId::PageInfo,
                PanelId::Metadata,
                PanelId::AnnotationInspector,
            ] {
                let active = app.workspace.active_right_panel == *panel;
                
                if ui.selectable_label(active, panel.icon()).clicked() {
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
    Section::new("View Properties").show(ui, |ui| {
        PropertyGrid::new()
            .add("Zoom", format!("{:.0}%", app.zoom_level))
            .add("Rotation", format!("{}°", app.rotation))
            .add("Two-Page", if app.two_page_mode { "Yes" } else { "No" })
            .show(ui);
    });
    
    ui.add_space(12.0);
    
    if let Some(doc) = &app.document {
        Section::new("Document").show(ui, |ui| {
            PropertyGrid::new()
                .add("Pages", format!("{}", doc.page_count()))
                .add("Current", format!("{}", app.current_page + 1))
                .show(ui);
        });
    }
}

fn show_page_info_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    if app.document.is_some() {
        Section::new("Current Page").show(ui, |ui| {
            ui.label(egui::RichText::new("Page info available after implementing page API").color(FG_SECONDARY).italics());
        });
    } else {
        ui.label(egui::RichText::new("No document open").color(FG_SECONDARY));
    }
}

fn show_metadata_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    if let Some(_doc) = &app.document {
        Section::new("Document Metadata").show(ui, |ui| {
            ui.label(egui::RichText::new("Metadata extraction not yet implemented").color(FG_SECONDARY).italics());
        });
    } else {
        ui.label(egui::RichText::new("No document open").color(FG_SECONDARY));
    }
}

fn show_annotation_inspector_panel(ui: &mut egui::Ui, app: &mut DocLensApp) {
    Section::new("Selected Annotation").show(ui, |ui| {
        let annotations = app.annotation_manager.get_all_annotations();
        
        if annotations.is_empty() {
            ui.label(egui::RichText::new("No annotations").color(FG_SECONDARY));
        } else {
            ui.label(egui::RichText::new("Select an annotation to inspect").color(FG_SECONDARY));
        }
    });
}
