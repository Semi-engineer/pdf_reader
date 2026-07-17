# Performance Overhaul — Testing Plan

## Test Environment Setup

### Required Test Documents
1. **Small**: 10-50 pages (baseline comparison)
2. **Medium**: 100-500 pages (standard use case)
3. **Large**: 1000-1500 pages (target scenario)
4. **Very Large**: 3000-5000 pages (stress test)

### Metrics to Measure
- Document open time
- Time to first render
- Memory usage (Task Manager / Process Explorer)
- Scrolling FPS (subjective smoothness)
- Navigation responsiveness
- Thumbnail sidebar performance

## Test Cases

### 1. Document Opening Performance

#### Test: Open Large Document (1200 pages)
**Before (Expected)**:
- Long delay during open
- UI freeze
- High memory spike

**After (Target)**:
- Instant open (< 1 second)
- No UI freeze
- Gradual memory allocation

**Steps**:
1. Launch DocLens
2. File → Open → Select 1200-page PDF
3. Measure time until document is interactive
4. Observe memory usage in Task Manager

**Success Criteria**:
- Document opens in < 1 second
- UI remains responsive during open
- Initial memory < 100 MB

---

### 2. Page Navigation Performance

#### Test: Rapid Page Navigation
**Scenario**: User rapidly navigates through document

**Steps**:
1. Open large document (1000+ pages)
2. Press Page Down rapidly 10 times
3. Press Page Up rapidly 10 times
4. Use thumbnail sidebar to jump to page 500
5. Jump to page 1000
6. Jump back to page 1

**Success Criteria**:
- Each navigation responds within 100ms
- No accumulated lag
- Obsolete renders are cancelled
- Memory stays bounded

---

### 3. Scrolling Performance

#### Test: Smooth Scrolling
**Scenario**: User scrolls through document continuously

**Steps**:
1. Open large document
2. Hold Page Down for 30 seconds
3. Observe scrolling smoothness
4. Check render queue doesn't grow unbounded

**Success Criteria**:
- Smooth 60 FPS scrolling (subjective)
- No visible stuttering
- Render queue < 10 items
- Pages appear quickly

---

### 4. Memory Management

#### Test: Bounded Memory Usage
**Scenario**: Memory doesn't grow unbounded with large documents

**Steps**:
1. Open 3000-page document
2. Navigate to page 1500
3. Check memory usage
4. Navigate to page 3000
5. Check memory usage again
6. Navigate back to page 1
7. Check memory usage

**Success Criteria**:
- Memory stays under 1 GB throughout
- Memory doesn't continuously grow
- Cache eviction works properly

---

### 5. Thumbnail Performance

#### Test: Thumbnail Sidebar Virtualization
**Scenario**: Thumbnail sidebar remains responsive with large documents

**Steps**:
1. Open 1200-page document
2. Open thumbnail sidebar (left panel)
3. Scroll through thumbnails rapidly
4. Observe loading behavior
5. Check memory usage

**Success Criteria**:
- Only visible + nearby thumbnails loaded
- Scrolling remains smooth
- Thumbnail memory < 100 MB
- No UI freezes

---

### 6. Concurrent Operations

#### Test: Multi-tasking Responsiveness
**Scenario**: UI remains responsive during background rendering

**Steps**:
1. Open large document
2. Immediately try to:
   - Open search panel
   - Navigate pages
   - Zoom in/out
   - Open annotation tools

**Success Criteria**:
- All UI interactions respond immediately
- No blocking operations
- Background rendering doesn't block UI

---

### 7. Render Cancellation

#### Test: Obsolete Render Cancellation
**Scenario**: System cancels renders for pages no longer needed

**Steps**:
1. Open large document
2. Navigate to page 100
3. Immediately navigate to page 500 (before page 100 fully renders)
4. Observe render behavior

**Success Criteria**:
- Page 100 render is cancelled
- Page 500 renders immediately
- No wasted CPU on obsolete pages

---

### 8. Cache Efficiency

#### Test: Cache Hit Ratio
**Scenario**: Recently viewed pages load instantly from cache

**Steps**:
1. Open document
2. View pages 1, 2, 3
3. Navigate to page 100
4. Navigate back to page 2
5. Measure page 2 load time

**Success Criteria**:
- Page 2 appears instantly (< 10ms)
- No re-render required
- Cache hit ratio > 80% for repeated views

---

### 9. Zoom Performance

#### Test: Zoom Operations
**Scenario**: Zooming doesn't cause excessive delays

**Steps**:
1. Open large document
2. Zoom to 200%
3. Navigate several pages
4. Zoom to 50%
5. Navigate several pages
6. Reset to 100%

**Success Criteria**:
- Cache clears properly on zoom change
- New zoom level renders quickly
- No memory leaks
- Smooth transitions

---

### 10. Stress Test

#### Test: Maximum Document Size
**Scenario**: Test with extremely large document

**Steps**:
1. Open 5000-page document
2. Perform all basic operations:
   - Navigate to various pages
   - Zoom in/out
   - Scroll thumbnails
   - Search
3. Monitor memory and CPU
4. Run for 10 minutes

**Success Criteria**:
- Application remains responsive
- Memory stays under 1.5 GB
- No crashes
- All features work normally

---

## Performance Benchmarks

### Target Metrics (1200-page document)

| Metric | Before | Target | Measured |
|--------|--------|--------|----------|
| Open Time | ~10s | < 1s | _TBD_ |
| Memory Usage | ~2 GB | < 500 MB | _TBD_ |
| Page Navigation | ~200ms | < 50ms | _TBD_ |
| Scroll FPS | ~20 | 60 | _TBD_ |
| Thumbnail Load | All pages | Visible only | _TBD_ |

### Configuration for Testing

Default config should be suitable for testing:
```rust
page_cache_mb: 256
thumbnail_cache_mb: 64
texture_cache_mb: 512
prefetch_ahead: 2
prefetch_behind: 1
max_concurrent_renders: 4
```

## Debugging Tools

### Enable Verbose Logging
```bash
set RUST_LOG=debug
cargo run --release
```

### Monitor Metrics (Future)
Once metrics UI is added:
- Open Developer Panel
- Watch real-time FPS
- Monitor queue length
- Check cache hit ratio
- View memory usage

### Windows Performance Monitor
- Open Task Manager
- Go to Performance tab
- Monitor:
  - CPU usage
  - Memory usage
  - GPU usage (if available)

## Known Limitations

### Current Implementation Status
✅ Lazy document loading
✅ Viewport-based rendering
✅ Render scheduler with priorities
✅ Render cancellation
✅ Thumbnail virtualization
✅ Multi-level caching
✅ Memory tracking
✅ Performance metrics collection

❌ Progressive rendering (multi-resolution)
❌ GPU acceleration
❌ Metrics UI panel
❌ Adaptive cache sizing

## Regression Testing

### Areas to Check for Regressions
- [ ] Annotation placement still accurate
- [ ] Search still works correctly
- [ ] Text selection still functional
- [ ] Thumbnail clicks navigate properly
- [ ] Zoom levels display correctly
- [ ] Rotation works as expected
- [ ] Two-page mode still works
- [ ] Settings persist correctly

## Success Criteria Summary

The performance overhaul is considered successful if:

1. **Opening 1200-page documents is instant** (< 1 second)
2. **UI never freezes** during any operation
3. **Memory usage stays bounded** (< 1 GB for large docs)
4. **Scrolling is smooth** (subjective 60 FPS feel)
5. **Navigation is instantaneous** (< 50ms response)
6. **Thumbnail sidebar is responsive** with virtualization
7. **No regressions** in existing features

## Next Steps

1. ✅ Implementation complete
2. ⏭️ Build release version
3. ⏭️ Test with real large PDFs
4. ⏭️ Measure performance metrics
5. ⏭️ Document results
6. ⏭️ Iterate on any issues
7. ⏭️ Add metrics UI panel (future enhancement)
