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
    settings: CalcuuubeGuiSettings,

    #[serde(skip)]
    input_text: String,
    #[serde(skip)]
    result_text: String,
    #[serde(skip)]
    clicked: bool,
    #[serde(skip)]
    parser_context: kalk::parser::Context,
}

impl Default for CalcuuubeGui {
    fn default() -> Self {
        Self {
            settings: CalcuuubeGuiSettings::default(),

            input_text: "".to_owned(),
            result_text: "".to_owned(),
            clicked: false,
            parser_context: kalk::parser::Context::new(),
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
            ui.vertical(|ui| {
                let input_textedit = ui.add(
                    egui::TextEdit::singleline(&mut self.input_text)
                        .min_size([0.0, 40.0].into())
                        .horizontal_align(egui::Align::Max)
                        .font(egui::TextStyle::Heading)
                        .id("calcuuube_textedit".into()),
                );

                if input_textedit.changed() {
                    calculate_result(self);
                }

                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(&self.result_text).size(40.0),
                    ))
                });

                capture_events(self, ui);

                let grid_column_count = 4;
                let grid_row_count = 6;
                egui::Grid::new("main_ui_grid")
                    .num_columns(grid_column_count)
                    .min_col_width(
                        (ui.available_width()
                            - ui.spacing().item_spacing.x * (grid_column_count as f32 - 1.0))
                            / grid_column_count as f32,
                    )
                    .min_row_height(
                        (ui.available_height()
                            - ui.spacing().item_spacing.y * (grid_row_count as f32 - 1.0))
                            / grid_row_count as f32,
                    )
                    .show(ui, |ui| {
                        make_button(self, ui, "√");
                        make_button(self, ui, "");
                        make_button(self, ui, "^");
                        make_button(self, ui, "x²");
                        ui.end_row();
                        make_button(self, ui, "C");
                        make_button(self, ui, "(");
                        make_button(self, ui, ")");
                        make_button(self, ui, "÷");
                        ui.end_row();
                        make_button(self, ui, "7");
                        make_button(self, ui, "8");
                        make_button(self, ui, "9");
                        make_button(self, ui, "*");
                        ui.end_row();
                        make_button(self, ui, "4");
                        make_button(self, ui, "5");
                        make_button(self, ui, "6");
                        make_button(self, ui, "-");
                        ui.end_row();
                        make_button(self, ui, "1");
                        make_button(self, ui, "2");
                        make_button(self, ui, "3");
                        make_button(self, ui, "+");
                        ui.end_row();
                        make_button(self, ui, "0");
                        make_button(self, ui, ".");
                        make_backspace_button(self, ui);
                        make_button(self, ui, "=");
                        ui.end_row();
                    });
            });
        });
    }
}

fn make_button(calcuuube_gui: &mut CalcuuubeGui, ui: &mut egui::Ui, operation: &str) {
    let new_button = ui.add_sized(
        ui.available_size(),
        egui::Button::new(egui::RichText::new(operation)),
    );

    if calcuuube_gui.clicked && new_button.is_pointer_button_down_on() {
        calcuuube_gui.clicked = false;
        calcuuube_gui.input_text += operation;
        calculate_result(calcuuube_gui);
    }
}

fn make_backspace_button(calcuuube_gui: &mut CalcuuubeGui, ui: &mut egui::Ui) {
    let new_button = ui.add_sized(
        ui.available_size(),
        egui::Button::new(egui::RichText::new("<-")),
    );

    if calcuuube_gui.clicked && new_button.is_pointer_button_down_on() {
        calcuuube_gui.clicked = false;
        calcuuube_gui.input_text.pop();
        calculate_result(calcuuube_gui);
    }
}

fn calculate_result(calcuuube_gui: &mut CalcuuubeGui) {
    let calculation = crate::calculate::calculate_string_to_string(
        &calcuuube_gui.input_text,
        &mut calcuuube_gui.parser_context,
    );
    if calculation.is_some() {
        calcuuube_gui.result_text = calculation.unwrap();
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

fn light_dark_buttons(calcuuube_gui: &mut CalcuuubeGui, ui: &mut egui::Ui) {
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.light_dark_radio_buttons(ui);
    calcuuube_gui.settings.dark_mode = visuals.dark_mode;
    set_theme(ui.ctx(), visuals.dark_mode);
}

fn set_theme(ctx: &egui::Context, dark_mode: bool) {
    if dark_mode {
        ctx.set_visuals(egui::Visuals::dark());
    } else {
        ctx.set_visuals(egui::Visuals::light());
    }
}

fn capture_events(calcuuube_gui: &mut CalcuuubeGui, ui: &mut egui::Ui) {
    ui.input_mut(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(_) => {}
                egui::Event::PointerButton {
                    pos: _,
                    button,
                    pressed,
                    modifiers: _,
                } => {
                    if button == &egui::PointerButton::Primary && *pressed {
                        calcuuube_gui.clicked = true;
                    }
                }
                _ => {}
            }
        }
    });
}
