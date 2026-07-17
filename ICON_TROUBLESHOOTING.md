# Icon Troubleshooting - DocLens

## ปัญหา: Taskbar Icon ไม่แสดง

### สถานการณ์
- ✅ Title bar แสดงไอคอน PDF สีฟ้า (SVG-style) ถูกต้อง
- ❌ Taskbar ยังแสดง icon default ของ Windows

### สาเหตุ

**Frameless Window + egui Limitation:**
- egui/eframe ใช้ `with_icon()` เพื่อ set icon
- แต่ frameless window บน Windows อาจไม่แสดง icon ใน taskbar โดยอัตโนมัติ
- ต้องใช้ Windows API เพิ่มเติม

### วิธีทดสอบ

1. **รันและดู Log Output:**
   ```bash
   run_with_log.bat
   ```

2. **ตรวจสอบ Output:**
   ```
   Trying icon path: icon/icon.ico (exists: true)
   Read XXXXX bytes from icon/icon.ico
   ICO file contains X entries
   ✓ Successfully loaded icon: 32x32
   ```

   ถ้าเห็นข้อความข้างบน แสดงว่า icon ถูกโหลดสำเร็จ

3. **ทดสอบแบบมี decorations:**
   แก้ไขใน `src/main.rs`:
   ```rust
   .with_decorations(true)  // เปลี่ยนจาก false เป็น true
   ```
   
   Build ใหม่และทดสอบ - ถ้า icon แสดงใน taskbar แสดงว่าปัญหาเกิดจาก frameless window

### วิธีแก้ (สำหรับ Frameless Window)

#### Option 1: ใช้ Windows API (แนะนำ)

เพิ่ม dependency:
```toml
[dependencies]
windows = { version = "0.58", features = ["Win32_UI_WindowsAndMessaging", "Win32_Foundation"] }
```

เพิ่มโค้ดใน `main.rs`:
```rust
#[cfg(target_os = "windows")]
fn set_window_icon(window: &winit::window::Window, icon_data: &egui::viewport::IconData) {
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::Win32::Foundation::HWND;
    
    unsafe {
        let hwnd = HWND(window.hwnd() as isize);
        
        // Create HICON from RGBA data
        // ... (implementation needed)
        
        // Set icon for window
        SendMessageW(hwnd, WM_SETICON, WPARAM(1), LPARAM(hicon as isize));
        SendMessageW(hwnd, WM_SETICON, WPARAM(0), LPARAM(hicon as isize));
    }
}
```

#### Option 2: ใช้ Decorations แบบปกติ

ถ้าไม่จำเป็นต้อง frameless:
```rust
.with_decorations(true)  // ใช้ standard Windows title bar
```

Icon จะแสดงโดยอัตโนมัติ

#### Option 3: ใช้ External Tool

Build exe แล้วใช้ tool เช่น Resource Hacker เพื่อฝัง icon เข้า exe:
```bash
rcedit doclens.exe --set-icon icon/icon.ico
```

### Windows Icon Requirements

- **ขนาดที่รองรับ:** 16x16, 32x32, 48x48, 256x256
- **Format:** ICO file with multiple sizes
- **Color depth:** 32-bit RGBA
- **Taskbar:** ใช้ 32x32 หรือ 48x48
- **Alt+Tab:** ใช้ 32x32 หรือ 48x48  
- **Title bar:** ใช้ 16x16 หรือ 32x32

### Current Status

**โค้ดปัจจุบัน:**
- ✅ โหลดไฟล์ .ico ได้ถูกต้อง
- ✅ เลือกขนาด 32x32 หรือ 48x48 โดยอัตโนมัติ
- ✅ Fallback เป็น icon สีน้ำเงิน
- ✅ มี debug logging ครบถ้วน
- ❌ Frameless window ไม่แสดง icon ใน taskbar (Windows limitation)

### Workaround ชั่วคราว

จนกว่าจะแก้ไขด้วย Windows API:

1. **ใช้ decorations แบบปกติ** (เสียความสวยงาม แต่ icon แสดง)
2. **Build exe และใช้ rcedit** (manual step)
3. **ใช้ custom titlebar จาก egui** (ปัจจุบันทำแล้ว แต่ taskbar ยังไม่แสดง)

### Next Steps

เพื่อแก้ไขให้สมบูรณ์:

1. ✅ ทดสอบด้วย `with_decorations(true)` ก่อน
2. ❌ ถ้า icon แสดง → ยืนยันว่าเป็นปัญหาจาก frameless window
3. ❌ ต้องใช้ Windows API เพื่อ set icon manual
4. ❌ หรือใช้ rcedit ใน build script

---

**สรุป:** Icon โหลดถูกต้องแล้ว แต่ frameless window ใน Windows ต้องการ setup เพิ่มเติมเพื่อแสดง icon ใน taskbar
