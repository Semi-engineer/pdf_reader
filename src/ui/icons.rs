/*!
Icon System
Centralized icon definitions (Unicode symbols) and programmatic icon drawing.

All UI files should import icons from this module rather than using
inline strings.  A future PR may migrate to SVG/rasterized icons.
*/

use eframe::egui::{self, Color32, Stroke, Vec2};

// ═══════════════════════════════════════════════════════════════════════════════
// ACTIVITY BAR
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_THUMBNAILS:   &str = "▦";
pub const ICON_SEARCH:       &str = "⚲";
pub const ICON_OUTLINE:      &str = "≡";
pub const ICON_BOOKMARKS:    &str = "◈";
pub const ICON_ANNOTATIONS:  &str = "✎";
pub const ICON_ATTACHMENTS:  &str = "⚓";
pub const ICON_SETTINGS:     &str = "⚙";

// ═══════════════════════════════════════════════════════════════════════════════
// TOOLBAR — FILE
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_OPEN:    &str = "📂";
pub const ICON_SAVE:    &str = "💾";
pub const ICON_PRINT:   &str = "🖨";
pub const ICON_EXPORT:  &str = "📤";

// ═══════════════════════════════════════════════════════════════════════════════
// TOOLBAR — NAVIGATION
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_PREV:         &str = "◀";
pub const ICON_NEXT:         &str = "▶";
pub const ICON_PREV_RESULT:  &str = "▲";
pub const ICON_NEXT_RESULT:  &str = "▼";

// ═══════════════════════════════════════════════════════════════════════════════
// TOOLBAR — VIEW
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_ZOOM_IN:      &str = "+";
pub const ICON_ZOOM_OUT:     &str = "−";
pub const ICON_FIT_PAGE:     &str = "⊡";
pub const ICON_FIT_WIDTH:    &str = "⊟";
pub const ICON_ROTATE_LEFT:  &str = "↺";
pub const ICON_ROTATE_RIGHT: &str = "↻";

// ═══════════════════════════════════════════════════════════════════════════════
// TOOLBAR — ANNOTATION TOOLS
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_SELECT:       &str = "↖";
pub const ICON_HIGHLIGHT:    &str = "🖍";
pub const ICON_RECTANGLE:    &str = "▭";
pub const ICON_CIRCLE:       &str = "○";
pub const ICON_LINE:         &str = "╱";
pub const ICON_ARROW:        &str = "➜";
pub const ICON_PEN:          &str = "✎";
pub const ICON_TEXT:         &str = "📝";

// ═══════════════════════════════════════════════════════════════════════════════
// TOOLBAR — SEARCH
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_SEARCH_DOC:   &str = "🔍";
pub const ICON_CLEAR:        &str = "✕";

// ═══════════════════════════════════════════════════════════════════════════════
// WINDOW CONTROLS
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_MINIMIZE:     &str = "─";
pub const ICON_MAXIMIZE:     &str = "◻";
pub const ICON_RESTORE:      &str = "◱";
pub const ICON_CLOSE:        &str = "✕";

// ═══════════════════════════════════════════════════════════════════════════════
// INSPECTOR / SIDEBAR
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_INSPECTOR:    &str = "ℹ";
pub const ICON_PROPERTIES:   &str = "⚙";
pub const ICON_PAGE_INFO:    &str = "📄";
pub const ICON_METADATA:     &str = "ℹ";
pub const ICON_ANNOTATION_INSPECT: &str = "🔎";

// ═══════════════════════════════════════════════════════════════════════════════
// STATUS BAR
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_FILE:         &str = "📄";
pub const ICON_CACHE:        &str = "💾";
pub const ICON_RENDERER:     &str = "⚡";
pub const ICON_TOOL:         &str = "🖊";

// ═══════════════════════════════════════════════════════════════════════════════
// MISC
// ═══════════════════════════════════════════════════════════════════════════════

pub const ICON_COLOR:        &str = "●";
pub const ICON_DELETE:       &str = "✖";
pub const ICON_TRASH:        &str = "🗑";
pub const ICON_COPY:         &str = "⎘";
pub const ICON_CHECK:        &str = "✓";
pub const ICON_PLUS:         &str = "+";
pub const ICON_MINUS:        &str = "−";
pub const ICON_MORE:         &str = "⋮";
pub const ICON_REFRESH:      &str = "↻";
pub const ICON_EMPTY:        &str = "▣";

// ═══════════════════════════════════════════════════════════════════════════════
// PROGRAMMATIC ICON DRAWING
// ═══════════════════════════════════════════════════════════════════════════════

/// Draw a document/file icon (page with folded corner)
pub fn draw_document_icon(ui: &mut egui::Ui, rect: egui::Rect, color: Color32) {
    let painter = ui.painter();
    let padding = rect.width() * 0.15;
    let icon_rect = rect.shrink(padding);

    // Document body with folded corner
    let fold_size = rect.width() * 0.2;
    let points = vec![
        icon_rect.left_top(),
        egui::pos2(icon_rect.right() - fold_size, icon_rect.top()),
        icon_rect.right_top() + egui::vec2(0.0, fold_size),
        icon_rect.right_bottom(),
        icon_rect.left_bottom(),
    ];

    painter.add(egui::Shape::convex_polygon(
        points,
        color,
        Stroke::new(1.0, color.gamma_multiply(0.7)),
    ));

    // Folded corner
    let fold_points = vec![
        egui::pos2(icon_rect.right() - fold_size, icon_rect.top()),
        egui::pos2(icon_rect.right() - fold_size, icon_rect.top() + fold_size),
        icon_rect.right_top() + egui::vec2(0.0, fold_size),
    ];
    painter.add(egui::Shape::convex_polygon(
        fold_points,
        color.gamma_multiply(0.6),
        Stroke::NONE,
    ));
}

/// Draw a search/magnifying glass icon
pub fn draw_search_icon(ui: &mut egui::Ui, rect: egui::Rect, color: Color32) {
    let painter = ui.painter();
    let center = rect.center() - egui::vec2(rect.width() * 0.1, rect.height() * 0.1);
    let radius = rect.width() * 0.28;

    // Circle
    painter.circle_stroke(center, radius, Stroke::new(1.8, color));

    // Handle
    let handle_start = center + egui::vec2(radius * 0.7, radius * 0.7);
    let handle_end = handle_start + egui::vec2(radius * 0.6, radius * 0.6);
    painter.line_segment([handle_start, handle_end], Stroke::new(1.8, color));
}

/// Draw a folder icon
pub fn draw_folder_icon(ui: &mut egui::Ui, rect: egui::Rect, color: Color32) {
    let painter = ui.painter();
    let padding = rect.width() * 0.15;
    let icon_rect = rect.shrink(padding);

    let tab_width = icon_rect.width() * 0.35;
    let tab_height = icon_rect.height() * 0.2;

    // Folder body
    painter.rect_filled(
        egui::Rect::from_min_max(
            icon_rect.left_top() + egui::vec2(0.0, tab_height),
            icon_rect.right_bottom(),
        ),
        2.0,
        color,
    );

    // Folder tab
    painter.rect_filled(
        egui::Rect::from_min_size(
            icon_rect.left_top(),
            egui::vec2(tab_width, tab_height),
        ),
        2.0,
        color.gamma_multiply(0.8),
    );
}

/// Draw a minimalist PDF document icon (outline style for title bar / logo)
pub fn draw_pdf_icon(ui: &mut egui::Ui, size: Vec2) {
    use crate::ui::theme::FG_ACCENT;

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
