/*!
DocLens Design System
Centralised theme tokens, typography, spacing, radii, and style setup.

Design Philosophy:
  • Industrial Minimal aesthetic
  • High information density
  • Flat surfaces with thin borders
  • Neutral gray palette with blue/orange accents
  • No gradients, no glassmorphism
  • Consistent spacing on an 8-point grid
*/

use eframe::egui::{self, Color32, FontId, Stroke, Vec2, Visuals};

// ═══════════════════════════════════════════════════════════════════════════════
// SEMANTIC COLOR TOKENS
// ═══════════════════════════════════════════════════════════════════════════════

// ─── Background Tokens ───────────────────────────────────────────────────────

/// Base canvas — deepest background (window chrome, title bar)
pub const BG_BASE: Color32        = Color32::from_rgb(24,  24,  28);

/// Workspace canvas — slightly warmer than base, for document viewing area
pub const BG_WORKSPACE: Color32   = Color32::from_rgb(28,  28,  32);

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

// ─── Foreground Tokens ───────────────────────────────────────────────────────

/// Primary text — high contrast, main content
pub const FG_PRIMARY: Color32     = Color32::from_rgb(235, 235, 242);

/// Secondary text — lower contrast, supporting content
pub const FG_SECONDARY: Color32   = Color32::from_rgb(160, 165, 180);

/// Tertiary text — lowest contrast, hints and placeholders
pub const FG_TERTIARY: Color32    = Color32::from_rgb(120, 125, 140);

/// Disabled text — non-interactive elements
pub const FG_DISABLED: Color32    = Color32::from_rgb(80,  84,  96);

/// Accent — interactive elements, links, highlights
pub const FG_ACCENT: Color32      = Color32::from_rgb(80, 150, 255);

/// Success — positive states, confirmations
pub const FG_SUCCESS: Color32     = Color32::from_rgb(80, 200, 120);

/// Warning — caution, pending actions (orange)
pub const FG_WARNING: Color32     = Color32::from_rgb(255, 170,  60);

/// Error — destructive actions, errors
pub const FG_ERROR: Color32       = Color32::from_rgb(255,  90,  90);

// ─── Border Tokens ───────────────────────────────────────────────────────────

/// Default border — panel separators, subtle divisions
pub const BORDER: Color32         = Color32::from_rgb(48,  48,  58);

/// Focus border — active input focus ring
pub const BORDER_FOCUS: Color32   = Color32::from_rgb(80, 150, 255);

/// Strong border — emphasized separations
pub const BORDER_STRONG: Color32  = Color32::from_rgb(64,  64,  74);

// ─── Specialized Tokens ──────────────────────────────────────────────────────

/// Selection background — text/element selection
pub static SELECTION_BG: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(60, 120, 216, 100));

/// Search match background — search results highlight
pub static SEARCH_BG: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(255, 200, 0, 130));

/// Current search match — active search result
pub static SEARCH_CURRENT: std::sync::LazyLock<Color32> =
    std::sync::LazyLock::new(|| Color32::from_rgba_unmultiplied(255, 150, 30, 220));

// ═══════════════════════════════════════════════════════════════════════════════
// SPACING SYSTEM — 8-point grid
// ═══════════════════════════════════════════════════════════════════════════════

/// 4px — tightest spacing (inline gaps, icon padding)
pub const SP_XS: f32  = 4.0;
/// 8px — default small spacing (between related items)
pub const SP_SM: f32  = 8.0;
/// 12px — medium spacing (between groups)
pub const SP_MD: f32  = 12.0;
/// 16px — large spacing (section padding)
pub const SP_LG: f32  = 16.0;
/// 24px — extra large spacing (major separations)
pub const SP_XL: f32  = 24.0;
/// 32px — maximum spacing (top-level sections)
pub const SP_XXL: f32 = 32.0;

// ═══════════════════════════════════════════════════════════════════════════════
// CORNER RADIUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Small radius — buttons, inputs, tags
pub const RADIUS_SM: f32 = 4.0;
/// Medium radius — cards, panels, dropdowns
pub const RADIUS_MD: f32 = 6.0;
/// Large radius — modals, dialogs
pub const RADIUS_LG: f32 = 8.0;

// ═══════════════════════════════════════════════════════════════════════════════
// TYPOGRAPHY SCALE
// ═══════════════════════════════════════════════════════════════════════════════

/// Heading — page titles, modal headers (16px)
pub const FONT_SIZE_HEADING: f32 = 16.0;
/// Section — panel titles, group labels (14px)
pub const FONT_SIZE_SECTION: f32 = 14.0;
/// Body / UI — primary content, menus, buttons (13px)
pub const FONT_SIZE_BODY: f32    = 13.0;
/// Caption — status bar, metadata, tooltips (12px)
pub const FONT_SIZE_CAPTION: f32 = 12.0;
/// Tiny — annotations hints, badges (11px)
pub const FONT_SIZE_TINY: f32    = 11.0;

// Aliases for backward compatibility
pub const FONT_SIZE_UI: f32    = FONT_SIZE_BODY;
pub const FONT_SIZE_SMALL: f32 = FONT_SIZE_CAPTION;

// ═══════════════════════════════════════════════════════════════════════════════
// LAYOUT CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

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

/// Icon button size (square)
pub const ICON_BTN_SIZE: f32 = 28.0;

/// Activity bar button size (square)
pub const ACTIVITY_BTN_SIZE: f32 = 40.0;

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE PRESENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Subtle drop shadow for rendered PDF pages
pub fn page_shadow() -> egui::epaint::Shadow {
    egui::epaint::Shadow {
        offset: [0, 2],
        blur: 8,
        spread: 0,
        color: Color32::from_black_alpha(50),
    }
}

/// Border stroke around rendered PDF pages
pub fn page_border() -> Stroke {
    Stroke::new(1.0, Color32::from_black_alpha(40))
}

// ═══════════════════════════════════════════════════════════════════════════════
// APPLY THEME
// ═══════════════════════════════════════════════════════════════════════════════

pub fn apply(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();

    // ── Window ────────────────────────────────────────────────────────────────
    visuals.window_fill = BG_BASE;
    visuals.window_stroke = Stroke::new(1.0, BORDER);
    visuals.window_corner_radius = egui::CornerRadius::same(RADIUS_SM as u8);
    visuals.window_shadow = egui::epaint::Shadow::NONE; // Flat, no window shadows

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
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(RADIUS_SM as u8);

    // Inactive (default clickable state)
    visuals.widgets.inactive.bg_fill    = BG_ELEVATED;
    visuals.widgets.inactive.bg_stroke  = Stroke::new(1.0, BORDER);
    visuals.widgets.inactive.fg_stroke  = Stroke::new(1.0, FG_PRIMARY);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(RADIUS_SM as u8);

    // Hovered
    visuals.widgets.hovered.bg_fill    = BG_HOVER;
    visuals.widgets.hovered.bg_stroke  = Stroke::new(1.0, BORDER_FOCUS);
    visuals.widgets.hovered.fg_stroke  = Stroke::new(1.0, FG_PRIMARY);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(RADIUS_SM as u8);

    // Active (pressed/selected)
    visuals.widgets.active.bg_fill    = BG_ACTIVE;
    visuals.widgets.active.bg_stroke  = Stroke::NONE;
    visuals.widgets.active.fg_stroke  = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(RADIUS_SM as u8);

    // Open (dropdown/combo open)
    visuals.widgets.open.bg_fill    = BG_ACTIVE;
    visuals.widgets.open.bg_stroke  = Stroke::NONE;
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(RADIUS_SM as u8);

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

    // Compact industrial spacing (8-point based)
    style.spacing.item_spacing   = Vec2::new(SP_SM, SP_XS);
    style.spacing.button_padding = Vec2::new(SP_SM, SP_XS);
    style.spacing.window_margin  = egui::Margin::same(SP_SM as i8);
    style.spacing.menu_margin    = egui::Margin::same(SP_XS as i8);
    style.spacing.indent         = SP_LG;
    style.spacing.interact_size  = Vec2::new(SP_XXL, 22.0);
    style.spacing.slider_width   = 160.0;

    // ── Font System ───────────────────────────────────────────────────────────
    setup_fonts(ctx);

    // Typography scale
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(FONT_SIZE_HEADING, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(FONT_SIZE_BODY, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(FONT_SIZE_BODY, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        FontId::new(FONT_SIZE_CAPTION, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        FontId::new(FONT_SIZE_CAPTION, egui::FontFamily::Monospace),
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

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER WIDGETS
// ═══════════════════════════════════════════════════════════════════════════════

/// A compact square button with no border when inactive.
pub fn icon_btn(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(icon).size(FONT_SIZE_SECTION))
        .min_size(Vec2::splat(ICON_BTN_SIZE))
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
        egui::RichText::new(icon).size(FONT_SIZE_SECTION).color(Color32::WHITE)
    } else {
        egui::RichText::new(icon).size(FONT_SIZE_SECTION).color(FG_SECONDARY)
    };
    let btn = egui::Button::new(text)
        .min_size(Vec2::splat(ICON_BTN_SIZE))
        .fill(if active { BG_ACTIVE } else { Color32::TRANSPARENT })
        .frame(true);
    ui.add(btn).on_hover_text(tooltip)
}
