use egui::FontData;

// Include emoji font data directly in the binary
// This ensures emoji support works on any computer without system font dependencies
const NOTO_EMOJI_FONT: &[u8] = include_bytes!("../fonts/NotoColorEmoji.ttf");

pub fn try_load_vietnamese_font(ctx: &egui::Context) -> bool {
    // Try to load a system font with Vietnamese support
    let system_fonts = [
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\arial.ttf",
        "C:\\Windows\\Fonts\\tahoma.ttf",
        "/System/Library/Fonts/Arial.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];

    for font_path in &system_fonts {
        if let Ok(font_data) = std::fs::read(font_path) {
            setup_fonts_with_emoji_and_system(ctx, &font_data);
            eprintln!("Loaded system font from: {}", font_path);
            return true;
        }
    }

    // Fallback: use default fonts with emoji
    setup_fonts_with_emoji_only(ctx);
    false
}

fn setup_fonts_with_emoji_and_system(ctx: &egui::Context, system_font_data: &[u8]) {
    let mut font_def = egui::FontDefinitions::default();
    
    // Add system font for Vietnamese text
    font_def.font_data.insert(
        "SystemFont".to_owned(),
        FontData::from_owned(system_font_data.to_vec()),
    );
    
    // Add the bundled emoji font as a fallback
    font_def.font_data.insert(
        "NotoEmoji".to_owned(),
        FontData::from_owned(NOTO_EMOJI_FONT.to_vec()),
    );
    
    // Set up font families: system font first, then emoji as fallback
    for family in font_def.families.values_mut() {
        family.insert(0, "SystemFont".to_owned());
        family.push("NotoEmoji".to_owned());
    }
    
    ctx.set_fonts(font_def);
    eprintln!("Loaded system font with emoji fallback support");
}

fn setup_fonts_with_emoji_only(ctx: &egui::Context) {
    let mut font_def = egui::FontDefinitions::default();
    
    // Add the bundled emoji font as fallback only
    font_def.font_data.insert(
        "NotoEmoji".to_owned(),
        FontData::from_owned(NOTO_EMOJI_FONT.to_vec()),
    );
    
    // Add emoji as fallback in all families
    for family in font_def.families.values_mut() {
        family.push("NotoEmoji".to_owned());
    }
    
    ctx.set_fonts(font_def);
    eprintln!("Using default fonts with emoji fallback");
}


