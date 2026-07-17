# เปลี่ยนเป็น Standard Windows Title Bar

## การเปลี่ยนแปลง

### ✅ ปิด Custom Frameless Title Bar
### ✅ เปิดใช้ Standard Windows Decorations
### ✅ Icon จะแสดงใน Taskbar และ Alt+Tab

---

## ไฟล์ที่แก้ไข

### 1. `src/main.rs`

**เปลี่ยน:**
```rust
.with_decorations(false)  // ← custom frameless
```

**เป็น:**
```rust
.with_decorations(true)   // ← standard Windows title bar
```

### 2. `src/app.rs`

**ปิดการแสดง custom title bar:**
```rust
// Comment out custom title bar rendering
/*
if crate::ui::show_title_bar(ctx, doc_name) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    return;
}
*/
```

---

## ผลลัพธ์

### ข้อดี ✅

1. **Icon แสดงใน Taskbar** - Windows taskbar จะแสดง icon จากไฟล์ .ico
2. **Icon แสดงใน Alt+Tab** - Switcher แสดง icon ถูกต้อง
3. **Standard Window Controls** - Minimize, Maximize, Close ทำงานมาตรฐาน
4. **ไม่ต้อง Handle Drag** - Windows จัดการให้อัตโนมัติ
5. **Snap/Aero Peek** - ฟีเจอร์ Windows ทำงานปกติ

### ข้อเสีย ❌

1. **เสีย Custom Title Bar Design** - ไม่มี custom PDF icon และชื่อไฟล์ใน title bar แล้ว
2. **ไม่สามารถ Customize สี** - Title bar เป็นสีของ Windows theme
3. **พื้นที่ Toolbar สูงขึ้น** - มี title bar + toolbar รวมกัน

---

## การใช้งาน

### รัน Application:
```bash
cargo run --release
```

หรือ:
```bash
target\release\doclens.exe
```

### ตรวจสอบ Icon:
1. ดู Taskbar → ควรเห็น icon PDF
2. กด Alt+Tab → ควรเห็น icon และชื่อแอป "DocLens"
3. ดู Title Bar → แสดงชื่อไฟล์ที่เปิดอยู่

---

## Window Title

แอปจะแสดงชื่อใน title bar ตามนี้:
- **ไม่ได้เปิดไฟล์:** `DocLens`
- **เปิดไฟล์:** `filename.pdf - DocLens`

Windows จัดการโดยอัตโนมัติจาก:
```rust
eframe::run_native(
    "DocLens",  // ← ชื่อแอป
    options,
    ...
)
```

---

## การกลับไปใช้ Custom Title Bar

ถ้าต้องการกลับไปใช้ custom frameless title bar (แต่ icon ใน taskbar จะไม่แสดง):

### 1. แก้ `src/main.rs`:
```rust
.with_decorations(false)  // เปลี่ยนกลับเป็น false
```

### 2. แก้ `src/app.rs`:
```rust
// Uncomment custom title bar code
let doc_name = self.doc_path.as_deref().and_then(|p| {
    std::path::Path::new(p)
        .file_name()
        .and_then(|n| n.to_str())
});
if crate::ui::show_title_bar(ctx, doc_name) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    return;
}
```

### 3. Rebuild:
```bash
cargo build --release
```

---

## ข้อมูลเพิ่มเติม

### Custom Title Bar Features (ถูกปิดไว้)
- PDF icon สีน้ำเงินวาดด้วย SVG
- แสดงชื่อไฟล์ใน title bar
- Window control buttons (minimize, maximize, close) ที่ออกแบบเอง
- Drag-to-move window
- Double-click to maximize

### Icon Loading (ยังทำงานอยู่)
- โหลดจาก `icon/icon.ico`
- เลือกขนาด 32x32 หรือ 48x48 อัตโนมัติ
- มี fallback icon สีน้ำเงิน
- มี debug logging ใน console

---

## Performance

**ไฟล์ที่ไม่ถูกใช้งานอีกต่อไป (แต่ยังอยู่ในโค้ด):**
- `src/ui/titlebar.rs` - Custom title bar component
- `draw_pdf_icon()` function

**ประหยัด CPU:**
- ไม่ต้องวาด custom title bar ทุก frame
- Windows จัดการ title bar อย่างมีประสิทธิภาพ

---

## สรุป

✅ **แก้ไขเสร็จสมบูรณ์**

- Standard Windows title bar แสดงผล
- Icon แสดงใน taskbar และ Alt+Tab
- Application ทำงานปกติทุกฟังก์ชัน
- Build เวอร์ชัน: 8.3 MB (8,759,296 bytes)

**ลองรันและดู icon ใน taskbar ได้เลยครับ!** 🎉

---

**วันที่อัพเดท:** 2026-07-17  
**Build Time:** 11:11:46 AM  
**เวอร์ชัน:** DocLens v0.1.0
