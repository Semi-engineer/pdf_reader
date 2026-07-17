/*!
Activity Bar — Vertical navigation ribbon (left edge)
Inspired by VS Code, provides primary navigation between workspace panels.
Uses Design System tokens and centralized icon definitions.
*/

use crate::ui::theme::*;
use crate::ui::icons;
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
            Self::ToggleThumbnails => icons::ICON_THUMBNAILS,
            Self::ToggleSearch => icons::ICON_SEARCH,
            Self::ToggleOutline => icons::ICON_OUTLINE,
            Self::ToggleBookmarks => icons::ICON_BOOKMARKS,
            Self::ToggleAnnotations => icons::ICON_ANNOTATIONS,
            Self::ToggleAttachments => icons::ICON_ATTACHMENTS,
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
                    ui.add_space(SP_SM);

                    // Logo / app icon at top
                    self.show_logo(ui);

                    ui.add_space(SP_MD);
                    ui.add(egui::Separator::default().horizontal().spacing(SP_SM));
                    ui.add_space(SP_SM);

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
                            ui.add_space(SP_SM);

                            // Settings button at bottom
                            if self.activity_button(ui, icons::ICON_SETTINGS, "Settings", false).clicked() {
                                // TODO: Open settings
                            }

                            ui.add_space(SP_XS);
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
        let size = Vec2::splat(ACTIVITY_BTN_SIZE);
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
            ui.painter().rect_filled(rect, RADIUS_SM, bg_color);
        }

        // Active indicator (left edge bar)
        if active {
            let indicator_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.min.y + SP_SM),
                Vec2::new(3.0, rect.height() - SP_LG),
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
        icons::draw_pdf_icon(ui, Vec2::new(32.0, 32.0));
    }
}

impl Default for ActivityBar {
    fn default() -> Self {
        Self::new()
    }
}
