# คู่มือการ Deploy DocLens สำหรับ Commercial Edition

## ภาพรวม

ระบบ deployment ของ DocLens ได้รับการปรับปรุงให้รองรับทั้ง Free Edition และ Commercial Edition โดยมีฟีเจอร์ดังนี้:

### Free Edition
- ใช้ MIT License (โอเพนซอร์ส)
- ไม่ต้องมีการ activate
- ฟีเจอร์พื้นฐาน
- แจกจ่ายได้อย่างอิสระ

### Commercial Edition
- ใช้ Proprietary License
- ต้องมีการ activate ด้วย license key
- ฟีเจอร์ขั้นสูงครบถ้วน
- ระยะทดลองใช้ 30 วัน
- รองรับ custom branding
- มี technical support

## การใช้งาน Deploy Script

### Windows (deploy.bat)

```cmd
deploy.bat
```

1. เลือก Edition (1=Free, 2=Commercial)
2. หาก Commercial: กรอกข้อมูล
   - ชื่อบริษัท
   - ชื่อผลิตภัณฑ์
   - เวอร์ชัน
   - License Server URL (ถ้ามี)
3. เลือกว่าจะ build executable หรือไม่
4. หาก Commercial: เลือกว่าจะสร้าง installer หรือไม่

### Linux/macOS (deploy.sh)

```bash
chmod +x deploy.sh
./deploy.sh
```

ขั้นตอนเหมือนกับ Windows

## ไฟล์ที่สร้างขึ้น

### Free Edition
- `dist/doclens-1.0.0-py3-none-any.whl` - Python package
- `dist/DocLens.exe` - Executable (Windows)
- `dist/DocLens` - Executable (Linux/macOS)

### Commercial Edition
- `dist/DocLens Pro.exe` - Executable (Windows)
- `dist/DocLens Pro_Setup.exe` - Installer (Windows)
- `dist/DocLens Pro.app` - App bundle (macOS)
- `dist/DocLens_Pro_Setup.dmg` - DMG installer (macOS)
- `build_config.json` - Configuration file

## License Management

### การสร้าง License Key

```python
python license_manager.py
```

หรือใช้ในโค้ด:

```python
from license_manager import generate_license_key

license_key = generate_license_key("customer@email.com")
print(f"License Key: {license_key}")
```

### รูปแบบ License Key
```
DOCL-A1B2-C3D4-5678-9ABC
│    │    │    │    └─ Checksum
│    │    │    └────── Timestamp
│    │    └─────────── Random
│    └──────────────── Customer ID hash
└───────────────────── Product code
```

### การ Validate License

```python
from license_manager import LicenseManager

lm = LicenseManager()

# ตรวจสอบว่าเป็น commercial edition หรือไม่
if lm.is_commercial_edition():
    # ตรวจสอบ license
    if lm.is_licensed():
        print("Licensed!")
    else:
        print("Not licensed")
        
    # ตรวจสอบ trial
    if lm.is_trial_active():
        days = lm.get_trial_days_remaining()
        print(f"Trial: {days} days remaining")
```

### การ Activate License

```python
from license_manager import LicenseManager

lm = LicenseManager()

# Activate with license key
success = lm.activate_license(
    license_key="DOCL-A1B2-C3D4-5678-9ABC",
    email="customer@email.com"
)

if success:
    print("Activation successful!")
else:
    print("Activation failed!")
```

### การเริ่ม Trial

```python
from license_manager import LicenseManager

lm = LicenseManager()

if lm.start_trial():
    print("Trial started!")
    print(f"Days remaining: {lm.get_trial_days_remaining()}")
```

## License Server (Optional)

หากต้องการใช้ online activation สามารถตั้ง license server ได้:

### API Endpoints

```
POST /activate
{
  "license_key": "DOCL-...",
  "email": "customer@email.com",
  "product": "DocLens Pro",
  "version": "1.0.0",
  "machine_id": "abc123..."
}

Response:
{
  "success": true,
  "message": "Activation successful"
}
```

### ตัวอย่าง License Server (Flask)

```python
from flask import Flask, request, jsonify
import sqlite3

app = Flask(__name__)

@app.route('/activate', methods=['POST'])
def activate():
    data = request.json
    license_key = data.get('license_key')
    email = data.get('email')
    machine_id = data.get('machine_id')
    
    # Validate license key in database
    # Check activation limit
    # Save activation record
    
    return jsonify({
        'success': True,
        'message': 'Activation successful'
    })

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
```

## การสร้าง Installer

### Windows (Inno Setup)

1. ติดตั้ง Inno Setup: https://jrsoftware.org/isdl.php
2. แก้ไข `installer_commercial.iss`:
   - เปลี่ยน `{YOUR-GUID-HERE}` เป็น GUID ของคุณ
   - อัพเดทข้อมูลบริษัท
3. รัน deploy script และเลือกสร้าง installer

### macOS (DMG)

1. ติดตั้ง create-dmg:
```bash
brew install create-dmg
```

2. รัน deploy script และเลือกสร้าง installer

### Linux (AppImage/DEB)

สามารถใช้เครื่องมือเช่น:
- `fpm` สำหรับสร้าง DEB/RPM
- `AppImageTool` สำหรับสร้าง AppImage

## การปรับแต่ง Branding

แก้ไขไฟล์ `build_config.json`:

```json
{
  "edition": "commercial",
  "company": "Your Company",
  "product": "DocLens Pro",
  "version": "1.0.0",
  "license_type": "proprietary",
  "license_server": "https://license.yourcompany.com",
  "branding": {
    "logo": "assets/logo.png",
    "icon": "assets/icon.ico",
    "splash": "assets/splash.png",
    "colors": {
      "primary": "#0066CC",
      "secondary": "#FF6600"
    }
  },
  "features": {
    "watermark": false,
    "trial_days": 30,
    "require_activation": true,
    "advanced_features": true,
    "ocr": true,
    "batch_processing": true,
    "api_access": true
  }
}
```

## การทดสอบ

### ทดสอบ Free Edition
```bash
python main.py
```

### ทดสอบ Commercial Edition (Trial)
```bash
python main.py --start-trial
```

### ทดสอบ Commercial Edition (Licensed)
```bash
python main.py --activate "DOCL-..." "email@example.com"
```

## การแจกจ่าย

### Free Edition
- อัพโหลดไปยัง PyPI: `twine upload dist/*`
- แจกจ่าย executable ได้อย่างอิสระ
- เผยแพร่ source code บน GitHub

### Commercial Edition
- แจกจ่ายผ่าน website ของคุณ
- ส่ง license key ให้ลูกค้าทาง email
- ให้ technical support
- อัพเดทผ่าน auto-update system

## Checklist ก่อน Deploy

### Free Edition
- [ ] ทดสอบ functionality ทั้งหมด
- [ ] อัพเดท README.md
- [ ] ตรวจสอบ LICENSE (MIT)
- [ ] Build และทดสอบ executable
- [ ] เตรียม documentation

### Commercial Edition
- [ ] ทดสอบ functionality ทั้งหมด
- [ ] ตั้งค่า license server (ถ้าใช้)
- [ ] สร้าง license keys สำหรับลูกค้า
- [ ] ทดสอบ activation process
- [ ] ทดสอบ trial mode
- [ ] สร้าง installer
- [ ] เตรียม support documentation
- [ ] ตั้งค่า payment gateway
- [ ] เตรียม customer portal

## การ Support

### Free Edition
- GitHub Issues
- Community forum
- Documentation

### Commercial Edition
- Priority email support
- Phone support
- Live chat
- Dedicated account manager (enterprise)
- Custom development (enterprise)

## ข้อควรระวัง

1. **License Keys**: เก็บรักษา algorithm การสร้าง license key ให้ปลอดภัย
2. **Activation Limits**: จำกัดจำนวนครั้งที่ activate ได้
3. **Trial Period**: ป้องกันการ reset trial period
4. **Code Obfuscation**: ใช้ PyArmor หรือเครื่องมืออื่นเพื่อป้องกัน reverse engineering
5. **Update Mechanism**: มีระบบ auto-update ที่ปลอดภัย
6. **Analytics**: เก็บข้อมูลการใช้งาน (ด้วยความยินยอมของผู้ใช้)

## ตัวอย่างการใช้งาน

### Scenario 1: Deploy Free Edition
```bash
./deploy.sh
# เลือก 1 (Free Edition)
# เลือก y (build executable)
# ได้ไฟล์: dist/DocLens
```

### Scenario 2: Deploy Commercial Edition
```bash
./deploy.sh
# เลือก 2 (Commercial Edition)
# กรอกข้อมูลบริษัท
# เลือก y (build executable)
# เลือก y (create installer)
# ได้ไฟล์: dist/DocLens_Pro_Setup.dmg
```

### Scenario 3: Generate License Keys
```bash
python license_manager.py
# กรอก customer email
# ได้ license key: DOCL-A1B2-C3D4-5678-9ABC
```

## สรุป

ระบบ deployment ที่ปรับปรุงแล้วรองรับทั้งการใช้งานแบบ free และ commercial โดยมีระบบ license management ที่สมบูรณ์ รองรับ trial period และ online/offline activation พร้อมเครื่องมือสร้าง installer สำหรับทุก platform
