/*!
Menu Bar
Top menu bar with File, Edit, View, etc.
*/

use crate::commands::{Command, CommandDispatcher};
use crate::ui::theme::*;
use eframe::egui;

pub fn show_menubar(ui: &mut egui::Ui, dispatcher: &mut CommandDispatcher) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if menu_item(ui, "Open", "Ctrl+O", true, dispatcher) {
                dispatcher.execute(Command::OpenDocument);
            }
            if menu_item(ui, "Save", "Ctrl+S", false, dispatcher) {
                dispatcher.execute(Command::SaveDocument);
            }
            ui.separator();
            if menu_item(ui, "Export PDF", "", false, dispatcher) {
                dispatcher.execute(Command::ExportPDF);
            }
            if menu_item(ui, "Print", "Ctrl+P", false, dispatcher) {
                dispatcher.execute(Command::PrintDocument);
            }
            ui.separator();
            if menu_item(ui, "Close", "Ctrl+W", false, dispatcher) {
                dispatcher.execute(Command::CloseDocument);
            }
            if menu_item(ui, "Quit", "Ctrl+Q", true, dispatcher) {
                dispatcher.execute(Command::Quit);
            }
        });
        
        ui.menu_button("Edit", |ui| {
            if menu_item(ui, "Copy", "Ctrl+C", false, dispatcher) {
                dispatcher.execute(Command::CopyText);
            }
            ui.separator();
            if menu_item(ui, "Search", "Ctrl+F", false, dispatcher) {
                dispatcher.execute(Command::Search);
            }
        });
        
        ui.menu_button("View", |ui| {
            if menu_item(ui, "Zoom In", "Ctrl++", true, dispatcher) {
                dispatcher.execute(Command::ZoomIn);
            }
            if menu_item(ui, "Zoom Out", "Ctrl+-", true, dispatcher) {
                dispatcher.execute(Command::ZoomOut);
            }
            if menu_item(ui, "Actual Size", "Ctrl+0", true, dispatcher) {
                dispatcher.execute(Command::Zoom100);
            }
            ui.separator();
            if menu_item(ui, "Rotate Left", "Ctrl+L", true, dispatcher) {
                dispatcher.execute(Command::RotateLeft);
            }
            if menu_item(ui, "Rotate Right", "Ctrl+R", true, dispatcher) {
                dispatcher.execute(Command::RotateRight);
            }
            ui.separator();
            if menu_item(ui, "Two-Page Mode", "", false, dispatcher) {
                dispatcher.execute(Command::ToggleTwoPageMode);
            }
        });
        
        ui.menu_button("Tools", |ui| {
            if menu_item(ui, "Highlight", "", false, dispatcher) {
                dispatcher.execute(Command::AddHighlight);
            }
            if menu_item(ui, "Note", "", false, dispatcher) {
                dispatcher.execute(Command::AddNote);
            }
            if menu_item(ui, "Drawing", "", false, dispatcher) {
                dispatcher.execute(Command::AddDrawing);
            }
        });
    });
}

fn menu_item(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    enabled: bool,
    _dispatcher: &CommandDispatcher,
) -> bool {
    let mut response = ui.add_enabled(
        enabled,
        egui::Button::new(
            egui::RichText::new(label).size(13.0).color(if enabled { FG_PRIMARY } else { FG_SECONDARY })
        )
        .frame(false)
    );
    
    if !shortcut.is_empty() {
        response = response.on_hover_text(shortcut);
    }
    
    response.clicked()
}
