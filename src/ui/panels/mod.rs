/*!
Workspace Panels
Dockable sidebar panels
*/

pub mod left_sidebar;
pub mod right_sidebar;
pub mod menubar;
pub mod command_palette;

pub use left_sidebar::LeftSidebar;
pub use right_sidebar::RightSidebar;
pub use menubar::show_menubar;
pub use command_palette::CommandPalette;
