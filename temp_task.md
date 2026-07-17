# DocLens Redesign — Task Tracker

## Phase 1 — Design System Foundation ✅ COMPLETE
- [x] Update `theme.rs` — typography, spacing, radius, shadow, tokens
- [x] Update `icons.rs` — unify all icon constants, move drawing functions

## Phase 2 — Component Library ✅ COMPLETE
- [x] `components/button.rs` — IconButton component (icon_button helper)
- [x] `components/toolbar.rs` — ToolbarGroup, ToolbarDivider
- [x] `components/panel.rs` — improved Panel with icon header
- [x] `components/search_box.rs` — styled with DS tokens
- [x] `components/section.rs` — collapsible section with icon
- [x] `components/tree_view.rs` — TreeView (CollapsingHeader-based, ready for Outline/Bookmarks)
- [x] `components/property_grid.rs` — improved layout
- [x] `components/mod.rs` — export updates

## Phase 3 — Activity Bar & Sidebar ✅ COMPLETE
- [x] `activity_bar.rs` — use icons.rs, DS tokens
- [x] `panels/left_sidebar.rs` — tabs, thumbnails, search, stubs
- [x] `panels/right_sidebar.rs` — tabs, property grid
- [x] Delete `sidebar.rs` (legacy) — DELETED
- [x] `workspace.rs` — PanelId::icon() uses icons.rs constants

## Phase 4 — Toolbar Redesign ✅ COMPLETE
- [x] `toolbar.rs` — shared components, annotation integration
- [x] Delete `tool_palette.rs` — DELETED

## Phase 5 — Viewer & Status Bar Polish ✅ COMPLETE
- [x] `viewer.rs` — DS tokens, page shadow, polished empty state, BG_WORKSPACE
- [x] `statusbar.rs` — icons.rs constants (no more inline emoji)
- [x] `titlebar.rs` — icons.rs constants, removed duplicate draw_pdf_icon
- [x] `panels/menubar.rs` — DS tokens (was already clean)

## Phase 6 — Integration & Cleanup ✅ COMPLETE
- [x] `app.rs` — removed legacy fields (sidebar, tool_palette, sidebar_visible, tool_palette_visible)
- [x] `ui/mod.rs` — removed legacy modules (sidebar, tool_palette)
- [x] `workspace.rs` — PanelId::icon() uses icons.rs
- [x] `config.rs` — removed sidebar_visible, tool_palette_visible, tool_palette_x/y settings

## Verification ✅ PASSED
- [x] `cargo check` passes — 0 errors, 0 warnings
- [x] `cargo build` passes — 0 errors, 0 warnings
- [ ] `cargo clippy` — optional, not yet run

---

## Summary of Changes

| Action | File | Phase | Status |
|--------|------|-------|--------|
| MODIFY | `src/ui/theme.rs` | 1 | ✅ |
| MODIFY | `src/ui/icons.rs` | 1 | ✅ |
| MODIFY | `src/ui/components/button.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/toolbar.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/panel.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/search_box.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/section.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/tree_view.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/property_grid.rs` | 2 | ✅ |
| MODIFY | `src/ui/components/mod.rs` | 2 | ✅ |
| MODIFY | `src/ui/activity_bar.rs` | 3 | ✅ |
| MODIFY | `src/ui/panels/left_sidebar.rs` | 3 | ✅ |
| MODIFY | `src/ui/panels/right_sidebar.rs` | 3 | ✅ |
| DELETE | `src/ui/sidebar.rs` | 3 | ✅ |
| MODIFY | `src/ui/toolbar.rs` | 4 | ✅ |
| DELETE | `src/ui/tool_palette.rs` | 4 | ✅ |
| MODIFY | `src/ui/viewer.rs` | 5 | ✅ |
| MODIFY | `src/ui/statusbar.rs` | 5 | ✅ |
| MODIFY | `src/ui/titlebar.rs` | 5 | ✅ |
| MODIFY | `src/ui/panels/menubar.rs` | 5 | ✅ (already clean) |
| MODIFY | `src/app.rs` | 6 | ✅ |
| MODIFY | `src/ui/mod.rs` | 6 | ✅ |
| MODIFY | `src/workspace.rs` | 6 | ✅ |
| MODIFY | `src/config.rs` | 6 | ✅ |

**Total: 22 files modified, 2 files deleted — cargo check ✅  cargo build ✅**
