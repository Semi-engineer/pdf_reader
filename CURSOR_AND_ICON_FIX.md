# แก้ไขปัญหา Mouse Cursor และ Icon

## ปัญหาที่แก้ไข

### 1. 🖱️ **Mouse Cursor เป็น Text Mode ตลอดเวลา**

**ปัญหา:**
- Cursor แสดงเป็น I-beam (text cursor) ตลอดเวลาแม้ไม่ได้อยู่บนพื้นที่ PDF
- ไม่สะดวกในการใช้งานเพราะดูสับสน

**สาเหตุ:**
- ฟังก์ชัน `handle_text_selection()` และ annotation tools ตั้งค่า cursor แบบไม่มีเงื่อนไข
- ไม่ได้เช็คว่า mouse กำลัง hover บน PDF หรือไม่

**วิธีแก้:**
เพิ่มเงื่อนไข `response.hovered()` ก่อนตั้งค่า cursor:

```rust
// เดิม - ตั้งค่าโดยไม่เช็ค hover
ui.ctx().set_cursor_icon(egui::CursorIcon::Text);

// ใหม่ - ตั้งค่าเฉพาะเมื่อ hover
if response.hovered() {
    ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
}
```

**พื้นที่ที่แก้ไข:**
- ✅ Text selection mode - `handle_text_selection()`
- ✅ Text annotation tool
- ✅ Pen tool - Crosshair cursor
- ✅ Highlight/Rectangle/Circle tools - Crosshair cursor
- ✅ Line/Arrow tools - Crosshair cursor

**ผลลัพธ์:**
- Cursor จะเป็น arrow ปกติเมื่ออยู่นอกพื้นที่ PDF
- Cursor เป็น I-beam เฉพาะเมื่อ hover บน PDF ในโหมด text selection
- Cursor เป็น crosshair เฉพาะเมื่อ hover บน PDF ในโหมด annotation

---

### 2. 🎯 **Icon Bitmap ไม่แสดงใน Taskbar/Title Bar**

**ปัญหา:**
- ไฟล์ `icon/icon.ico` มีอยู่แต่ไม่ถูกโหลด
- Taskbar และ Alt+Tab แสดง icon เปล่าหรือ default icon

**สาเหตุ:**
- โค้ดเดิมไม่รองรับการอ่านไฟล์ `.ico` โดยตรง
- ไม่มีการ fallback ที่ดี
- ไม่มี logging เพื่อ debug

**วิธีแก้:**

1. **เพิ่ม dependency `ico`** ใน `Cargo.toml`:
```toml
ico = "0.3"
```

2. **ปรับปรุงฟังก์ชัน `load_icon()`:**
   - รองรับการอ่านไฟล์ `.ico` โดยตรง
   - ลองหลายเส้นทาง (icon/icon.ico, ./icon/icon.ico, icon.ico)
   - มี logging เพื่อ debug
   - Fallback เป็น icon สีน้ำเงินของ DocLens ถ้าไม่พบไฟล์

**โค้ดใหม่:**
```rust
fn load_icon() -> egui::viewport::IconData {
    // Try multiple icon paths
    let icon_paths = [
        "icon/icon.ico",
        "./icon/icon.ico",
        "icon.ico",
    ];
    
    for icon_path in &icon_paths {
        if path.exists() {
            // Try ICO format first
            if let Ok(ico) = ico::IconDir::read(...) {
                // Load first icon entry
                return icon_data;
            }
            
            // Fallback: try as regular image
            if let Ok(img) = image::load_from_memory(...) {
                return icon_data;
            }
        }
    }
    
    // Generate fallback icon (blue square)
    // ... ดูโค้ดเต็มใน main.rs
}
```

**Fallback Icon:**
- ถ้าไม่พบไฟล์ icon จะสร้าง icon สี่เหลี่ยมสีน้ำเงิน
- ใช้โทนสีของ DocLens (rgb(88, 112, 214))
- ขนาด 32x32 px
- มี border สวยงาม

**Logging:**
- แสดงข้อความเมื่อโหลด icon สำเร็จ: `✓ Loaded icon from icon/icon.ico: 32x32`
- แสดง error เมื่อไม่พบไฟล์: `✗ Failed to read icon/icon.ico: ...`
- แสดงเตือนเมื่อใช้ fallback: `⚠ No icon found, using default empty icon`

---

## การตรวจสอบ

### ตรวจสอบ Icon

```bash
# รันโปรแกรมและดู console output
cargo run --release

# ควรเห็นข้อความ:
# ✓ Loaded icon from icon/icon.ico: 32x32
```

ถ้าเห็นข้อความข้างบน แสดงว่า icon ถูกโหลดสำเร็จ!

### ตรวจสอบ Cursor

1. เปิด PDF ใดๆ
2. เลื่อน mouse ไปบน PDF → cursor ควรเป็น I-beam (|)
3. เลื่อน mouse ออกนอก PDF → cursor ควรเป็น arrow (➤)
4. คลิกเครื่องมือ annotation (เช่น Pen) → cursor บน PDF ควรเป็น crosshair (+)
5. เลื่อน mouse ออกนอก PDF → cursor ควรกลับเป็น arrow

---

## ไฟล์ที่แก้ไข

### 1. `src/main.rs`
- ปรับปรุงฟังก์ชัน `load_icon()`
- รองรับ `.ico` format
- เพิ่ม fallback icon
- เพิ่ม logging

### 2. `src/ui/viewer.rs`
- แก้ไข `handle_text_selection()` - เพิ่มเงื่อนไข `hovered()`
- แก้ไข Text tool cursor
- แก้ไข Pen tool cursor
- แก้ไข Highlight/Rectangle/Circle cursor
- แก้ไข Line/Arrow cursor

### 3. `Cargo.toml`
- เพิ่ม dependency `ico = "0.3"`

---

## สรุป

### ✅ ปัญหาที่แก้แล้ว

1. **Cursor ไม่ติดเป็น text mode อีกต่อไป**
   - Cursor จะเปลี่ยนตามบริบทที่เหมาะสม
   - ใช้งานได้ธรรมชาติและสะดวกมากขึ้น

2. **Icon แสดงใน taskbar และ Alt+Tab**
   - รองรับไฟล์ `.ico` แบบเต็มรูปแบบ
   - มี fallback icon ที่สวยงาม
   - มี logging สำหรับ debug

### 🎯 ประโยชน์

- **UX ดีขึ้น:** cursor บอกได้ว่าอยู่ใน mode ไหน
- **Professional:** icon แสดงถูกต้องใน Windows taskbar
- **Debug-friendly:** มี logging ชัดเจน
- **Robust:** มี fallback ทุกกรณี

---

**อัพเดทเมื่อ:** 2026-07-17  
**เวอร์ชัน:** DocLens v0.1.0
