/*!
Custom Title Bar
Frameless window with drag-to-move, minimize, maximize, close.
Styled to match the DocLens dark theme.
*/

use super::theme::{BG_BASE, BG_ELEVATED, FG_PRIMARY, FG_SECONDARY};
use eframe::egui::{self, Color32, FontId, Stroke, Vec2};

pub const TITLE_BAR_HEIGHT: f32 = 34.0;

/// Renders the custom title bar and returns whether the window should close.
pub fn show(ctx: &egui::Context, doc_name: Option<&str>) -> bool {
    let mut should_close = false;

    egui::TopBottomPanel::top("title_bar")
        .exact_height(TITLE_BAR_HEIGHT)
        .frame(egui::Frame::new().fill(BG_BASE).stroke(Stroke::NONE))
        .show(ctx, |ui| {
            should_close = render_title_bar(ctx, ui, doc_name);
        });

    should_close
}

fn render_title_bar(ctx: &egui::Context, ui: &mut egui::Ui, doc_name: Option<&str>) -> bool {
    let mut close = false;

    // Full-width interaction zone for dragging
    let bar_rect = ui.max_rect();
    let drag_resp = ui.interact(
        bar_rect,
        egui::Id::new("titlebar_drag"),
        egui::Sense::click(),
    );

    if drag_resp.double_clicked() {
        let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    } else if drag_resp.is_pointer_button_down_on() {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    // Paint background
    ui.painter()
        .rect_filled(bar_rect, 0.0, BG_BASE);

    // Bottom separator line
    ui.painter().line_segment(
        [bar_rect.left_bottom(), bar_rect.right_bottom()],
        Stroke::new(1.0, Color32::from_rgb(45, 45, 58)),
    );

    ui.horizontal(|ui| {
        ui.set_min_height(TITLE_BAR_HEIGHT);
        ui.spacing_mut().item_spacing.x = 0.0;

        // ── Left: icon + title ──────────────────────────────────────────
        ui.add_space(12.0);

        ui.label(
            egui::RichText::new("▣")
                .size(15.0),
        );

        ui.add_space(6.0);

        ui.label(
            egui::RichText::new("DocLens")
                .size(13.5)
                .color(FG_PRIMARY)
                .strong(),
        );

        // Dim separator + document name
        if let Some(name) = doc_name {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("—")
                    .size(12.0)
                    .color(Color32::from_rgb(70, 70, 90)),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(name)
                    .size(12.5)
                    .color(FG_SECONDARY),
            );
        }

        // ── Right: window control buttons ───────────────────────────────
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(4.0);

            // Close — red hover
            if win_btn(
                ui,
                "✕",
                TITLE_BAR_HEIGHT,
                Some(Color32::from_rgb(196, 43, 28)),
                Some(Color32::WHITE),
            )
            .clicked()
            {
                close = true;
            }

            // Maximize / Restore
            let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
            let max_icon = if is_maximized { "❐" } else { "□" };
            if win_btn(ui, max_icon, TITLE_BAR_HEIGHT, None, None).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
            }

            // Minimize
            if win_btn(ui, "─", TITLE_BAR_HEIGHT, None, None).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
        });
    });

    close
}

/// Single window-control button (close / max / min).
fn win_btn(
    ui: &mut egui::Ui,
    icon: &str,
    height: f32,
    hover_fill: Option<Color32>,
    hover_fg: Option<Color32>,
) -> egui::Response {
    let w = 46.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, height), egui::Sense::click());

    let is_hovered = resp.hovered();
    let bg = if is_hovered {
        hover_fill.unwrap_or(BG_ELEVATED)
    } else {
        Color32::TRANSPARENT
    };
    let fg = if is_hovered {
        hover_fg.unwrap_or(FG_PRIMARY)
    } else {
        FG_SECONDARY
    };

    ui.painter().rect_filled(rect, 0.0, bg);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        FontId::proportional(13.0),
        fg,
    );

    resp
}
