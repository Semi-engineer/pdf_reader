# การวิเคราะห์: Custom Title Bar + Taskbar Icon

## 🎯 เป้าหมาย
- ✅ Custom frameless title bar (สวยงาม)
- ✅ Icon แสดงใน Windows taskbar
- ✅ Icon แสดงใน Alt+Tab switcher

---

## 🔍 ปัญหาที่พบ

### การทำงานปัจจุบัน:
```
User → eframe (decorations=false) → winit → Windows API
                    ↓
            set window icon
                    ↓
        Taskbar: ❌ ไม่แสดง icon
        Title bar: ✅ แสดง custom UI
```

### สาเหตุ:
1. **egui/eframe** ส่ง icon ผ่าน `ViewportBuilder::with_icon()`
2. **winit** แปลง icon เป็น RGBA bitmap แล้วส่งให้ Windows
3. **Windows frameless window** ต้องการ **HICON handle** แยกต่างหาก
4. Icon ที่ส่งผ่าน winit ไม่ได้ถูก set เป็น **WM_SETICON** message

---

## 💡 วิธีแก้ไข (3 แนวทาง)

### แนวทาง 1: ใช้ Windows API โดยตรง ⭐ (แนะนำ)

**ข้อดี:**
- ✅ Custom title bar ยังใช้ได้
- ✅ Icon แสดงใน taskbar
- ✅ ควบคุมได้เต็มที่

**ข้อเสีย:**
- ❌ ต้องใช้ Windows-specific code
- ❌ ซับซ้อนกว่า

**Implementation:**

#### 1. เพิ่ม Dependencies:
```toml
[dependencies]
# Windows API
windows = { version = "0.58", features = [
    "Win32_UI_WindowsAndMessaging",
    "Win32_Foundation",
    "Win32_Graphics_Gdi",
    "Win32_UI_Shell"
]}

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser", "wingdi", "shellapi"] }
```

#### 2. สร้าง HICON จาก RGBA data:
```rust
#[cfg(target_os = "windows")]
unsafe fn create_hicon_from_rgba(rgba: &[u8], width: u32, height: u32) -> Option<HICON> {
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::Win32::Graphics::Gdi::*;
    
    // Create BITMAPV5HEADER
    let mut bi = BITMAPV5HEADER {
        bV5Size: std::mem::size_of::<BITMAPV5HEADER>() as u32,
        bV5Width: width as i32,
        bV5Height: -(height as i32), // Top-down
        bV5Planes: 1,
        bV5BitCount: 32,
        bV5Compression: BI_BITFIELDS,
        bV5RedMask: 0x00FF0000,
        bV5GreenMask: 0x0000FF00,
        bV5BlueMask: 0x000000FF,
        bV5AlphaMask: 0xFF000000,
        ..Default::default()
    };
    
    // Create DIB section
    let hdc = GetDC(None);
    let mut bits: *mut c_void = std::ptr::null_mut();
    
    let hbm = CreateDIBSection(
        hdc,
        &bi as *const _ as *const BITMAPINFO,
        DIB_RGB_COLORS,
        &mut bits,
        None,
        0,
    )?;
    
    // Copy RGBA data (convert RGBA → BGRA)
    let dst = std::slice::from_raw_parts_mut(
        bits as *mut u8,
        (width * height * 4) as usize
    );
    
    for i in 0..(width * height) as usize {
        let src_idx = i * 4;
        let dst_idx = i * 4;
        dst[dst_idx + 0] = rgba[src_idx + 2]; // B
        dst[dst_idx + 1] = rgba[src_idx + 1]; // G
        dst[dst_idx + 2] = rgba[src_idx + 0]; // R
        dst[dst_idx + 3] = rgba[src_idx + 3]; // A
    }
    
    // Create mask bitmap (for transparency)
    let hbm_mask = CreateBitmap(width as i32, height as i32, 1, 1, None);
    
    // Create icon
    let ii = ICONINFO {
        fIcon: TRUE,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: hbm_mask,
        hbmColor: hbm,
    };
    
    let hicon = CreateIconIndirect(&ii)?;
    
    // Cleanup
    DeleteObject(hbm);
    DeleteObject(hbm_mask);
    ReleaseDC(None, hdc);
    
    Some(hicon)
}
```

#### 3. Set icon ให้ window:
```rust
#[cfg(target_os = "windows")]
pub fn set_window_icon_win32(window_handle: *mut c_void, icon_data: &egui::viewport::IconData) {
    use windows::Win32::Foundation::*;
    use windows::Win32::UI::WindowsAndMessaging::*;
    
    unsafe {
        let hwnd = HWND(window_handle as isize);
        
        // Create HICON from RGBA data
        if let Some(hicon) = create_hicon_from_rgba(
            &icon_data.rgba,
            icon_data.width,
            icon_data.height
        ) {
            // Set both small and large icons
            SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_SMALL as usize), LPARAM(hicon.0));
            SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(hicon.0));
            
            eprintln!("✓ Set window icon via Win32 API");
        }
    }
}
```

#### 4. เรียกใช้หลัง window สร้าง:
```rust
impl eframe::App for DocLensApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Set icon on first frame (only once)
        if !self.icon_set {
            #[cfg(target_os = "windows")]
            {
                if let Some(info) = frame.info().window_info.as_ref() {
                    // Get native window handle from egui
                    ctx.input(|i| {
                        if let Some(viewport) = i.viewport() {
                            // Access raw window handle through winit
                            // Note: Need to access through eframe internals
                        }
                    });
                }
            }
            self.icon_set = true;
        }
        
        // ... rest of update code
    }
}
```

**ปัญหา:** eframe 0.31 ไม่ expose raw window handle โดยตรง

---

### แนวทาง 2: ใช้ raw-window-handle ⭐⭐ (ง่ายกว่า)

**แนวทางนี้ใช้ได้ดีกว่า!**

#### 1. เพิ่ม dependency:
```toml
[dependencies]
raw-window-handle = "0.6"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser", "wingdi", "shellapi"] }
```

#### 2. ใช้ eframe callback:
```rust
fn main() -> Result<(), eframe::Error> {
    let icon_data = load_icon();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_decorations(false)
            .with_icon(icon_data.clone()),
        ..Default::default()
    };
    
    eframe::run_native(
        "DocLens",
        options,
        Box::new(move |cc| {
            // Set icon after window creation
            #[cfg(target_os = "windows")]
            {
                if let Some(window) = cc.egui_ctx.viewport(|vp| vp.window.clone()) {
                    set_window_icon_winapi(&window, &icon_data);
                }
            }
            
            Ok(Box::new(DocLensApp::new(cc)))
        }),
    )
}
```

---

### แนวทาง 3: ใช้ rcedit (Build-time) ⭐⭐⭐ (ง่ายที่สุด)

**ข้อดี:**
- ✅ ไม่ต้องแก้โค้ด
- ✅ Icon embedded ใน .exe file
- ✅ ทำงานกับทุกแอป Windows

**ข้อเสีย:**
- ❌ ต้อง run หลัง build
- ❌ ต้องมี rcedit.exe

**Implementation:**

#### 1. ดาวน์โหลด rcedit:
```bash
# ดาวน์โหลดจาก: https://github.com/electron/rcedit/releases
# หรือใช้ Chocolatey
choco install rcedit
```

#### 2. สร้าง build script:
```batch
@echo off
echo Building DocLens...
cargo build --release

echo Embedding icon...
rcedit target\release\doclens.exe --set-icon icon\icon.ico

echo Done!
```

#### 3. หรือใช้ build.rs:
```rust
// build.rs
fn main() {
    #[cfg(windows)]
    {
        // Embed icon in Windows resources
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon/icon.ico");
        res.compile().unwrap();
    }
}
```

เพิ่มใน Cargo.toml:
```toml
[build-dependencies]
winres = "0.1"
```

---

## 📊 เปรียบเทียบแนวทาง

| แนวทาง | ความยาก | Custom Title Bar | Icon Taskbar | Cross-platform |
|--------|----------|------------------|--------------|----------------|
| Windows API | ⚠️⚠️⚠️ สูง | ✅ | ✅ | ❌ Windows only |
| raw-window-handle | ⚠️⚠️ ปานกลาง | ✅ | ✅ | ✅ (conditional) |
| rcedit/winres | ⚠️ ง่าย | ✅ | ✅ | ✅ (build-time) |

---

## 🎯 คำแนะนำ

### สำหรับโปรเจคนี้:

**แนวทาง 3 (rcedit/winres) เหมาะที่สุด** เพราะ:
1. ✅ ง่ายที่สุด - แค่เพิ่ม build script
2. ✅ ไม่ต้องแตะโค้ด runtime
3. ✅ Icon embedded ในไฟล์ .exe
4. ✅ ทำงานกับ custom title bar
5. ✅ Cross-platform (conditional compilation)

### การทำงาน:
```
cargo build → doclens.exe (no icon)
     ↓
  winres/rcedit
     ↓
doclens.exe (with embedded icon)
     ↓
Windows loads .exe → อ่าน icon จาก resources
     ↓
แสดง icon ใน taskbar ✅
```

---

## 🚀 แผนการดำเนินงาน

### Step 1: ใช้ winres (แนะนำ)
1. เพิ่ม `winres` ใน build-dependencies
2. สร้างไฟล์ `build.rs`
3. Build ใหม่
4. Icon จะ embed ใน .exe อัตโนมัติ

### Step 2: ทดสอบ
1. Build release
2. Run doclens.exe
3. ตรวจสอบ taskbar icon
4. ตรวจสอบ Alt+Tab

### Step 3: Verify
1. ใช้ Resource Hacker ดู resources ใน .exe
2. ควรเห็น ICON group

---

## ⚠️ ข้อควรระวัง

1. **Icon size:** Windows ต้องการหลายขนาด (16x16, 32x32, 48x48, 256x256)
2. **Format:** .ico file ต้องมี multiple entries
3. **Build order:** winres ทำงานตอน compile, ก่อน link
4. **Cache:** Windows อาจ cache icon, ต้องรี login หรือ restart explorer.exe

---

คุณต้องการให้ฉันทำแนวทางไหนครับ? 

**แนะนำ: แนวทาง 3 (winres)** - ง่าย รวดเร็ว ได้ผลแน่นอน! 🎯
