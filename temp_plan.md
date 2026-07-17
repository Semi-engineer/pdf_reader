# Design System & Workspace Redesign — Implementation Plan

## Summary

Redesign the DocLens presentation layer from a traditional PDF reader into a modern engineering workspace. This affects **only UI/theme/layout code** — rendering, caching, PDF backend, and all core logic remain untouched.

> [!IMPORTANT]
> This is a large, multi-phase PR touching ~20 files. The plan is organized into **6 sequential phases** to keep each step reviewable and compilable.

---

## Current State Assessment

After reviewing every source file, here is what already exists and what needs to change:

| Area | Current State | Issues |
|------|--------------|--------|
| **Theme** | [theme.rs](file:///e:/dev/pdf_reader/src/ui/theme.rs) — Good semantic token system with Industrial Minimal palette | Font sizes don't match spec (13→16 heading), spacing is ad-hoc, corner radii are inconsistent (2px everywhere), no spacing constants exported |
| **Icons** | [icons.rs](file:///e:/dev/pdf_reader/src/ui/icons.rs) — Unicode-only icon definitions | Mixed emoji + Unicode across files; `toolbar.rs` and `activity_bar.rs` use inline string icons instead of constants |
| **Components** | [components/](file:///e:/dev/pdf_reader/src/ui/components) — Panel, SearchBox, PropertyGrid, Section, Button, TreeView (stub) | Most are thin wrappers; `button.rs` and `toolbar.rs` component are unused; no `icon_button`, `splitter`, or `tree_view` implementations |
| **Layout** | [app.rs](file:///e:/dev/pdf_reader/src/app.rs#L620-L801) — Modern workspace layout already in place | Already has Activity Bar → Left Sidebar → Central → Right Sidebar → Status Bar. Structure is sound. |
| **Toolbar** | [toolbar.rs](file:///e:/dev/pdf_reader/src/ui/toolbar.rs) — Groups: File, Nav, View, Search, Inspector toggle | Uses local `icon_btn` instead of shared component; hardcoded sizes; search duplicated from left sidebar |
| **Activity Bar** | [activity_bar.rs](file:///e:/dev/pdf_reader/src/ui/activity_bar.rs) — 6 panel toggles + settings | Uses inline Unicode icons instead of `icons.rs` constants; hand-drawn logo |
| **Sidebars** | [left_sidebar.rs](file:///e:/dev/pdf_reader/src/ui/panels/left_sidebar.rs), [right_sidebar.rs](file:///e:/dev/pdf_reader/src/ui/panels/right_sidebar.rs) | Tab selection uses emoji from `PanelId::icon()`; Outline/Bookmarks/Metadata are stubs |
| **Viewer** | [viewer.rs](file:///e:/dev/pdf_reader/src/ui/viewer.rs) — Full annotation + text selection | Hardcoded colors (L346-347, L591, L621-623); no soft shadow on pages; basic empty state |
| **Status Bar** | [statusbar.rs](file:///e:/dev/pdf_reader/src/ui/statusbar.rs) — Shows page, zoom, tool, search, cache, RAM | Uses emoji icons inline; good overall structure |
| **Tool Palette** | [tool_palette.rs](file:///e:/dev/pdf_reader/src/ui/tool_palette.rs) — Annotation tools with color picker | References `tool_palette_visible` in app but isn't shown in the current layout |
| **Legacy Sidebar** | [sidebar.rs](file:///e:/dev/pdf_reader/src/ui/sidebar.rs) — Thumbnail-only sidebar | Duplicate of thumbnails in `left_sidebar.rs`; can be removed |

---

## User Review Required

> [!IMPORTANT]
> **Icon library choice**: The spec recommends Lucide or Phosphor SVG icons. However, egui does not natively render SVG icons without extra crates (e.g., `egui_extras` image support or custom SVG rendering). Options:
> 1. **Continue with Unicode/Symbol icons** (current approach) — consistent and zero-dependency, but limited visual expressiveness.
> 2. **Pre-rasterize SVGs at build time** — convert Lucide/Phosphor SVGs to PNG atlases, load as textures. More work, better icons.
> 3. **Use `egui_extras` image loading** — load individual SVG/PNG icons at runtime.
>
> **Recommendation**: Option 1 for this PR (unify all Unicode icons through `icons.rs` constants). A follow-up PR can migrate to SVG icons. This avoids adding dependencies and keeps the PR scoped to design system + layout.

> [!IMPORTANT]
> **Tool palette placement**: The tool palette exists but is not rendered in the current layout. Should it be:
> 1. **Integrated into the toolbar** as annotation tool buttons (compact, always visible)
> 2. **Shown as a right-side floating panel** when an annotation tool is selected (auto-hide, less clutter)
> 3. **Placed in the left sidebar** as an "Annotations" panel tab (already partially exists)
>
> **Recommendation**: Option 1 — add annotation tools as a toolbar group, with color picker in a dropdown. Remove the standalone tool palette widget.

---

## Open Questions

> [!NOTE]
> **Light theme**: The spec doesn't mention a light theme. The current codebase has a `dark_mode` toggle that is a no-op. Should this redesign include a light theme, or should we remove the toggle and commit to dark-only? **Recommendation**: Dark-only for now; remove the non-functional toggle.

> [!NOTE]
> **Page shadow style**: The spec calls for "soft shadows" on document pages. egui has `Shadow` support but it was set to `NONE` in the current theme. Should we add a subtle drop shadow (e.g., 8px spread, 20% black) around rendered pages? **Recommendation**: Yes, add subtle page shadow.

---

## Proposed Changes

### Phase 1 — Design System Foundation

Establishes the token system that all other phases depend on.

---

#### [MODIFY] [theme.rs](file:///e:/dev/pdf_reader/src/ui/theme.rs)

Upgrade the existing theme module into a full Design System:

- **Typography scale** — Update font sizes to match spec: Heading 16px, Section 14px, Body 13px, Caption 12px. Add `FONT_SIZE_SECTION` and `FONT_SIZE_CAPTION` constants.
- **Spacing system** — Add exported constants: `SP_XS = 4.0`, `SP_SM = 8.0`, `SP_MD = 12.0`, `SP_LG = 16.0`, `SP_XL = 24.0`, `SP_XXL = 32.0`. Replace all ad-hoc spacing in `apply()`.
- **Corner radius** — Add `RADIUS_SM = 4.0`, `RADIUS_MD = 6.0`, `RADIUS_LG = 8.0`. Update widget corner radii from `2.0` → `RADIUS_SM`.
- **Page shadow** — Add `PAGE_SHADOW` constant and apply a subtle drop shadow to pages.
- **Additional semantic tokens** — Add `BG_WORKSPACE` (distinct from `BG_BASE`, slightly warmer for document canvas area), `FG_DISABLED`.
- **Remove icon drawing functions** — Move `draw_document_icon`, `draw_search_icon`, `draw_folder_icon` to `icons.rs`.

---

#### [MODIFY] [icons.rs](file:///e:/dev/pdf_reader/src/ui/icons.rs)

Unify all icon references:

- Add the icon drawing functions from `theme.rs`.
- Add activity bar icons as named constants: `ICON_THUMBNAILS`, `ICON_SEARCH`, `ICON_OUTLINE`, `ICON_BOOKMARKS`, `ICON_ANNOTATIONS`, `ICON_ATTACHMENTS`, `ICON_SETTINGS`.
- Add toolbar icons as named constants matching the spec groups.
- Ensure every UI file imports icons from this module instead of using inline strings.

---

### Phase 2 — Component Library

Build/improve the shared component library that all panels and screens use.

---

#### [MODIFY] [components/button.rs](file:///e:/dev/pdf_reader/src/ui/components/button.rs)

Replace current stub with a full `IconButton` component:
- Consistent sizing, hover/active states, tooltip support.
- Support for `active` toggle state (for toolbar buttons).
- Uses Design System tokens for all colors, radii, spacing.

#### [MODIFY] [components/toolbar.rs](file:///e:/dev/pdf_reader/src/ui/components/toolbar.rs)

Add `ToolbarGroup` and `ToolbarDivider` components:
- `ToolbarGroup` — horizontal container with consistent spacing.
- `ToolbarDivider` — thin vertical separator.

#### [MODIFY] [components/panel.rs](file:///e:/dev/pdf_reader/src/ui/components/panel.rs)

Improve the `Panel` component:
- Add collapsible header with icon support.
- Consistent padding using spacing system.

#### [MODIFY] [components/search_box.rs](file:///e:/dev/pdf_reader/src/ui/components/search_box.rs)

Improve search box:
- Consistent styling with Design System tokens.
- Built-in clear button.
- Result count display.

#### [MODIFY] [components/section.rs](file:///e:/dev/pdf_reader/src/ui/components/section.rs)

Improve collapsible sections for the inspector:
- Consistent header styling.
- Optional icon.

#### [MODIFY] [components/tree_view.rs](file:///e:/dev/pdf_reader/src/ui/components/tree_view.rs)

Implement `TreeView` component (currently a stub):
- Expand/collapse nodes.
- Used by Outline and Bookmark panels.

#### [MODIFY] [components/property_grid.rs](file:///e:/dev/pdf_reader/src/ui/components/property_grid.rs)

Improve property grid:
- Consistent key-value layout.
- Better alignment using spacing system.

#### [MODIFY] [components/mod.rs](file:///e:/dev/pdf_reader/src/ui/components/mod.rs)

Export new and updated components.

---

### Phase 3 — Activity Bar & Sidebar Redesign

---

#### [MODIFY] [activity_bar.rs](file:///e:/dev/pdf_reader/src/ui/activity_bar.rs)

- Replace inline Unicode icons with `icons.rs` constants.
- Use Design System spacing and radius tokens.
- Improve active indicator styling (use accent bar on left edge).
- Improve logo rendering.

#### [MODIFY] [panels/left_sidebar.rs](file:///e:/dev/pdf_reader/src/ui/panels/left_sidebar.rs)

- Replace emoji tab icons with `icons.rs` constants.
- Improve thumbnail panel layout with better spacing.
- Improve search panel to use the `SearchBox` component properly.
- Add placeholder content for Outline and Bookmarks panels (with `TreeView`).
- Use Design System tokens throughout.

#### [MODIFY] [panels/right_sidebar.rs](file:///e:/dev/pdf_reader/src/ui/panels/right_sidebar.rs)

- Replace emoji tab icons with `icons.rs` constants.
- Improve property grid layout.
- Use Design System tokens throughout.

#### [DELETE] [sidebar.rs](file:///e:/dev/pdf_reader/src/ui/sidebar.rs)

- Remove the legacy thumbnail-only sidebar. Its functionality is fully duplicated in `left_sidebar.rs`.

---

### Phase 4 — Toolbar Redesign

---

#### [MODIFY] [toolbar.rs](file:///e:/dev/pdf_reader/src/ui/toolbar.rs)

Major toolbar overhaul:
- Replace local `icon_btn` function with shared `IconButton` component.
- Use `ToolbarGroup` and `ToolbarDivider` components.
- Group actions by workflow as specified: File | Navigation | View | Annotation | Search.
- Add annotation tool buttons (integrated from tool palette): Highlight, Note, Drawing.
- Use Design System tokens for all sizing.
- Move keyboard shortcut handling to `CommandDispatcher` (already partially there).

#### [DELETE] [tool_palette.rs](file:///e:/dev/pdf_reader/src/ui/tool_palette.rs)

- Remove standalone tool palette. Annotation tools will be integrated into the toolbar.
- Color picker will be accessible from a toolbar dropdown.

---

### Phase 5 — Viewer & Status Bar Polish

---

#### [MODIFY] [viewer.rs](file:///e:/dev/pdf_reader/src/ui/viewer.rs)

- Replace hardcoded colors with Design System tokens (`SELECTION_BG`, `SEARCH_BG`, etc.).
- Add soft page shadow using `PAGE_SHADOW` constant.
- Improve empty state (no document) with better visual design: centered logo, styled button, keyboard shortcut hint.
- Better page spacing (use `SP_LG` between pages).
- Use `BG_WORKSPACE` for canvas background.

#### [MODIFY] [statusbar.rs](file:///e:/dev/pdf_reader/src/ui/statusbar.rs)

- Replace inline emoji icons with `icons.rs` constants.
- Use Design System tokens for all sizes.
- Add render status indicator (active/idle).
- Show background task count when applicable.

#### [MODIFY] [titlebar.rs](file:///e:/dev/pdf_reader/src/ui/titlebar.rs)

- Use Design System tokens for all sizing and spacing.
- Use `icons.rs` constants for window control icons.

#### [MODIFY] [panels/menubar.rs](file:///e:/dev/pdf_reader/src/ui/panels/menubar.rs)

- Use Design System tokens for font sizes.
- Consistent menu item styling.

---

### Phase 6 — Integration & Cleanup

---

#### [MODIFY] [app.rs](file:///e:/dev/pdf_reader/src/app.rs)

- Remove `tool_palette` and `sidebar` fields (legacy components being removed).
- Remove `tool_palette_visible` and `sidebar_visible` fields.
- Remove `std::mem::take` pattern for removed components.
- Update `save_settings` to remove `tool_palette_visible` and `sidebar_visible`.
- Ensure annotation tool selection now works through toolbar integration.

#### [MODIFY] [mod.rs](file:///e:/dev/pdf_reader/src/ui/mod.rs)

- Remove `pub mod sidebar` and `pub mod tool_palette`.
- Remove re-exports for `Sidebar` and `ToolPalette`.

#### [MODIFY] [workspace.rs](file:///e:/dev/pdf_reader/src/workspace.rs)

- Replace emoji icons in `PanelId::icon()` with `icons.rs` constants.

#### [MODIFY] [config.rs](file:///e:/dev/pdf_reader/src/config.rs)

- Remove `sidebar_visible` and `tool_palette_visible` settings (replaced by `WorkspaceState` persistence).

---

## File Impact Summary

| Action | File | Phase |
|--------|------|-------|
| MODIFY | `src/ui/theme.rs` | 1 |
| MODIFY | `src/ui/icons.rs` | 1 |
| MODIFY | `src/ui/components/button.rs` | 2 |
| MODIFY | `src/ui/components/toolbar.rs` | 2 |
| MODIFY | `src/ui/components/panel.rs` | 2 |
| MODIFY | `src/ui/components/search_box.rs` | 2 |
| MODIFY | `src/ui/components/section.rs` | 2 |
| MODIFY | `src/ui/components/tree_view.rs` | 2 |
| MODIFY | `src/ui/components/property_grid.rs` | 2 |
| MODIFY | `src/ui/components/mod.rs` | 2 |
| MODIFY | `src/ui/activity_bar.rs` | 3 |
| MODIFY | `src/ui/panels/left_sidebar.rs` | 3 |
| MODIFY | `src/ui/panels/right_sidebar.rs` | 3 |
| DELETE | `src/ui/sidebar.rs` | 3 |
| MODIFY | `src/ui/toolbar.rs` | 4 |
| DELETE | `src/ui/tool_palette.rs` | 4 |
| MODIFY | `src/ui/viewer.rs` | 5 |
| MODIFY | `src/ui/statusbar.rs` | 5 |
| MODIFY | `src/ui/titlebar.rs` | 5 |
| MODIFY | `src/ui/panels/menubar.rs` | 5 |
| MODIFY | `src/app.rs` | 6 |
| MODIFY | `src/ui/mod.rs` | 6 |
| MODIFY | `src/workspace.rs` | 6 |
| MODIFY | `src/config.rs` | 6 |

**Total: 22 files modified, 2 files deleted**

---

## Verification Plan

### Automated Tests

```bash
cargo check
cargo build
cargo clippy -- -W warnings
```

The project has no unit tests currently, so compilation + clippy is the primary automated verification.

### Manual Verification

1. **Build and launch** — `cargo run` with a sample PDF
2. **Visual checklist**:
   - [ ] Theme tokens applied consistently (no hardcoded colors)
   - [ ] Typography scale matches spec (16/14/13/12)
   - [ ] 8-point spacing system visible throughout
   - [ ] Corner radii consistent (4/6/8)
   - [ ] Activity bar icons consistent, active indicator works
   - [ ] Toolbar groups are visually separated and scannable
   - [ ] Annotation tools accessible from toolbar
   - [ ] Page shadow renders correctly
   - [ ] Empty workspace shows polished landing state
   - [ ] Status bar shows all info fields
   - [ ] Left sidebar panels switch correctly
   - [ ] Right sidebar inspector panels work
   - [ ] All keyboard shortcuts still functional
3. **Regression check** — Verify no functional changes:
   - [ ] PDF opens and renders correctly
   - [ ] Page navigation works
   - [ ] Zoom and rotation work
   - [ ] Search works
   - [ ] Text selection and copy work
   - [ ] Annotations (highlight, pen, text) work
   - [ ] Settings save/restore works
