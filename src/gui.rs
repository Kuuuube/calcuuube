use std::str::Chars;

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
    input_text_cursor_position: usize,
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
            input_text_cursor_position: 0,
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
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::Absolute {
                        initial: 40.0,
                        range: (40.0..=40.0).into(),
                    })
                    .size(egui_extras::Size::Absolute {
                        initial: 40.0,
                        range: (80.0..=80.0).into(),
                    })
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            let font = egui::FontId {
                                size: 35.0,
                                family: egui::FontFamily::Monospace,
                            };
                            let input_textedit =
                                egui::TextEdit::singleline(&mut self.input_text)
                                    .min_size([0.0, 40.0].into())
                                    .horizontal_align(egui::Align::Max)
                                    .font(font)
                                    .id("calcuuube_textedit".into())
                                    .vertical_align(egui::Align::Center)
                                    .show(ui);

                            if input_textedit.response.changed() {
                                calculate_result(self);
                            }

                            match input_textedit.cursor_range {
                                Some(some) => {
                                    self.input_text_cursor_position =
                                        some.primary.ccursor.index;
                                }
                                None => (),
                            }
                        });

                        strip.cell(|ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                let font_size = find_fit_text(
                                    ui,
                                    &self.result_text,
                                    egui::FontFamily::Monospace,
                                    35,
                                    ui.available_width(),
                                );
                                ui.add(
                                    egui::Label::new(egui::RichText::new(&self.result_text).font(
                                        egui::FontId {
                                            size: font_size,
                                            family: egui::FontFamily::Monospace,
                                        },
                                    ))
                                    .wrap_mode(egui::TextWrapMode::Truncate),
                                );
                            });
                        });
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
                        make_button(self, ui, "<-");
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
        match operation {
            "C" => {
                calcuuube_gui.input_text = Default::default();
                calcuuube_gui.input_text_cursor_position = Default::default();
            }
            "<-" => {
                let mut chars_vec: Vec<char> = calcuuube_gui.input_text.chars().collect();
                if calcuuube_gui.input_text_cursor_position > 0 {
                    let removal_position = calcuuube_gui.input_text_cursor_position - 1;
                    if chars_vec.len() > 0 && removal_position < chars_vec.len() {
                        chars_vec.remove(removal_position);
                        calcuuube_gui.input_text_cursor_position -= 1;
                        calcuuube_gui.input_text = chars_vec.into_iter().collect();
                    }
                }
            }
            "=" => {}
            _ => {
                let operation_chars: Chars = match operation {
                    "x²" => "^2".chars(),
                    _ => operation.chars(),
                };
                let mut chars_vec: Vec<char> = calcuuube_gui.input_text.chars().collect();
                for operation_char in operation_chars {
                    chars_vec.insert(calcuuube_gui.input_text_cursor_position, operation_char);
                    calcuuube_gui.input_text_cursor_position += 1;
                }
                calcuuube_gui.input_text = chars_vec.into_iter().collect();
            }
        }
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

fn find_fit_text(
    ui: &mut egui::Ui,
    input_string: &str,
    font_family: egui::FontFamily,
    max_font_size: i32,
    target_width: f32,
) -> f32 {
    for i in (5..max_font_size).rev() {
        let font_id = egui::FontId {
            size: i as f32,
            family: font_family.clone(),
        };
        let mut total_width = 0.0;
        for char in (input_string.to_owned() + "  ").chars() {
            total_width += ui.fonts(|f| f.glyph_width(&font_id, char));
        }
        if total_width <= target_width {
            return i as f32;
        }
    }
    return 1.0;
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
