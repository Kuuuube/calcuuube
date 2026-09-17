use crate::preprocessor;

pub enum CalculatorContext {
    Kalker(kalk::parser::Context),
    Fend(fend_core::Context),
}

impl CalculatorContext {
    pub fn calculate_string_to_string(&mut self, input_string: &str) -> Option<String> {
        if input_string.len() == 0 {
            return Some("".to_string());
        }

        let preprocessed_string = preprocessor::preprocessor(input_string, &self);
        match self {
            CalculatorContext::Kalker(kalker_context) => {
                match kalk::parser::eval(kalker_context, &preprocessed_string, 1024) {
                    Ok(ok) => return Some(ok?.to_string_clean().replace(" ", "")),
                    Err(_) => return None,
                }
            }
            CalculatorContext::Fend(fend_context) => {
                if input_string.len() == 0 {
                    return Some("".to_string());
                }
                match fend_core::evaluate(&preprocessed_string, fend_context) {
                    Ok(ok) => return Some(ok.get_main_result().to_string()),
                    Err(_) => return None,
                }
            }
        }
    }
}
