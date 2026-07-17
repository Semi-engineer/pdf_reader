/*!
DocLens Theme
Centralised colour palette and style setup.
*/

use eframe::egui::{self, Color32, FontId, Stroke, Vec2, Visuals};

// ═══════════════════════════════════════════════════════════════════════════════
// THEME SYSTEM — Industrial Minimal Design Language
// ═══════════════════════════════════════════════════════════════════════════════
//
// Design Philosophy:
//   • Industrial Minimal aesthetic
//   • High information density
//   • Flat surfaces with thin borders
//   • Neutral gray palette with blue/orange accents
//   • No gradients, no glassmorphism
//   • Minimal corner radius (2-4px)
//
// ═══════════════════════════════════════════════════════════════════════════════

// ─── Background Semantic Tokens ──────────────────────────────────────────────

/// Base canvas — deepest background (window, central workspace)
pub const BG_BASE: Color32        = Color32::from_rgb(24,  24,  28);   

/// Surface elevation — panels, sidebars, activity bar
pub const BG_SURFACE: Color32     = Color32::from_rgb(32,  32,  38);   

/// Elevated controls — buttons, inputs, cards
pub const BG_ELEVATED: Color32    = Color32::from_rgb(42,  42,  50);   

/// Hover state — interactive element hover
pub const BG_HOVER: Color32       = Color32::from_rgb(52,  52,  62);   

/// Active/Selected — primary accent blue
pub const BG_ACTIVE: Color32      = Color32::from_rgb(60, 120, 216);   

/// Active dim — semi-transparent active background
pub static BG_ACTIVE_DIM: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(60, 120, 216, 40));

// ─── Foreground Semantic Tokens ──────────────────────────────────────────────

/// Primary text — high contrast, main content
pub const FG_PRIMARY: Color32     = Color32::from_rgb(235, 235, 242);  

/// Secondary text — lower contrast, supporting content
pub const FG_SECONDARY: Color32   = Color32::from_rgb(160, 165, 180);  

/// Tertiary text — lowest contrast, hints and placeholders
pub const FG_TERTIARY: Color32    = Color32::from_rgb(120, 125, 140);  

/// Accent — interactive elements, links, highlights
pub const FG_ACCENT: Color32      = Color32::from_rgb(80, 150, 255);   

/// Success — positive states, confirmations
pub const FG_SUCCESS: Color32     = Color32::from_rgb(80, 200, 120);   

/// Warning — caution, pending actions (orange)
pub const FG_WARNING: Color32     = Color32::from_rgb(255, 170,  60);  

/// Error — destructive actions, errors
pub const FG_ERROR: Color32       = Color32::from_rgb(255,  90,  90);  

// ─── Border Semantic Tokens ──────────────────────────────────────────────────

/// Default border — panel separators, subtle divisions
pub const BORDER: Color32         = Color32::from_rgb(48,  48,  58);   

/// Focus border — active input focus ring
pub const BORDER_FOCUS: Color32   = Color32::from_rgb(80, 150, 255);   

/// Strong border — emphasized separations
pub const BORDER_STRONG: Color32  = Color32::from_rgb(64,  64,  74);

// ─── Specialized Semantic Tokens ─────────────────────────────────────────────

/// Selection background — text/element selection
pub static SELECTION_BG: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(60, 120, 216, 100));

/// Search match background — search results highlight
pub static SEARCH_BG: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(255, 200, 0, 130));

/// Current search match — active search result
pub static SEARCH_CURRENT: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(255, 150, 30, 220));

// ─── Layout Constants ─────────────────────────────────────────────────────────

/// Activity bar width (vertical left ribbon)
pub const ACTIVITY_BAR_WIDTH: f32 = 48.0;

/// Titlebar height (custom frameless)
pub const TITLE_BAR_HEIGHT: f32 = 32.0;

/// Toolbar height (main toolbar below titlebar)
pub const TOOLBAR_HEIGHT: f32 = 38.0;

/// Status bar height (bottom info bar)
pub const STATUS_BAR_HEIGHT: f32 = 24.0;

/// Sidebar minimum width
pub const SIDEBAR_MIN_WIDTH: f32 = 200.0;

/// Sidebar default width
pub const SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;

/// Inspector panel default width
pub const INSPECTOR_DEFAULT_WIDTH: f32 = 280.0;

// ─── Typography Scale ─────────────────────────────────────────────────────────

/// UI font size — menus, toolbars, panels
pub const FONT_SIZE_UI: f32 = 13.0;

/// Body font size — primary content
pub const FONT_SIZE_BODY: f32 = 13.5;

/// Small font size — status bar, metadata
pub const FONT_SIZE_SMALL: f32 = 12.0;

/// Tiny font size — annotations, hints
pub const FONT_SIZE_TINY: f32 = 11.0;

/// Heading font size
pub const FONT_SIZE_HEADING: f32 = 14.5;

// ═══════════════════════════════════════════════════════════════════════════════
// APPLY THEME
// ═══════════════════════════════════════════════════════════════════════════════

pub fn apply(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // ── Window ────────────────────────────────────────────────────────────────
    visuals.window_fill = BG_BASE;
    visuals.window_stroke = Stroke::new(1.0, BORDER);
    visuals.window_corner_radius = egui::CornerRadius::same(4);
    visuals.window_shadow = egui::epaint::Shadow::NONE; // Flat, no shadows

    // ── Panel ─────────────────────────────────────────────────────────────────
    visuals.panel_fill = BG_SURFACE;

    // ── Backgrounds ───────────────────────────────────────────────────────────
    visuals.extreme_bg_color = BG_BASE;
    visuals.faint_bg_color = BG_BASE;

    // ── Widgets — Industrial Minimal Style ────────────────────────────────────
    
    // Noninteractive (static elements)
    visuals.widgets.noninteractive.bg_fill    = BG_ELEVATED;
    visuals.widgets.noninteractive.bg_stroke  = Stroke::new(1.0, BORDER);
    visuals.widgets.noninteractive.fg_stroke  = Stroke::new(1.0, FG_SECONDARY);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);

    // Inactive (default clickable state)
    visuals.widgets.inactive.bg_fill    = BG_ELEVATED;
    visuals.widgets.inactive.bg_stroke  = Stroke::new(1.0, BORDER);
    visuals.widgets.inactive.fg_stroke  = Stroke::new(1.0, FG_PRIMARY);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(2);

    // Hovered
    visuals.widgets.hovered.bg_fill    = BG_HOVER;
    visuals.widgets.hovered.bg_stroke  = Stroke::new(1.0, BORDER_FOCUS);
    visuals.widgets.hovered.fg_stroke  = Stroke::new(1.0, FG_PRIMARY);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(2);

    // Active (pressed/selected)
    visuals.widgets.active.bg_fill    = BG_ACTIVE;
    visuals.widgets.active.bg_stroke  = Stroke::NONE;
    visuals.widgets.active.fg_stroke  = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(2);

    // Open (dropdown/combo open)
    visuals.widgets.open.bg_fill    = BG_ACTIVE;
    visuals.widgets.open.bg_stroke  = Stroke::NONE;
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(2);

    // ── Selection ─────────────────────────────────────────────────────────────
    visuals.selection.bg_fill = BG_ACTIVE;
    visuals.selection.stroke  = Stroke::new(1.0, BORDER_FOCUS);

    // ── Text Override ─────────────────────────────────────────────────────────
    visuals.override_text_color = Some(FG_PRIMARY);

    // ── Hyperlinks ────────────────────────────────────────────────────────────
    visuals.hyperlink_color = FG_ACCENT;

    ctx.set_visuals(visuals);

    // ── Spacing & Sizing ──────────────────────────────────────────────────────
    let mut style = (*ctx.style()).clone();
    
    // Compact industrial spacing
    style.spacing.item_spacing   = Vec2::new(6.0, 4.0);
    style.spacing.button_padding = Vec2::new(8.0, 4.0);
    style.spacing.window_margin  = egui::Margin::same(8);
    style.spacing.menu_margin    = egui::Margin::same(4);
    style.spacing.indent         = 16.0;
    style.spacing.interact_size  = Vec2::new(32.0, 22.0);
    style.spacing.slider_width   = 160.0;

    // ── Font System ───────────────────────────────────────────────────────────
    setup_fonts(ctx);

    // Typography scale
    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(FONT_SIZE_BODY, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(FONT_SIZE_UI, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(FONT_SIZE_HEADING, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        FontId::new(FONT_SIZE_SMALL, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        FontId::new(FONT_SIZE_SMALL, egui::FontFamily::Monospace),
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
