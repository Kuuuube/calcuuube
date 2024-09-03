use crate::preprocessor;

pub fn calculate_string_to_string(
    input_string: &str,
    parser_context: &mut kalk::parser::Context,
) -> Option<String> {
    if input_string.len() == 0 {
        return Some("".to_string());
    }

    match kalk::parser::eval(
        parser_context,
        &preprocessor::preprocessor(input_string),
        100,
    ) {
        Ok(ok) => Some(ok?.to_string_clean().replace(" ", "")),
        Err(_) => return None,
    }
}
