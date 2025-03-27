#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(false)
            .with_inner_size([300.0, 450.0])
            .with_min_inner_size([300.0, 450.0])
            .with_max_inner_size([300.0, 450.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/Scramble_Logo_256.png")[..])
                    .expect("Failed to load icon"),
            )
            .with_resizable(false)
            .with_maximize_button(false),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "String Scrambler",
        native_options,
        Box::new(|cc| Ok(Box::new(StringScramblerApp::TemplateApp::new(cc)))),
    )
}
