pub fn set_font_styles(settings: &mut crate::gui::CalcuuubeGuiSettings, ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Name("textedit".into()),
        egui::FontId::new(
            settings.textedit_font_size,
            egui::FontFamily::Name("Noto".into()),
        ),
    );
    style.text_styles.insert(
        egui::TextStyle::Name("button".into()),
        egui::FontId::new(
            settings.button_font_size,
            egui::FontFamily::Name("Noto".into()),
        ),
    );
    ctx.set_style(style);
}

pub fn add_font_files(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "NotoSansMono".into(),
        egui::FontData::from_owned(get_noto_sans_mono()),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Noto".into()),
        vec!["NotoSansMono".into()],
    );
    cc.egui_ctx.set_fonts(fonts);
}

fn get_noto_sans_mono() -> Vec<u8> {
    return include_bytes!("../assets/NotoSansMono-Regular.ttf").to_vec();
}
