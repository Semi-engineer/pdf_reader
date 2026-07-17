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
        ui.add_space(10.0);

        // Modern PDF icon
        draw_pdf_icon(ui, Vec2::new(20.0, 20.0));

        ui.add_space(8.0);

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
                "×",  // More modern × symbol
                TITLE_BAR_HEIGHT,
                Some(Color32::from_rgb(196, 43, 28)),
                Some(Color32::WHITE),
            )
            .clicked()
            {
                close = true;
            }

            // Maximize / Restore - using modern icons
            let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
            let max_icon = if is_maximized { "◱" } else { "◻" };  // More modern symbols
            if win_btn(ui, max_icon, TITLE_BAR_HEIGHT, None, None).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
            }

            // Minimize - using horizontal line
            if win_btn(ui, "−", TITLE_BAR_HEIGHT, None, None).clicked() {
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

/// Draw a modern PDF document icon (SVG-style)
fn draw_pdf_icon(ui: &mut egui::Ui, size: Vec2) {
    let (rect, _resp) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    
    let icon_color = Color32::from_rgb(108, 182, 255); // Accent blue
    let fold_color = Color32::from_rgb(70, 130, 200);  // Darker blue
    
    let padding = size.x * 0.15;
    let icon_rect = rect.shrink(padding);
    
    // Main document body
    let points = vec![
        icon_rect.left_top(),
        egui::pos2(icon_rect.right() - size.x * 0.25, icon_rect.top()),
        icon_rect.right_top() + egui::vec2(0.0, size.y * 0.25),
        icon_rect.right_bottom(),
        icon_rect.left_bottom(),
    ];
    
    painter.add(egui::Shape::convex_polygon(
        points,
        icon_color,
        Stroke::new(1.2, icon_color.gamma_multiply(0.8)),
    ));
    
    // Folded corner
    let fold_points = vec![
        egui::pos2(icon_rect.right() - size.x * 0.25, icon_rect.top()),
        egui::pos2(icon_rect.right() - size.x * 0.25, icon_rect.top() + size.y * 0.25),
        icon_rect.right_top() + egui::vec2(0.0, size.y * 0.25),
    ];
    painter.add(egui::Shape::convex_polygon(
        fold_points,
        fold_color,
        Stroke::NONE,
    ));
    
    // PDF text lines (3 horizontal lines)
    let text_color = Color32::WHITE;
    let line_width = icon_rect.width() * 0.6;
    let line_x = icon_rect.center().x - line_width * 0.5;
    let start_y = icon_rect.center().y - size.y * 0.1;
    
    for i in 0..3 {
        let y = start_y + (i as f32) * size.y * 0.15;
        painter.line_segment(
            [egui::pos2(line_x, y), egui::pos2(line_x + line_width, y)],
            Stroke::new(1.0, text_color),
        );
    }
}
