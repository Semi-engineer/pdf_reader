/*!
Activity Bar — Vertical navigation ribbon (left edge)
Inspired by VS Code, provides primary navigation between workspace panels
*/

use super::theme::*;
use crate::workspace::PanelId;
use eframe::egui::{self, Color32, Stroke, Vec2};

/// Activity bar button state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityBarAction {
    ToggleThumbnails,
    ToggleSearch,
    ToggleOutline,
    ToggleBookmarks,
    ToggleAnnotations,
    ToggleAttachments,
}

impl ActivityBarAction {
    fn from_panel(panel: PanelId) -> Option<Self> {
        match panel {
            PanelId::Thumbnails => Some(Self::ToggleThumbnails),
            PanelId::Search => Some(Self::ToggleSearch),
            PanelId::Outline => Some(Self::ToggleOutline),
            PanelId::Bookmarks => Some(Self::ToggleBookmarks),
            PanelId::Annotations => Some(Self::ToggleAnnotations),
            PanelId::Attachments => Some(Self::ToggleAttachments),
            _ => None,
        }
    }

    fn to_panel(&self) -> PanelId {
        match self {
            Self::ToggleThumbnails => PanelId::Thumbnails,
            Self::ToggleSearch => PanelId::Search,
            Self::ToggleOutline => PanelId::Outline,
            Self::ToggleBookmarks => PanelId::Bookmarks,
            Self::ToggleAnnotations => PanelId::Annotations,
            Self::ToggleAttachments => PanelId::Attachments,
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::ToggleThumbnails => "▦",   // Grid icon
            Self::ToggleSearch => "⚲",       // Search/target icon
            Self::ToggleOutline => "≡",      // List icon
            Self::ToggleBookmarks => "◈",    // Bookmark diamond
            Self::ToggleAnnotations => "✎",  // Pen icon
            Self::ToggleAttachments => "⚓",  // Attachment icon
        }
    }

    fn tooltip(&self) -> &'static str {
        match self {
            Self::ToggleThumbnails => "Thumbnails",
            Self::ToggleSearch => "Search",
            Self::ToggleOutline => "Outline",
            Self::ToggleBookmarks => "Bookmarks",
            Self::ToggleAnnotations => "Annotations",
            Self::ToggleAttachments => "Attachments",
        }
    }
}

pub struct ActivityBar {
    actions: Vec<ActivityBarAction>,
}

impl ActivityBar {
    pub fn new() -> Self {
        Self {
            actions: vec![
                ActivityBarAction::ToggleThumbnails,
                ActivityBarAction::ToggleSearch,
                ActivityBarAction::ToggleOutline,
                ActivityBarAction::ToggleBookmarks,
                ActivityBarAction::ToggleAnnotations,
                ActivityBarAction::ToggleAttachments,
            ],
        }
    }

    /// Show activity bar and return the clicked action (if any)
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        active_panel: PanelId,
        sidebar_visible: bool,
    ) -> Option<ActivityBarAction> {
        let mut clicked_action = None;

        egui::Frame::new()
            .fill(BG_SURFACE)
            .stroke(Stroke::new(1.0, BORDER))
            .show(ui, |ui| {
                ui.set_width(ACTIVITY_BAR_WIDTH);
                ui.set_min_height(ui.available_height());

                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);

                    // Logo / app icon at top
                    self.show_logo(ui);

                    ui.add_space(12.0);
                    ui.add(egui::Separator::default().horizontal().spacing(8.0));
                    ui.add_space(8.0);

                    // Activity buttons
                    for action in &self.actions {
                        let panel = action.to_panel();
                        let is_active = sidebar_visible && active_panel == panel;

                        if self.activity_button(ui, action.icon(), action.tooltip(), is_active).clicked() {
                            clicked_action = Some(*action);
                        }

                        ui.add_space(2.0);
                    }

                    // Push settings to bottom
                    ui.with_layout(
                        egui::Layout::bottom_up(egui::Align::Center),
                        |ui| {
                            ui.add_space(8.0);

                            // Settings button at bottom
                            if self.activity_button(ui, "⚙", "Settings", false).clicked() {
                                // TODO: Open settings
                            }

                            ui.add_space(4.0);
                        },
                    );
                });
            });

        clicked_action
    }

    /// Activity bar button (icon only, vertically stacked)
    fn activity_button(
        &self,
        ui: &mut egui::Ui,
        icon: &str,
        tooltip: &str,
        active: bool,
    ) -> egui::Response {
        let size = Vec2::splat(40.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        // Visual state
        let bg_color = if active {
            BG_ACTIVE
        } else if response.hovered() {
            BG_HOVER
        } else {
            Color32::TRANSPARENT
        };

        let fg_color = if active {
            Color32::WHITE
        } else if response.hovered() {
            FG_PRIMARY
        } else {
            FG_SECONDARY
        };

        // Draw button background
        if bg_color != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, 2.0, bg_color);
        }

        // Active indicator (left edge bar)
        if active {
            let indicator_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.min.y + 8.0),
                Vec2::new(3.0, rect.height() - 16.0),
            );
            ui.painter().rect_filled(indicator_rect, 1.0, FG_ACCENT);
        }

        // Draw icon
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(18.0),
            fg_color,
        );

        response.on_hover_text(tooltip)
    }

    /// Logo area at top of activity bar
    fn show_logo(&self, ui: &mut egui::Ui) {
        let size = Vec2::new(32.0, 32.0);
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

        // Simple PDF icon representation
        let painter = ui.painter();
        let icon_color = FG_ACCENT;

        // Document rectangle with folded corner
        let doc_rect = rect.shrink(4.0);
        let corner_size = size.x * 0.2;

        let points = vec![
            doc_rect.left_top(),
            egui::pos2(doc_rect.right() - corner_size, doc_rect.top()),
            doc_rect.right_top() + egui::vec2(0.0, corner_size),
            doc_rect.right_bottom(),
            doc_rect.left_bottom(),
        ];

        painter.add(egui::Shape::closed_line(
            points,
            Stroke::new(2.0, icon_color),
        ));

        // Text lines inside
        let line_width = doc_rect.width() * 0.5;
        let line_x = doc_rect.center().x - line_width * 0.5;
        let start_y = doc_rect.center().y - 4.0;

        for i in 0..2 {
            let y = start_y + (i as f32) * 6.0;
            painter.line_segment(
                [egui::pos2(line_x, y), egui::pos2(line_x + line_width, y)],
                Stroke::new(1.5, icon_color),
            );
        }
    }
}

impl Default for ActivityBar {
    fn default() -> Self {
        Self::new()
    }
}
