use crate::preprocessor;

pub fn calculate_string_to_string(input_string: &str) -> Option<String> {
    if input_string.len() == 0 {
        return Some("".to_string());
    }

    match evalexpr::eval(&preprocessor::preprocessor(input_string)) {
        Ok(ok) => return Some(ok.to_string()),
        Err(_) => return None,
    };
}
