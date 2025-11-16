use egui::FontData;

pub fn try_load_vietnamese_font(ctx: &egui::Context) -> bool {
    // Try multiple approaches to load a font with Vietnamese support

    // Method 1: Try common system font names
    let system_fonts = [
        "C:\\Windows\\Fonts\\arial.ttf",
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\tahoma.ttf",
        "/System/Library/Fonts/Arial.ttf",
        "/System/Library/Fonts/Helvetica.ttc",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    ];

    for font_path in &system_fonts {
        if let Ok(font_data) = std::fs::read(font_path) {
            if load_font_into_egui(ctx, &font_data, "SystemFont") {
                eprintln!("Loaded system font: {}", font_path);
                return true;
            }
        }
    }

    // Method 2: Try to create a simple Unicode-capable font configuration
    setup_unicode_font_fallback(ctx);
    false
}

fn load_font_into_egui(ctx: &egui::Context, font_data: &[u8], font_name: &str) -> bool {
    let mut font_def = egui::FontDefinitions::empty();

    font_def.font_data.insert(
        font_name.to_owned(),
        FontData::from_owned(font_data.to_vec()),
    );

    // Set as primary font with Unicode support
    font_def.families.insert(
        egui::FontFamily::default(),
        vec![font_name.to_owned()],
    );

    // Also set for monospace
    font_def.families.insert(
        egui::FontFamily::Monospace,
        vec![font_name.to_owned()],
    );

    ctx.set_fonts(font_def);
    true
}

fn setup_unicode_font_fallback(ctx: &egui::Context) {
    let mut font_def = egui::FontDefinitions::empty();

    // Configure font families with proper Unicode fallbacks
    font_def.families.insert(
        egui::FontFamily::default(),
        vec![
            "Segoe UI".to_owned(),
            "Arial".to_owned(),
            "DejaVu Sans".to_owned(),
            "Liberation Sans".to_owned(),
            "sans-serif".to_owned(),
        ],
    );

    font_def.families.insert(
        egui::FontFamily::Monospace,
        vec![
            "Consolas".to_owned(),
            "Courier New".to_owned(),
            "monospace".to_owned(),
        ],
    );

    ctx.set_fonts(font_def);
    eprintln!("Configured Unicode font fallback chain");
}
