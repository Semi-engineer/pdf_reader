/*!
UI Module
Contains all UI components
*/

pub mod theme;
pub mod icons;
pub mod components;
pub mod panels;
pub mod activity_bar;

// Legacy components (kept for now)
pub mod titlebar;
pub mod toolbar;
pub mod sidebar;
pub mod viewer;
pub mod statusbar;
pub mod tool_palette;

pub use activity_bar::ActivityBar;
pub use titlebar::show as show_title_bar;
pub use toolbar::Toolbar;
pub use sidebar::Sidebar;
pub use viewer::PdfViewer;
pub use statusbar::StatusBar;
pub use tool_palette::ToolPalette;
