/*!
Workspace Layout System
Manages dockable panels and workspace state
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelId {
    // Left Sidebar
    Thumbnails,
    Outline,
    Search,
    Bookmarks,
    Annotations,
    Attachments,
    
    // Right Sidebar
    Properties,
    PageInfo,
    Metadata,
    AnnotationInspector,
}

impl PanelId {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Thumbnails => "Thumbnails",
            Self::Outline => "Outline",
            Self::Search => "Search",
            Self::Bookmarks => "Bookmarks",
            Self::Annotations => "Annotations",
            Self::Attachments => "Attachments",
            Self::Properties => "Properties",
            Self::PageInfo => "Page Info",
            Self::Metadata => "Metadata",
            Self::AnnotationInspector => "Annotation Inspector",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Thumbnails => "🖼",
            Self::Outline => "📋",
            Self::Search => "🔍",
            Self::Bookmarks => "🔖",
            Self::Annotations => "✏",
            Self::Attachments => "📎",
            Self::Properties => "⚙",
            Self::PageInfo => "📄",
            Self::Metadata => "ℹ",
            Self::AnnotationInspector => "🔎",
        }
    }
    
    pub fn is_left_panel(&self) -> bool {
        matches!(
            self,
            Self::Thumbnails
                | Self::Outline
                | Self::Search
                | Self::Bookmarks
                | Self::Annotations
                | Self::Attachments
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub left_sidebar_visible: bool,
    pub right_sidebar_visible: bool,
    pub left_sidebar_width: f32,
    pub right_sidebar_width: f32,
    pub active_left_panel: PanelId,
    pub active_right_panel: PanelId,
    pub two_page_mode: bool,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            left_sidebar_visible: true,
            right_sidebar_visible: false,
            left_sidebar_width: 240.0,
            right_sidebar_width: 280.0,
            active_left_panel: PanelId::Thumbnails,
            active_right_panel: PanelId::Properties,
            two_page_mode: false,
        }
    }
}

impl WorkspaceState {
    pub fn toggle_left_sidebar(&mut self) {
        self.left_sidebar_visible = !self.left_sidebar_visible;
    }
    
    pub fn toggle_right_sidebar(&mut self) {
        self.right_sidebar_visible = !self.right_sidebar_visible;
    }
    
    pub fn set_active_left_panel(&mut self, panel: PanelId) {
        if panel.is_left_panel() {
            self.active_left_panel = panel;
            self.left_sidebar_visible = true;
        }
    }
    
    pub fn set_active_right_panel(&mut self, panel: PanelId) {
        if !panel.is_left_panel() {
            self.active_right_panel = panel;
            self.right_sidebar_visible = true;
        }
    }
}
