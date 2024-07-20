#![windows_subsystem = "windows"]

mod gui;
mod preprocessor;
mod tests;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 450.0])
            .with_min_inner_size([300.0, 450.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Hentaigana Input",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::CalcuuubeGui::new(cc)))),
    )
}

// use kalk::parser;

// mod preprocessor;
// mod tests;

// fn main() {
//     let mut parser_context = parser::Context::new();
//     let precision = 53;
//     let result = parser::eval(&mut parser_context, &preprocessor::preprocessor("√2+2"), precision).unwrap().unwrap();
//     println!("{:?}", result.to_f64());
// }
