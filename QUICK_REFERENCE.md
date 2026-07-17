# Performance Overhaul — Quick Reference Card

## 🎯 Core Concepts

| Concept | Description |
|---------|-------------|
| **Lazy Loading** | Don't initialize pages until needed |
| **Viewport Culling** | Only render visible + nearby pages |
| **Priority Queue** | Visible pages render first |
| **Render Cancellation** | Stop rendering invisible pages |
| **Virtualization** | Only load visible thumbnails |
| **LRU Caching** | Evict least recently used items |

---

## 📦 Component Map

```
src/
├── render_scheduler.rs      ← Priority queue & cancellation
├── performance_config.rs    ← Memory budgets & settings
├── performance_metrics.rs   ← FPS, memory, queue stats
├── viewport.rs              ← Visible page calculation
├── texture_pool.rs          ← GPU texture reuse
├── page_cache.rs           ← LRU page cache (ENHANCED)
├── render_worker.rs        ← Background rendering (ENHANCED)
├── thumbnail_manager.rs    ← Thumbnail virtualization (ENHANCED)
└── app.rs                  ← Integration point (ENHANCED)
```

---

## ⚙️ Default Configuration

```rust
page_cache_mb: 256          // ~128 pages at 100% zoom
thumbnail_cache_mb: 64      // ~640 thumbnails
texture_cache_mb: 512       // ~256 textures
prefetch_ahead: 2           // Render 2 pages ahead
prefetch_behind: 1          // Render 1 page behind
max_concurrent_renders: 4   // 4 parallel render jobs
```

---

## 🎮 Key Functions

### Request Render with Priority
```rust
worker.render_page(
    page,
    zoom,
    rotation,
    RenderPriority::Visible  // or Prefetch, Thumbnail, Background
);
```

### Cancel Renders
```rust
// Cancel specific request
worker.cancel_request(request_id);

// Cancel all below priority
worker.cancel_below_priority(RenderPriority::Prefetch);
```

### Check Viewport
```rust
viewport.set_current_page(page);
viewport.is_visible(page);          // true if in viewport
viewport.is_prefetch(page);         // true if in prefetch range
viewport.visible_range();           // Iterator over visible pages
viewport.prefetch_range();          // Iterator over prefetch pages
```

### Memory Tracking
```rust
app.page_cache.memory_usage_mb();
app.thumbnail_manager.memory_usage_mb();
```

### Metrics
```rust
let metrics = app.render_scheduler.metrics();
metrics.fps();
metrics.avg_queue_length();
metrics.cache_hit_ratio();
metrics.avg_render_duration_ms();
```

---

## 🔄 Typical Flow

### Document Open
```
1. PdfDocument::open(path)              → Load metadata only
2. viewport = Viewport::new(page_count)  → Initialize viewport
3. request_visible_page_renders()        → Queue visible pages
4. UI shows instantly ✅
```

### Page Navigation
```
1. viewport.set_current_page(new_page)           → Update viewport
2. worker.cancel_below_priority(Prefetch)        → Cancel obsolete
3. for page in viewport.visible_range()          → Queue visible
4. for page in viewport.prefetch_range()         → Queue prefetch
5. Page appears quickly ✅
```

### Render Response
```
1. worker.try_recv()                    → Non-blocking poll
2. page_cache.put(key, image)           → Cache result
3. texture_pool.get_or_create()         → Reuse texture
4. ctx.request_repaint()                → Update UI
```

---

## 📊 Priority Levels

```
┌─────────────────────────────────────┐
│ Priority 0: Visible                 │ ← User sees NOW
├─────────────────────────────────────┤
│ Priority 1: Prefetch                │ ← User will see soon
├─────────────────────────────────────┤
│ Priority 2: Thumbnail               │ ← Background generation
├─────────────────────────────────────┤
│ Priority 3: Background              │ ← Lowest priority
└─────────────────────────────────────┘
```

---

## 🎯 Viewport Math

```rust
// Single page mode
visible_range = [current, current + 1)

// Two-page mode
visible_range = [current, current + 2)

// Prefetch range
prefetch_range = [
    current - prefetch_behind,
    current + visible_pages + prefetch_ahead
)
```

**Example** (current = 100, single page, prefetch 2/1):
```
Visible:  [100, 101)   → Page 100
Prefetch: [99, 103)    → Pages 99, 100, 101, 102
```

---

## 💾 Memory Budget Calculator

```
Page at 100% zoom (A4, 150 DPI):
- Size: 1245 × 1755 × 4 bytes ≈ 8.7 MB

Thumbnail at 20% scale:
- Size: 249 × 351 × 4 bytes ≈ 350 KB

GPU Texture:
- Same as page: ~8.7 MB

Budget Examples:
- 256 MB page cache = ~30 pages at 100% zoom
- 64 MB thumbnail cache = ~183 thumbnails
- 512 MB texture cache = ~60 textures
```

---

## 🚀 Performance Targets

| Metric | Target | Test Document |
|--------|--------|---------------|
| Open Time | < 1 second | 1200 pages |
| Navigation | < 50ms | Any page jump |
| Scrolling | 60 FPS | Continuous scroll |
| Memory | < 500 MB | 1200 pages active |
| UI Freeze | 0 | Any operation |

---

## 🐛 Debug Commands

### Enable Logging
```bash
set RUST_LOG=debug
cargo run --release
```

### Build Commands
```bash
# Check compilation
cargo check

# Build release
cargo build --release

# Run release
cargo run --release
```

### Monitor Memory
```
Windows Task Manager → Performance → Memory
Look for: doclens.exe process
```

---

## 📝 Common Patterns

### Check Cache Before Render
```rust
let key = CacheKey::new(page, zoom, rotation);
if let Some(image) = page_cache.get(&key) {
    // Use cached image
} else {
    // Request render
    worker.render_page(page, zoom, rotation, priority);
}
```

### Update Viewport on Navigation
```rust
fn goto_page(&mut self, page: usize) {
    self.current_page = page;
    self.viewport.set_current_page(page);
    self.request_visible_page_renders();
}
```

### Clear on Document Close
```rust
page_cache.clear();
thumbnail_manager.clear();
texture_pool.clear();
render_scheduler.cancel_all_pending();
```

---

## ⚠️ Important Notes

### Thread Safety
- All caches use `Arc<Mutex<...>>` for thread safety
- Render worker spawns OS thread with Tokio runtime
- No blocking operations on UI thread

### Memory Management
- LRU eviction happens automatically
- Cache capacity is HARD LIMIT
- Memory tracking is best-effort estimate

### Cancellation
- Only cancels PENDING requests
- Active renders complete
- Cancelled renders don't update cache

---

## 🎓 Learning Path

1. Read `PERFORMANCE_README.md` — High-level overview
2. Read `PERFORMANCE_OVERHAUL.md` — Detailed architecture
3. Review `src/render_scheduler.rs` — Core scheduler logic
4. Review `src/viewport.rs` — Viewport calculation
5. Review `src/app.rs` — Integration points
6. Run tests from `TESTING_PLAN.md`

---

## 📚 File Purposes

| File | Purpose |
|------|---------|
| `render_scheduler.rs` | Priority queue, cancellation, metrics |
| `performance_config.rs` | Configuration struct, defaults |
| `performance_metrics.rs` | FPS, memory, timing tracking |
| `viewport.rs` | Visible/prefetch range calculation |
| `texture_pool.rs` | GPU texture reuse |
| `page_cache.rs` | LRU cache with memory tracking |
| `render_worker.rs` | Background PDF rendering |
| `thumbnail_manager.rs` | Virtualized thumbnail loading |
| `app.rs` | Integration & orchestration |

---

## ✅ Checklist for Changes

When modifying performance code:

- [ ] Update viewport when current_page changes
- [ ] Clear caches on document close
- [ ] Cancel renders on navigation
- [ ] Respect priority order
- [ ] Limit UI thread work (< 16ms per frame)
- [ ] Update metrics if adding new operations
- [ ] Test with large document (1000+ pages)
- [ ] Check memory doesn't grow unbounded
- [ ] Verify no UI freezes

---

**Quick Ref Version**: 1.0  
**Last Updated**: Implementation Complete  
**Status**: ✅ Ready for Use
