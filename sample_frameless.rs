use eframe::egui;

pub fn configure_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_decorations(false), // Remove all OS decorations including title bar
        ..Default::default()
    }
}

pub fn render_custom_title_bar(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app_name: &str,
) -> egui::Response {
    let title_bar_height = 32.0;
    let title_bar_rect = {
        let mut rect = ui.max_rect();
        rect.max.y = rect.min.y + title_bar_height;
        rect
    };
    
    let title_bar_response = ui.interact(
        title_bar_rect, 
        egui::Id::new("title_bar"),
        egui::Sense::click()
    );
    
    // Make title bar draggable for window movement
    if title_bar_response.double_clicked() {
        let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    } else if title_bar_response.is_pointer_button_down_on() {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    
    // Draw title bar background
    ui.painter().rect_filled(
        title_bar_rect,
        0.0,
        egui::Color32::from_rgb(40, 40, 40),
    );
    
    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(title_bar_rect), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            
            // App icon and title
            ui.label(
                egui::RichText::new(format!("🖴 {}", app_name))
                    .size(16.0)
                    .color(egui::Color32::WHITE)
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Close button (standard Windows style)
                let close_response = render_title_button(
                    ui,
                    "×", // Unicode multiplication sign
                    title_bar_height,
                    Some(egui::Color32::from_rgb(196, 43, 28)), // Windows red
                );
                
                if close_response.clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                
                // Maximize/Restore button
                let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                let max_icon = if is_maximized { "🗗" } else { "🗖" };
                
                let max_response = render_title_button(
                    ui,
                    max_icon,
                    title_bar_height,
                    None,
                );
                
                if max_response.clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                }
                
                // Minimize button
                let min_response = render_title_button(
                    ui,
                    "🗕",
                    title_bar_height,
                    None,
                );
                
                if min_response.clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
                
                ui.add_space(12.0);
                
                // Subtitle
                ui.label(
                    egui::RichText::new("Disk Usage Monitor")
                        .size(12.0)
                        .color(egui::Color32::from_gray(160))
                );
            });
        });
    });
    
    title_bar_response
}

fn render_title_button(
    ui: &mut egui::Ui,
    text: &str,
    height: f32,
    hover_color: Option<egui::Color32>,
) -> egui::Response {
    let button_width = 46.0;
    let default_hover_color = egui::Color32::from_rgb(60, 60, 60);
    let hover_bg = hover_color.unwrap_or(default_hover_color);
    
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(button_width, height),
        egui::Sense::click(),
    );
    
    // Draw hover background
    if response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            hover_bg,
        );
    }
    
    // Draw icon
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0),
        egui::Color32::WHITE,
    );
    
    response
}
