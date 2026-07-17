/*!
Toolbar Component — Design System
Toolbar layout helpers: groups, dividers.
*/

use crate::ui::theme::*;
use eframe::egui;

/// Vertical divider between toolbar groups.
pub fn toolbar_divider(ui: &mut egui::Ui) {
    ui.add_space(SP_XS);
    ui.add(
        egui::Separator::default()
            .vertical()
            .spacing(SP_XS)
    );
    ui.add_space(SP_XS);
}
