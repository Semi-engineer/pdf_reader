/*!
Custom Title Bar — Frameless window with modern styling
Matches Industrial Minimal design language
*/

use super::icons;
use super::theme::*;
use eframe::egui::{self, Color32, Stroke, Vec2};

/// Renders the custom title bar and returns whether the window should close.
pub fn show(ctx: &egui::Context, doc_name: Option<&str>) -> bool {
    let mut should_close = false;

    egui::TopBottomPanel::top("title_bar")
        .exact_height(TITLE_BAR_HEIGHT)
        .frame(egui::Frame::new()
            .fill(BG_BASE)
            .stroke(Stroke::new(1.0, BORDER)))
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

    // Paint background (already set by frame)
    // Bottom separator line removed for cleaner look

    ui.horizontal(|ui| {
        ui.set_min_height(TITLE_BAR_HEIGHT);
        ui.spacing_mut().item_spacing.x = 0.0;

        // ── Left: icon + title ────────────────────────────────────────────
        ui.add_space(12.0);

        // Modern minimalist PDF icon
        icons::draw_pdf_icon(ui, Vec2::new(18.0, 18.0));

        ui.add_space(10.0);

        ui.label(
            egui::RichText::new("DocLens")
                .size(FONT_SIZE_UI)
                .color(FG_PRIMARY)
                .strong(),
        );

        // Document name
        if let Some(name) = doc_name {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("—")
                    .size(FONT_SIZE_SMALL)
                    .color(BORDER_STRONG),
            );
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(name)
                    .size(FONT_SIZE_SMALL)
                    .color(FG_SECONDARY),
            );
        }

        // ── Right: window control buttons ─────────────────────────────────
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Close — red hover
            if win_btn(
                ui,
                icons::ICON_CLOSE,
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
            let max_icon = if is_maximized { icons::ICON_RESTORE } else { icons::ICON_MAXIMIZE };
            if win_btn(ui, max_icon, TITLE_BAR_HEIGHT, None, None).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
            }

            // Minimize
            if win_btn(ui, icons::ICON_MINIMIZE, TITLE_BAR_HEIGHT, None, None).clicked() {
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
        hover_fill.unwrap_or(BG_HOVER)
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
        egui::FontId::proportional(FONT_SIZE_UI),
        fg,
    );

    resp
}
