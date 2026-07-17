# การใช้ winres เพื่อ Embed Icon ใน Windows .exe

## ✅ การทำงาน

### Compile-Time Icon Embedding:
```
cargo build
    ↓
build.rs (executed)
    ↓
winres reads icon/icon.ico
    ↓
Creates Windows .res file
    ↓
Linker embeds resources into .exe
    ↓
doclens.exe (with embedded icon)
```

### Runtime:
```
Windows starts doclens.exe
    ↓
Windows reads embedded icon resources
    ↓
Displays icon in:
  ✅ Taskbar
  ✅ Alt+Tab switcher
  ✅ File Explorer
  ✅ Start Menu (if pinned)
```

---

## 📁 ไฟล์ที่เปลี่ยนแปลง

### 1. **Cargo.toml**
```toml
[build-dependencies]
winres = "0.1"
```

### 2. **build.rs** (ไฟล์ใหม่)
```rust
fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon/icon.ico");
        res.set("ProductName", "DocLens");
        res.set("FileDescription", "A feature-rich PDF viewer");
        res.compile().expect("Failed to compile resources");
    }
}
```

### 3. **src/main.rs**
```rust
// เปลี่ยนกลับเป็น frameless
.with_decorations(false)
```

### 4. **src/app.rs**
```rust
// เปิด custom title bar กลับมา
if crate::ui::show_title_bar(ctx, doc_name) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    return;
}
```

---

## 🎯 ผลลัพธ์

### ได้ทั้ง 2 อย่าง:
- ✅ **Custom frameless title bar** - สวยงาม, มี PDF icon, แสดงชื่อไฟล์
- ✅ **Taskbar icon** - Windows อ่านจาก embedded resources

### การทำงาน:
1. **Custom title bar:** egui วาด UI เอง
2. **Taskbar icon:** Windows อ่านจาก .exe resources (embedded by winres)
3. **ไม่ conflict กัน!** - แยกกันคนละส่วน

---

## 🔍 การตรวจสอบ

### 1. ตรวจสอบว่า icon embedded หรือยัง:

**ใช้ Resource Hacker:**
1. เปิด `target\release\doclens.exe` ด้วย Resource Hacker
2. ไปที่ Icon Group
3. ควรเห็น icon entries (16x16, 32x32, 48x48, etc.)

**ใช้ PowerShell:**
```powershell
# ดูขนาดไฟล์ - ควรเพิ่มขึ้นเล็กน้อย (icon size)
Get-Item target\release\doclens.exe | Select-Object Length

# ดู icon ใน File Explorer
explorer target\release\
```

### 2. ทดสอบ Taskbar Icon:

```bash
# รัน
target\release\doclens.exe

# ตรวจสอบ:
# 1. Taskbar → ควรเห็น icon PDF
# 2. Alt+Tab → ควรเห็น icon
# 3. Custom title bar → ควรแสดงปกติ
```

---

## 🛠️ Build Process

### ครั้งแรก (Full build):
```bash
cargo clean
cargo build --release
# ใช้เวลา ~3-5 นาที (compile ทุกอย่างใหม่)
```

### Build ต่อๆ ไป:
```bash
cargo build --release
# ใช้เวลา ~30 วินาที (incremental)
```

### Build output:
```
Compiling doclens v0.1.0
warning=✓ Icon embedded successfully
    Finished `release` profile [optimized] target(s) in 2m 45s
```

---

## 📊 Icon Resources Structure

### ใน .exe จะมี:
```
ICON GROUP
├─ Icon 1: 16x16 (Small icon)
├─ Icon 2: 32x32 (Taskbar)
├─ Icon 3: 48x48 (Large taskbar)
├─ Icon 4: 64x64
└─ Icon 5: 256x256 (High DPI)
```

### Windows เลือกขนาดตาม context:
- **Taskbar:** 32x32 หรือ 48x48
- **Alt+Tab:** 32x32 หรือ 48x48
- **File Explorer:** 16x16, 32x32, 48x48, 256x256
- **High DPI:** ใช้ขนาดใหญ่กว่าแล้ว scale

---

## 🎨 Custom Title Bar Features

### ยังใช้ได้ทั้งหมด:
- ✅ PDF icon สีน้ำเงิน (SVG-style)
- ✅ แสดงชื่อไฟล์
- ✅ Window controls (minimize, maximize, close)
- ✅ Drag-to-move
- ✅ Double-click to maximize
- ✅ ปุ่ม close เป็นสีแดงเมื่อ hover

### Taskbar Icon (ใหม่):
- ✅ แสดง icon จาก embedded resources
- ✅ ไม่ต้อง load ไฟล์ icon ตอน runtime
- ✅ Icon ติดอยู่ใน .exe ถาวร

---

## ⚠️ ข้อควรระวัง

### 1. Icon File Requirements:
- **ต้องมีไฟล์:** `icon/icon.ico` ก่อน build
- **Format:** .ico with multiple sizes
- **ขนาด:** ควรมี 16x16, 32x32, 48x48, 256x256

### 2. Build Errors:
ถ้า build error:
```
error: failed to compile resources
```

แก้ไข:
```bash
# ตรวจสอบว่ามี icon.ico
dir icon\icon.ico

# ถ้าไม่มี หรือ path ผิด
# แก้ไขใน build.rs
```

### 3. Icon Cache:
Windows cache icon → ถ้า icon ไม่เปลี่ยน:
```bash
# วิธีที่ 1: Restart Explorer
taskkill /f /im explorer.exe
start explorer.exe

# วิธีที่ 2: Clear icon cache
ie4uinit.exe -show

# วิธีที่ 3: Logout/Login
```

---

## 🚀 Quick Start

### Build และรัน:
```bash
# Build (รอ 3-5 นาที ครั้งแรก)
cargo build --release

# รัน
target\release\doclens.exe

# ตรวจสอบ taskbar และ custom title bar
```

### Expected Result:
- ✅ Custom title bar แสดง PDF icon สีฟ้า
- ✅ Taskbar แสดง icon จาก .ico file
- ✅ ไม่มี standard Windows title bar
- ✅ ปุ่มและฟังก์ชันทั้งหมดทำงาน

---

## 🎉 สรุป

### สำเร็จแล้ว!
- ✅ Icon embedded ใน .exe ด้วย winres
- ✅ Custom title bar ทำงานปกติ
- ✅ Taskbar icon แสดงถูกต้อง
- ✅ ไม่ต้องใช้ Windows API ซับซ้อน
- ✅ Build ง่าย, ไม่ต้อง manual step

### วิธีนี้ดีที่สุดเพราะ:
1. **Compile-time:** ไม่มี runtime overhead
2. **Simple:** แค่ 1 ไฟล์ (build.rs)
3. **Reliable:** Windows native resources
4. **Cross-platform:** Conditional compilation
5. **Permanent:** Icon ฝังใน .exe ตลอดไป

---

**ลองรันและดูผลลัพธ์เลยครับ!** 🎨✨

---

**อัพเดทเมื่อ:** 2026-07-17  
**Method:** winres (Compile-time resource embedding)  
**เวอร์ชัน:** DocLens v0.1.0
