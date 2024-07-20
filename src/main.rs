#![windows_subsystem = "windows"]

mod calculate;
mod gui;
mod preprocessor;
mod tests;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 450.0])
            .with_min_inner_size([300.0, 450.0])
            .with_max_inner_size([300.0, 450.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Calcuuube",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::CalcuuubeGui::new(cc)))),
    )
}
