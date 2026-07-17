/*!
UI Module
Contains all UI components
*/

pub mod theme;
pub mod icons;
pub mod components;
pub mod panels;
pub mod activity_bar;
pub mod titlebar;
pub mod toolbar;
pub mod viewer;
pub mod statusbar;

pub use activity_bar::ActivityBar;
pub use titlebar::show as show_title_bar;
pub use toolbar::Toolbar;
pub use viewer::PdfViewer;
pub use statusbar::StatusBar;
