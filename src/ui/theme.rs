/*!
DocLens Theme
Centralised colour palette and style setup.
*/

use eframe::egui::{self, Color32, FontId, Stroke, Vec2, Visuals};

// ─── Palette ──────────────────────────────────────────────────────────────────
// Modern dark theme with blue-purple accent

pub const BG_BASE: Color32        = Color32::from_rgb(18,  18,  24);   // Window / panel bg - darker for better contrast
pub const BG_SURFACE: Color32     = Color32::from_rgb(28,  28,  36);   // Toolbar / sidebar - subtle elevation
pub const BG_ELEVATED: Color32    = Color32::from_rgb(38,  38,  48);   // Buttons, inputs - clear hierarchy
pub const BG_HOVER: Color32       = Color32::from_rgb(52,  52,  68);   // Button hover - noticeable but smooth
pub const BG_ACTIVE: Color32      = Color32::from_rgb(88, 112, 214);   // Selected / active - vibrant blue-purple

// from_rgba_unmultiplied is not const in egui 0.31 — use a lazy static
pub static BG_ACTIVE_DIM: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(88, 112, 214, 50));

pub const FG_PRIMARY: Color32     = Color32::from_rgb(240, 240, 248);  // Normal text - crisp white
pub const FG_SECONDARY: Color32   = Color32::from_rgb(156, 163, 185);  // Dimmed text - better readability
pub const FG_ACCENT: Color32      = Color32::from_rgb(108, 182, 255);  // Accent / links - bright blue
pub const FG_SUCCESS: Color32     = Color32::from_rgb(75,  210, 143);  // Success - fresh green
pub const FG_WARNING: Color32     = Color32::from_rgb(255, 188,  66);  // Warning - warm amber
pub const FG_ERROR: Color32       = Color32::from_rgb(255,  85,  85);  // Error - clear red

pub const BORDER: Color32         = Color32::from_rgb(52,  52,  68);   // Borders - subtle separation
pub const BORDER_FOCUS: Color32   = Color32::from_rgb(108, 182, 255);  // Focus - matches accent

pub static SELECTION_BG: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(88, 112, 214, 85));
pub static SEARCH_BG: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(255, 215, 0, 120));
pub static SEARCH_CURRENT: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(255, 150, 30, 200));

// ─── Apply theme ─────────────────────────────────────────────────────────────

pub fn apply(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // Window
    visuals.window_fill = BG_BASE;
    visuals.window_stroke = Stroke::new(1.0, BORDER);
    visuals.window_corner_radius = egui::CornerRadius::same(6);

    // Panel - all panels use surface color
    visuals.panel_fill = BG_SURFACE;

    // Extreme (scroll area etc.) - use base for backgrounds
    visuals.extreme_bg_color = BG_BASE;
    
    // Faint background (for central panel and viewer area)
    visuals.faint_bg_color = BG_BASE;

    // Widgets — normal state
    visuals.widgets.noninteractive.bg_fill    = BG_ELEVATED;
    visuals.widgets.noninteractive.bg_stroke  = Stroke::new(1.0, BORDER);
    visuals.widgets.noninteractive.fg_stroke  = Stroke::new(1.0, FG_SECONDARY);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(4);

    // Widgets — inactive (hoverable)
    visuals.widgets.inactive.bg_fill    = BG_ELEVATED;
    visuals.widgets.inactive.bg_stroke  = Stroke::new(1.0, BORDER);
    visuals.widgets.inactive.fg_stroke  = Stroke::new(1.5, FG_PRIMARY);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);

    // Widgets — hovered
    visuals.widgets.hovered.bg_fill    = BG_HOVER;
    visuals.widgets.hovered.bg_stroke  = Stroke::new(1.0, BORDER_FOCUS);
    visuals.widgets.hovered.fg_stroke  = Stroke::new(1.5, FG_PRIMARY);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);

    // Widgets — active (pressed)
    visuals.widgets.active.bg_fill    = BG_ACTIVE;
    visuals.widgets.active.bg_stroke  = Stroke::NONE;
    visuals.widgets.active.fg_stroke  = Stroke::new(1.5, Color32::WHITE);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);

    // Widgets — open (combo/drop-down open)
    visuals.widgets.open.bg_fill    = BG_ACTIVE;
    visuals.widgets.open.bg_stroke  = Stroke::NONE;
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(4);

    // Selection
    visuals.selection.bg_fill = BG_ACTIVE;
    visuals.selection.stroke  = Stroke::new(1.0, BORDER_FOCUS);

    // Override
    visuals.override_text_color = Some(FG_PRIMARY);

    ctx.set_visuals(visuals);

    // Spacing / sizing
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing   = Vec2::new(6.0, 4.0);
    style.spacing.button_padding = Vec2::new(8.0, 4.0);
    style.spacing.window_margin  = egui::Margin::same(10);
    style.spacing.menu_margin    = egui::Margin::same(6);
    style.spacing.indent         = 14.0;
    style.spacing.interact_size  = Vec2::new(36.0, 24.0);

    // ── Fonts: Segoe UI (Thai/Unicode) + Segoe UI Emoji + Symbol fallback ───
    setup_fonts(ctx);

    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(13.5, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(13.5, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(15.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        FontId::new(11.5, egui::FontFamily::Proportional),
    );

    ctx.set_style(style);
}

// ─── Font setup ───────────────────────────────────────────────────────────────

/// Load system fonts with Unicode/Thai/Emoji support.
///
/// Uses Segoe UI (Thai + Latin + most scripts) and Segoe UI Symbol
/// (arrows, math, box-drawing) from Windows system fonts.
///
/// NOTE: egui's font renderer (ab_glyph) only supports outline/TrueType fonts.
/// Colour emoji fonts (COLR, CBDT, SVG) are NOT supported and will panic.
/// We therefore skip seguiemj.ttf (colour emoji) entirely.
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Font candidates — ordered by preference.
    // Leelawadee UI is designed specifically for Thai and uses pre-composed
    // glyphs that work with ab_glyph's simple rendering (no GSUB shaping needed).
    // Segoe UI has Thai glyphs but relies more on OpenType shaping.
    let candidates: &[(&str, &str)] = &[
        ("LeelawUI",    "C:\\Windows\\Fonts\\LeelawUI.ttf"),   // Leelawadee UI — best Thai
        ("SegoeUI",     "C:\\Windows\\Fonts\\segoeui.ttf"),    // Segoe UI fallback
        ("Tahoma",      "C:\\Windows\\Fonts\\tahoma.ttf"),     // Tahoma — old but has Thai
        ("SegoeSymbol", "C:\\Windows\\Fonts\\seguisym.ttf"),   // Symbols / arrows
    ];

    let mut loaded: Vec<String> = Vec::new();
    for (name, path) in candidates {
        match std::fs::read(path) {
            Ok(bytes) => {
                fonts.font_data.insert(
                    (*name).into(),
                    egui::FontData::from_owned(bytes).into(),
                );
                loaded.push((*name).into());
            }
            Err(_) => {}
        }
    }

    // Build Proportional chain: our fonts first, then egui built-ins
    let prop = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    let existing: Vec<String> = prop.drain(..).collect();
    prop.extend(loaded.clone());
    prop.extend(existing);

    // Monospace too (for path labels etc.)
    let mono = fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default();
    for name in &loaded {
        if !mono.contains(name) {
            mono.push(name.clone());
        }
    }

    ctx.set_fonts(fonts);
}

// ─── Helper: icon button ──────────────────────────────────────────────────────

/// A compact square button with no border when inactive.
pub fn icon_btn(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(icon).size(14.0))
        .min_size(Vec2::splat(28.0))
        .frame(true);
    ui.add(btn).on_hover_text(tooltip)
}

/// A toggled icon button (highlighted when active).
pub fn icon_toggle_btn(
    ui: &mut egui::Ui,
    icon: &str,
    tooltip: &str,
    active: bool,
) -> egui::Response {
    let text = if active {
        egui::RichText::new(icon).size(14.0).color(Color32::WHITE)
    } else {
        egui::RichText::new(icon).size(14.0).color(FG_SECONDARY)
    };
    let btn = egui::Button::new(text)
        .min_size(Vec2::splat(28.0))
        .fill(if active { BG_ACTIVE } else { Color32::TRANSPARENT })
        .frame(true);
    ui.add(btn).on_hover_text(tooltip)
}

// ─── SVG-style Icon Drawing Functions ─────────────────────────────────────────

/// Draw a document/file icon
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
        egui::Stroke::new(1.0, color.gamma_multiply(0.7)),
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
        egui::Stroke::NONE,
    ));
}

/// Draw a search/magnifying glass icon
pub fn draw_search_icon(ui: &mut egui::Ui, rect: egui::Rect, color: Color32) {
    let painter = ui.painter();
    let center = rect.center() - egui::vec2(rect.width() * 0.1, rect.height() * 0.1);
    let radius = rect.width() * 0.28;
    
    // Circle
    painter.circle_stroke(center, radius, egui::Stroke::new(1.8, color));
    
    // Handle
    let handle_start = center + egui::vec2(radius * 0.7, radius * 0.7);
    let handle_end = handle_start + egui::vec2(radius * 0.6, radius * 0.6);
    painter.line_segment([handle_start, handle_end], egui::Stroke::new(1.8, color));
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
