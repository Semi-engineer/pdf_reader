/*!
Custom Title Bar — Frameless window with modern styling
Matches Industrial Minimal design language
*/

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
        draw_pdf_icon(ui, Vec2::new(18.0, 18.0));

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
            let max_icon = if is_maximized { "◱" } else { "◻" };
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

/// Draw a modern minimalist PDF document icon
fn draw_pdf_icon(ui: &mut egui::Ui, size: Vec2) {
    let (rect, _resp) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    
    let icon_color = FG_ACCENT;
    let fold_color = FG_ACCENT.gamma_multiply(0.7);
    
    let padding = size.x * 0.12;
    let icon_rect = rect.shrink(padding);
    
    // Main document body
    let corner_size = size.x * 0.22;
    let points = vec![
        icon_rect.left_top(),
        egui::pos2(icon_rect.right() - corner_size, icon_rect.top()),
        icon_rect.right_top() + egui::vec2(0.0, corner_size),
        icon_rect.right_bottom(),
        icon_rect.left_bottom(),
    ];
    
    painter.add(egui::Shape::convex_polygon(
        points,
        Color32::TRANSPARENT,
        Stroke::new(1.5, icon_color),
    ));
    
    // Folded corner
    let fold_points = vec![
        egui::pos2(icon_rect.right() - corner_size, icon_rect.top()),
        egui::pos2(icon_rect.right() - corner_size, icon_rect.top() + corner_size),
        icon_rect.right_top() + egui::vec2(0.0, corner_size),
    ];
    painter.add(egui::Shape::convex_polygon(
        fold_points,
        fold_color,
        Stroke::NONE,
    ));
    
    // Two horizontal lines representing text
    let line_width = icon_rect.width() * 0.55;
    let line_x = icon_rect.center().x - line_width * 0.5;
    let start_y = icon_rect.center().y - 1.0;
    
    for i in 0..2 {
        let y = start_y + (i as f32) * 4.0;
        painter.line_segment(
            [egui::pos2(line_x, y), egui::pos2(line_x + line_width, y)],
            Stroke::new(1.0, icon_color),
        );
    }
}
