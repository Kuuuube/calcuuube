use crate::preprocessor;

pub fn calculate_string_to_string(
    input_string: &str,
    parser_context: &mut fend_core::Context,
) -> Option<String> {
    if input_string.len() == 0 {
        return Some("".to_string());
    }

    dbg!(&preprocessor::preprocessor(input_string));

    match fend_core::evaluate(
        &preprocessor::preprocessor(input_string),
        parser_context,
    ) {
        Ok(ok) => return Some(ok.get_main_result().to_string()),
        Err(_) => return None,
    }
}
