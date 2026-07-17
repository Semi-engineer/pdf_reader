/*!
Build script for DocLens
Embeds icon into Windows executable
*/

fn main() {
    // Only embed icon on Windows platform
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        
        // Set application icon (embedded in .exe)
        res.set_icon("icon/icon.ico");
        
        // Optional: Set application metadata
        res.set("ProductName", "DocLens");
        res.set("FileDescription", "A feature-rich PDF viewer");
        res.set("CompanyName", "DocLens Team");
        res.set("LegalCopyright", "Copyright © 2024-2026");
        
        // Compile resources
        match res.compile() {
            Ok(_) => println!("cargo:warning=✓ Icon embedded successfully"),
            Err(e) => {
                println!("cargo:warning=✗ Failed to embed icon: {}", e);
                println!("cargo:warning=  Make sure icon/icon.ico exists");
            }
        }
    }
    
    // On non-Windows platforms, do nothing
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:warning=Icon embedding is Windows-only");
    }
}
