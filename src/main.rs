/*!
DocLens - A Feature-Rich PDF Viewer in Rust
Main entry point
*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod app;
mod config;
mod pdf_document;
mod page_cache;
mod render_worker;
mod thumbnail_manager;
mod annotation;
mod search;
mod ui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(false)          // ← Custom frameless title bar
            .with_transparent(false)
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "DocLens",
        options,
        Box::new(|cc| Ok(Box::new(app::DocLensApp::new(cc)))),
    )
}

fn load_icon() -> egui::viewport::IconData {
    // Try multiple icon paths
    let icon_paths = [
        "icon/icon.ico",
        "./icon/icon.ico",
        "icon.ico",
    ];
    
    for icon_path in &icon_paths {
        let path = std::path::Path::new(icon_path);
        eprintln!("Trying icon path: {} (exists: {})", icon_path, path.exists());
        
        if path.exists() {
            match std::fs::read(path) {
                Ok(icon_bytes) => {
                    eprintln!("Read {} bytes from {}", icon_bytes.len(), icon_path);
                    
                    // Try to load as ICO first
                    match ico::IconDir::read(std::io::Cursor::new(&icon_bytes)) {
                        Ok(ico_dir) => {
                            eprintln!("ICO file contains {} entries", ico_dir.entries().len());
                            
                            // Try to find a 32x32 or 48x48 icon first
                            let mut selected_entry = None;
                            for entry in ico_dir.entries() {
                                let w = entry.width();
                                let h = entry.height();
                                eprintln!("  Entry: {}x{}", w, h);
                                
                                if w == 32 && h == 32 {
                                    selected_entry = Some(entry);
                                    break;
                                } else if w == 48 && h == 48 && selected_entry.is_none() {
                                    selected_entry = Some(entry);
                                }
                            }
                            
                            // Fallback to first entry
                            let entry = if let Some(e) = selected_entry {
                                e
                            } else if let Some(e) = ico_dir.entries().first() {
                                e
                            } else {
                                eprintln!("✗ No entries in ICO file");
                                continue;
                            };
                            
                            match entry.decode() {
                                Ok(image) => {
                                    let width = image.width();
                                    let height = image.height();
                                    let rgba = image.rgba_data().to_vec();
                                    
                                    eprintln!("✓ Successfully loaded icon: {}x{} ({} bytes)", width, height, rgba.len());
                                    
                                    return egui::viewport::IconData {
                                        rgba,
                                        width,
                                        height,
                                    };
                                }
                                Err(e) => eprintln!("✗ Failed to decode icon entry: {}", e),
                            }
                        }
                        Err(e) => eprintln!("✗ Failed to parse ICO: {}", e),
                    }
                    
                    // Fallback: try as regular image
                    match image::load_from_memory(&icon_bytes) {
                        Ok(img) => {
                            let rgba_img = img.to_rgba8();
                            let (width, height) = rgba_img.dimensions();
                            
                            eprintln!("✓ Loaded icon as regular image: {}x{}", width, height);
                            
                            return egui::viewport::IconData {
                                rgba: rgba_img.into_raw(),
                                width,
                                height,
                            };
                        }
                        Err(e) => eprintln!("✗ Failed to load as image: {}", e),
                    }
                }
                Err(e) => eprintln!("✗ Failed to read {}: {}", icon_path, e),
            }
        }
    }
    
    eprintln!("⚠ No icon found, using default fallback icon");
    
    // Default: create a simple colored square as fallback
    let size = 32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];
    
    // Draw a simple blue square with border (DocLens blue)
    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            let is_border = x == 0 || y == 0 || x == size - 1 || y == size - 1;
            let is_inner_border = x <= 2 || y <= 2 || x >= size - 3 || y >= size - 3;
            
            if is_border {
                // Dark border
                rgba[idx] = 40;
                rgba[idx + 1] = 40;
                rgba[idx + 2] = 60;
                rgba[idx + 3] = 255;
            } else if is_inner_border {
                // Blue border
                rgba[idx] = 88;
                rgba[idx + 1] = 112;
                rgba[idx + 2] = 214;
                rgba[idx + 3] = 255;
            } else {
                // Light blue center
                rgba[idx] = 108;
                rgba[idx + 1] = 182;
                rgba[idx + 2] = 255;
                rgba[idx + 3] = 255;
            }
        }
    }
    
    egui::viewport::IconData {
        rgba,
        width: size,
        height: size,
    }
}
