use egui::reset_button;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CalcuuubeGuiSettings {
    pub dark_mode: bool,
}

impl Default for CalcuuubeGuiSettings {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CalcuuubeGui {
    text: String,
    settings: CalcuuubeGuiSettings,

    #[serde(skip)]
    ime_text: String,
    #[serde(skip)]
    blocked_keys: Vec<String>,
}

impl Default for CalcuuubeGui {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            settings: CalcuuubeGuiSettings::default(),

            ime_text: "".to_owned(),
            blocked_keys: vec![],
        }
    }
}

impl CalcuuubeGui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //restore state
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        return Default::default();
    }
}

impl eframe::App for CalcuuubeGui {
    //save state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.settings.dark_mode);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.close_menu();
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                ui.menu_button("Settings", |ui| {
                    light_dark_buttons(self, ui);

                    if ui.button("Reset").clicked() {
                        self.settings = CalcuuubeGuiSettings::default();
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    unselectable_warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}

fn unselectable_warn_if_debug_build(ui: &mut egui::Ui) {
    if cfg!(debug_assertions) {
        ui.add(
            egui::Label::new(
                egui::RichText::new("⚠ Debug build ⚠")
                    .small()
                    .color(ui.visuals().warn_fg_color),
            )
            .selectable(false),
        );
    }
}

fn light_dark_buttons(hentaigana_input_gui: &mut CalcuuubeGui, ui: &mut egui::Ui) {
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.light_dark_radio_buttons(ui);
    hentaigana_input_gui.settings.dark_mode = visuals.dark_mode;
    set_theme(ui.ctx(), visuals.dark_mode);
}

fn set_theme(ctx: &egui::Context, dark_mode: bool) {
    if dark_mode {
        ctx.set_visuals(egui::Visuals::dark());
    } else {
        ctx.set_visuals(egui::Visuals::light());
    }
}
