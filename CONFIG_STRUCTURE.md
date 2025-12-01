# โครงสร้างไฟล์ Config ของ DocLens

## ตำแหน่งไฟล์

โปรแกรม DocLens จะสร้าง folder `config` ไว้ที่ตำแหน่งเดียวกับไฟล์ EXE เพื่อเก็บการตั้งค่าและข้อมูลต่างๆ

```
DocLens/
├── DocLens.exe (หรือ DocLensPro.exe)
├── config/
│   ├── settings.json      # การตั้งค่าโปรแกรม
│   └── license.json       # ข้อมูล license (Commercial Edition)
├── icon/
│   └── icon.ico
└── _internal/             # PyInstaller dependencies
```

## ไฟล์ Config

### 1. settings.json
เก็บการตั้งค่าของผู้ใช้และสถานะโปรแกรม

**ตัวอย่าง:**
```json
{
  "last_file": "C:/Documents/example.pdf",
  "last_page": 5,
  "last_zoom": 125,
  "sidebar_visible": true,
  "dark_mode": false,
  "window_geometry": {
    "x": 100,
    "y": 100,
    "width": 1200,
    "height": 800
  },
  "recent_files": [
    "C:/Documents/file1.pdf",
    "C:/Documents/file2.pdf"
  ]
}
```

**ข้อมูลที่เก็บ:**
- `last_file`: ไฟล์ PDF ที่เปิดล่าสุด
- `last_page`: หน้าที่เปิดค่าง
- `last_zoom`: ระดับการซูม (%)
- `sidebar_visible`: แสดง sidebar หรือไม่
- `dark_mode`: โหมดมืด
- `window_geometry`: ตำแหน่งและขนาดหน้าต่าง
- `recent_files`: รายการไฟล์ที่เปิดล่าสุด

### 2. license.json (Commercial Edition เท่านั้น)
เก็บข้อมูล license และการ activation

**ตัวอย่าง (Trial):**
```json
{
  "type": "trial",
  "install_date": "2025-12-01T10:30:00",
  "product": "DocLens Pro",
  "version": "1.0.0"
}
```

**ตัวอย่าง (Full License):**
```json
{
  "type": "full",
  "key": "DOCL-A1B2-C3D4-5678-9ABC",
  "email": "customer@example.com",
  "activation_date": "2025-12-01T10:30:00",
  "product": "DocLens Pro",
  "version": "1.0.0"
}
```

**ข้อมูลที่เก็บ:**
- `type`: ประเภท license (trial/full)
- `key`: License key (สำหรับ full license)
- `email`: Email ของผู้ใช้
- `install_date`: วันที่ติดตั้ง (สำหรับ trial)
- `activation_date`: วันที่ activate (สำหรับ full license)
- `product`: ชื่อผลิตภัณฑ์
- `version`: เวอร์ชัน

## ข้อดีของการใช้ Config Folder

### 1. Portable Application
- สามารถคัดลอกโปรแกรมไปใช้ที่เครื่องอื่นได้
- ไม่ต้องพึ่ง User Profile หรือ Registry
- เหมาะสำหรับใช้งานจาก USB Drive

### 2. Easy Backup
- Backup ง่าย แค่คัดลอก folder config
- Restore ง่าย แค่วาง folder config กลับมา

### 3. Multi-User Friendly
- แต่ละ user สามารถมี config แยกกันได้
- ไม่ conflict กับ user อื่นในเครื่องเดียวกัน

### 4. Easy Troubleshooting
- ลบ folder config เพื่อ reset การตั้งค่า
- ตรวจสอบปัญหาได้ง่าย

## การใช้งาน

### Reset การตั้งค่า
ลบไฟล์ `config/settings.json` แล้วเปิดโปรแกรมใหม่

### Reset License (Commercial)
ลบไฟล์ `config/license.json` แล้วเปิดโปรแกรมใหม่

### Backup การตั้งค่า
คัดลอก folder `config` ไปเก็บไว้

### Restore การตั้งค่า
วาง folder `config` กลับมาที่เดิม

## หมายเหตุ

- Folder `config` จะถูกสร้างอัตโนมัติเมื่อเปิดโปรแกรมครั้งแรก
- ไฟล์ config เป็น JSON format อ่านและแก้ไขได้ด้วย text editor
- ไม่ควรแก้ไข `license.json` ด้วยตนเอง (อาจทำให้ license ใช้งานไม่ได้)
- การลบ config จะไม่ส่งผลต่อไฟล์ PDF ของคุณ

## สำหรับ Developer

### อ่าน Config
```python
from main import load_settings

settings = load_settings()
last_file = settings.get('last_file')
```

### เขียน Config
```python
from main import save_settings

settings = {
    'last_file': 'path/to/file.pdf',
    'last_zoom': 150
}
save_settings(settings)
```

### ตรวจสอบ License
```python
from license_manager import LicenseManager

lm = LicenseManager()
if lm.is_licensed():
    print("Licensed!")
else:
    print("Not licensed")
```
