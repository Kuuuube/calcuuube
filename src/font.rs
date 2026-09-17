pub fn get_textedit_font(settings: &crate::gui::CalcuuubeGuiSettings) -> egui::FontId{
    return egui::FontId::new(
        settings.textedit_font_size,
        egui::FontFamily::Name("Noto".into()),
    );
}

pub fn get_button_font(settings: &crate::gui::CalcuuubeGuiSettings) -> egui::FontId{
    return egui::FontId::new(
        settings.button_font_size,
        egui::FontFamily::Name("Noto".into()),
    );
}

pub fn add_font_files(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "NotoSansMono".into(),
        egui::FontData::from_owned(get_noto_sans_mono()).into(),
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
