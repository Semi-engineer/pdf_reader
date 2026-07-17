/*!
Command Palette
Quick command launcher (Ctrl+Shift+P)
*/

use crate::commands::Command;
use crate::ui::theme::*;
use eframe::egui;

pub struct CommandPalette {
    visible: bool,
    query: String,
    filtered: Vec<Command>,
    selected_index: usize,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            visible: false,
            query: String::new(),
            filtered: Self::all_commands(),
            selected_index: 0,
        }
    }
    
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if self.visible {
            self.query.clear();
            self.filtered = Self::all_commands();
            self.selected_index = 0;
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context) -> Option<Command> {
        if !self.visible {
            return None;
        }
        
        let mut result = None;
        
        egui::Window::new("Command Palette")
            .anchor(egui::Align2::CENTER_TOP, [0.0, 100.0])
            .fixed_size([500.0, 400.0])
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Search input
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.query)
                            .hint_text("Type command name...")
                            .desired_width(ui.available_width())
                    );
                    
                    if response.changed() {
                        self.filter_commands();
                        self.selected_index = 0;
                    }
                    
                    response.request_focus();
                    
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);
                    
                    // Command list
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            for (i, cmd) in self.filtered.iter().enumerate() {
                                let selected = i == self.selected_index;
                                
                                let response = ui.selectable_label(
                                    selected,
                                    egui::RichText::new(cmd.name())
                                        .size(13.0)
                                        .color(if selected { egui::Color32::WHITE } else { FG_PRIMARY })
                                );
                                
                                if response.clicked() {
                                    result = Some(*cmd);
                                    self.visible = false;
                                }
                            }
                        });
                    
                    // Keyboard navigation
                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Some(cmd) = self.filtered.get(self.selected_index) {
                            result = Some(*cmd);
                            self.visible = false;
                        }
                    }
                    
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.visible = false;
                    }
                    
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        self.selected_index = (self.selected_index + 1).min(self.filtered.len().saturating_sub(1));
                    }
                    
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        self.selected_index = self.selected_index.saturating_sub(1);
                    }
                });
            });
        
        result
    }
    
    fn filter_commands(&mut self) {
        let query = self.query.to_lowercase();
        if query.is_empty() {
            self.filtered = Self::all_commands();
        } else {
            self.filtered = Self::all_commands()
                .into_iter()
                .filter(|cmd| cmd.name().to_lowercase().contains(&query))
                .collect();
        }
    }
    
    fn all_commands() -> Vec<Command> {
        vec![
            Command::OpenDocument,
            Command::SaveDocument,
            Command::ExportPDF,
            Command::PrintDocument,
            Command::CloseDocument,
            Command::NextPage,
            Command::PreviousPage,
            Command::FirstPage,
            Command::LastPage,
            Command::GoToPage,
            Command::ZoomIn,
            Command::ZoomOut,
            Command::ZoomFit,
            Command::ZoomWidth,
            Command::Zoom100,
            Command::RotateLeft,
            Command::RotateRight,
            Command::ToggleSidebarLeft,
            Command::ToggleSidebarRight,
            Command::ToggleTwoPageMode,
            Command::Search,
            Command::SearchNext,
            Command::SearchPrevious,
            Command::AddHighlight,
            Command::AddNote,
            Command::AddDrawing,
            Command::CopyText,
            Command::ShowProperties,
            Command::Quit,
        ]
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}
