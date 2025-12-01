# DocLens Pro - Commercial Edition

Thank you for choosing DocLens Pro! This commercial edition includes advanced features and professional support.

## Installation

The software has been installed to:
- Windows: `C:\Program Files\DocLens Pro`
- macOS: `/Applications/DocLens Pro.app`
- Linux: `/opt/doclens`

## License Activation

### First-Time Activation

1. Launch DocLens Pro
2. You will be prompted to enter your license key
3. Enter the license key provided in your purchase confirmation email
4. Enter your email address
5. Click "Activate"

### Trial Mode

If you don't have a license key yet, you can use the software in trial mode for 30 days with full functionality.

### Offline Activation

If you need to activate the software on a computer without internet access:

1. Go to Help → License → Offline Activation
2. Copy the machine ID shown
3. Visit https://www.yourcompany.com/activate-offline
4. Enter your license key and machine ID
5. Download the activation file
6. Import the activation file in the software

## Commercial Edition Features

### Advanced Features
- ✅ No watermarks
- ✅ Batch processing
- ✅ OCR text recognition
- ✅ Advanced annotation tools
- ✅ Form filling and signing
- ✅ Password protection
- ✅ Redaction tools
- ✅ Custom branding
- ✅ Command-line interface
- ✅ API access

### Professional Support
- 📧 Priority email support
- 📞 Phone support (business hours)
- 💬 Live chat support
- 📚 Comprehensive documentation
- 🎓 Video tutorials
- 🔄 Free updates for 1 year

## Getting Started

### Basic Usage

1. **Open a PDF**: File → Open or drag and drop
2. **Navigate**: Use mouse wheel, arrow keys, or thumbnail sidebar
3. **Zoom**: Ctrl + Mouse Wheel or zoom controls
4. **Search**: Ctrl+F to search text
5. **Annotate**: Use the annotation toolbar

### Advanced Features

#### Batch Processing
```
File → Batch Processing
- Convert multiple PDFs
- Apply watermarks
- Merge/split documents
- Extract pages
```

#### OCR Text Recognition
```
Tools → OCR → Recognize Text
- Convert scanned PDFs to searchable text
- Multiple language support
- High accuracy
```

#### Form Filling
```
Tools → Forms
- Fill interactive forms
- Save form data
- Export form data
```

#### Digital Signatures
```
Tools → Sign Document
- Create digital signatures
- Verify signatures
- Certificate management
```

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Open file | Ctrl+O |
| Save | Ctrl+S |
| Print | Ctrl+P |
| Search | Ctrl+F |
| Zoom in | Ctrl++ |
| Zoom out | Ctrl+- |
| Fit width | Ctrl+1 |
| Fit page | Ctrl+0 |
| Next page | Page Down / → |
| Previous page | Page Up / ← |
| First page | Home |
| Last page | End |

## Command-Line Interface

DocLens Pro includes a powerful CLI for automation:

```bash
# Convert PDF to images
doclens convert input.pdf --format png --output ./images/

# Merge PDFs
doclens merge file1.pdf file2.pdf --output merged.pdf

# Extract pages
doclens extract input.pdf --pages 1-5,10 --output output.pdf

# Add watermark
doclens watermark input.pdf --text "CONFIDENTIAL" --output output.pdf

# OCR processing
doclens ocr input.pdf --language eng --output output.pdf
```

## API Access

Integrate DocLens Pro into your applications:

```python
from doclens import PDFProcessor

# Initialize
processor = PDFProcessor(license_key="YOUR-LICENSE-KEY")

# Process PDF
result = processor.process(
    input_file="document.pdf",
    operations=["ocr", "watermark"],
    output_file="processed.pdf"
)
```

## Support

### Documentation
Visit our comprehensive documentation at:
https://docs.yourcompany.com/doclens

### Contact Support
- Email: support@yourcompany.com
- Phone: +1-XXX-XXX-XXXX (Mon-Fri, 9 AM - 5 PM EST)
- Live Chat: https://www.yourcompany.com/support
- Knowledge Base: https://kb.yourcompany.com

### Report Issues
Found a bug? Report it at:
https://www.yourcompany.com/support/report-issue

## Updates

### Automatic Updates
DocLens Pro checks for updates automatically. You can also check manually:
- Help → Check for Updates

### Manual Updates
Download the latest version from:
https://www.yourcompany.com/downloads

## License Management

### View License Information
Help → License → View License Info

### Transfer License
To transfer your license to another computer:
1. Deactivate on current computer: Help → License → Deactivate
2. Install on new computer
3. Activate with your license key

### Upgrade License
To upgrade from single-user to multi-user license:
https://www.yourcompany.com/upgrade

## Troubleshooting

### License Activation Issues
- Ensure you have internet connection
- Check that license key is entered correctly
- Verify email address matches purchase record
- Contact support if issues persist

### Performance Issues
- Close unnecessary applications
- Increase cache size: Settings → Performance
- Disable hardware acceleration if experiencing graphics issues

### Common Issues
- **PDF won't open**: Check file is not corrupted
- **Slow rendering**: Reduce quality in Settings
- **Annotations not saving**: Check file permissions

## Privacy & Security

DocLens Pro respects your privacy:
- No data collection without consent
- Local processing (no cloud upload required)
- Secure license validation
- Optional telemetry (can be disabled)

## System Requirements

### Minimum
- OS: Windows 10, macOS 10.14, Ubuntu 20.04
- RAM: 4 GB
- Storage: 500 MB
- Display: 1280x720

### Recommended
- OS: Windows 11, macOS 12+, Ubuntu 22.04
- RAM: 8 GB or more
- Storage: 1 GB
- Display: 1920x1080 or higher

## About

DocLens Pro is developed by [Your Company Name]
Version: 1.0.0
Copyright © 2025 [Your Company Name]. All rights reserved.

## Legal

This software is licensed under a commercial license agreement.
See LICENSE_COMMERCIAL.txt for full terms and conditions.
