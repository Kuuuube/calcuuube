use std::str::Chars;

use egui::ThemePreference;

use crate::{context::CalculatorContext, font::{get_button_font, get_textedit_font}};

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub struct CalcuuubeGuiSettings {
    pub theme_preference: ThemePreference,
    pub button_font_size: f32,
    pub textedit_font_size: f32,

    button_font_size_string: String,
    textedit_font_size_string: String,
}

impl Default for CalcuuubeGuiSettings {
    fn default() -> Self {
        Self {
            theme_preference: ThemePreference::System,
            button_font_size: 20.0,
            textedit_font_size: 35.0,

            button_font_size_string: "20".to_owned(),
            textedit_font_size_string: "35".to_owned(),
        }
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
    calculation_error: bool,
    #[serde(skip)]
    clicked: bool,
    #[serde(skip)]
    parser_context: CalculatorContext,
}

impl Default for CalcuuubeGui {
    fn default() -> Self {
        Self {
            settings: CalcuuubeGuiSettings::default(),

            input_text: "".to_owned(),
            input_text_cursor_position: 0,
            result_text: "".to_owned(),
            calculation_error: false,
            clicked: false,
            parser_context: CalculatorContext::Kalker(kalk::parser::Context::new()),
        }
    }
}

impl CalcuuubeGui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::font::add_font_files(cc);

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

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Settings", |ui| {
                    light_dark_buttons(self, ui);

                    ui.add(egui::Label::new("Calculator Engine:").selectable(false));
                    ui.horizontal(|ui| {
                        let kalker_active = matches!(self.parser_context, CalculatorContext::Kalker(_));
                        let fend_active = matches!(self.parser_context, CalculatorContext::Fend(_));
                        let kalker_button = ui.radio(kalker_active, "Kalker");
                        let fend_button = ui.radio(fend_active, "Fend");

                        if kalker_button.clicked() && !kalker_active {
                            self.parser_context = CalculatorContext::Kalker(kalk::parser::Context::new());
                        }

                        if fend_button.clicked() && !fend_active {
                            self.parser_context = CalculatorContext::Fend(fend_core::Context::new());
                        }

                    });
                    ui.end_row();

                    if ui.button("Reset").clicked() {
                        self.settings = CalcuuubeGuiSettings::default();
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.calculation_error {
                        calculation_error(ui);
                    }
                    unselectable_warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let textedit_font_id = get_textedit_font(&self.settings);
            let vertical_space_required = ui.fonts_mut(|f| f.row_height(&textedit_font_id));

            ui.vertical(|ui| {
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::Absolute {
                        initial: vertical_space_required,
                        range: (vertical_space_required..=vertical_space_required).into(),
                    })
                    .size(egui_extras::Size::Absolute {
                        initial: vertical_space_required,
                        range: (vertical_space_required..=vertical_space_required).into(),
                    })
                    .vertical(|mut strip| {
                        let textedit_id: egui::Id = "calcuuube_textedit".into();
                        strip.cell(|ui| {
                            let input_textedit = egui::TextEdit::singleline(&mut self.input_text)
                                .min_size([ui.available_width(), 40.0].into())
                                .horizontal_align(egui::Align::Max)
                                .font(get_textedit_font(&self.settings))
                                .id(textedit_id)
                                .vertical_align(egui::Align::Center)
                                .cursor_at_end(true)
                                .show(ui);

                            if input_textedit.response.changed() {
                                self.input_text =
                                    crate::realtimeprocessor::realtimeprocess(&self.input_text);
                                calculate_result(self);
                            }

                            match input_textedit.cursor_range {
                                Some(some) => {
                                    self.input_text_cursor_position = some.primary.index.0;
                                }
                                None => (),
                            }
                        });

                        strip.cell(|ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                let font_size = find_fit_text(
                                    ui,
                                    &self.result_text,
                                    egui::FontFamily::Name("Noto".into()),
                                    self.settings.textedit_font_size,
                                    ui.available_size_before_wrap().x,
                                );
                                ui.add(
                                    egui::Label::new(egui::RichText::new(&self.result_text).font(
                                        egui::FontId {
                                            size: font_size,
                                            family: textedit_font_id.family,
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
                        make_button(self, ui, "log(");
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
        egui::Button::new(egui::RichText::new(operation).font(get_button_font(&calcuuube_gui.settings))),
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
            "=" => {
                calcuuube_gui.input_text = calcuuube_gui.result_text.clone();
                calcuuube_gui.input_text_cursor_position = calcuuube_gui.input_text.len();
            }
            _ => {
                let operation_chars: Chars = match operation {
                    "x²" => "^2".chars(),
                    "log(" => "log₁₀(".chars(),
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

        set_textedit_cursor_position(ui, calcuuube_gui);

        calculate_result(calcuuube_gui);
    }
}

fn set_textedit_cursor_position(ui: &mut egui::Ui, calcuuube_gui: &mut CalcuuubeGui) {
    let text_edit_id = "calcuuube_textedit".into();
    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
        let ccursor = egui::text::CCursor::new(calcuuube_gui.input_text_cursor_position);
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
        state.store(ui.ctx(), text_edit_id);
        ui.ctx().memory_mut(|mem| mem.request_focus(text_edit_id));
    }
}

fn calculate_result(calcuuube_gui: &mut CalcuuubeGui) {
    let calculation = calcuuube_gui
        .parser_context
        .calculate_string_to_string(&calcuuube_gui.input_text);
    match calculation {
        Some(some) => {
            calcuuube_gui.result_text = some;
            calcuuube_gui.calculation_error = false;
        }
        None => calcuuube_gui.calculation_error = true,
    }
}

fn find_fit_text(
    ui: &mut egui::Ui,
    input_string: &str,
    font_family: egui::FontFamily,
    max_font_size: f32,
    target_width: f32,
) -> f32 {
    for i in (5..max_font_size as i64).rev() {
        let font_id = egui::FontId {
            size: i as f32,
            family: font_family.clone(),
        };
        let mut total_width = 0.0;
        for char in (input_string.to_owned() + "  ").chars() {
            total_width += ui.fonts_mut(|f| f.glyph_width(&font_id, char));
        }
        if total_width <= target_width {
            return i as f32;
        }
    }
    return 1.0;
}

fn calculation_error(ui: &mut egui::Ui) {
    ui.add(
        egui::Label::new(
            egui::RichText::new("Calculation Error")
                .small()
                .color(ui.visuals().error_fg_color),
        )
        .selectable(false),
    );
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
    egui::widgets::global_theme_preference_buttons(ui);
    calcuuube_gui.settings.theme_preference = ui.options(|opt| opt.theme_preference);
    ui.ctx().set_theme(calcuuube_gui.settings.theme_preference)
}

fn capture_events(calcuuube_gui: &mut CalcuuubeGui, ui: &mut egui::Ui) {
    let mut reset_cursor_position = false;

    ui.input_mut(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(_) => {}
                egui::Event::Key {
                    key,
                    physical_key: _,
                    pressed,
                    repeat: _,
                    modifiers: _,
                } => {
                    if *pressed && key == &egui::Key::from_name("Enter").unwrap() {
                        calcuuube_gui.input_text = calcuuube_gui.result_text.clone();
                        calcuuube_gui.input_text_cursor_position = calcuuube_gui.input_text.len();
                        reset_cursor_position = true;
                    }
                }
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

    if reset_cursor_position {
        set_textedit_cursor_position(ui, calcuuube_gui);
    }
}
