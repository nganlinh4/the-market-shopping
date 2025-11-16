mod app;
mod i18n;
mod models;
mod fonts;
mod ui;

use app::MyApp;
use fonts::try_load_vietnamese_font;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "The Market Siêu thị việt hàn",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // Try to load Vietnamese font support
            let font_loaded = try_load_vietnamese_font(&cc.egui_ctx);
            if font_loaded {
                eprintln!("Successfully loaded Vietnamese font support");
            } else {
                eprintln!("Using system fonts with Unicode support for Vietnamese text");
            }

            Box::new(MyApp::new())
        }),
    )
}
