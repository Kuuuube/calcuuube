use crate::preprocessor;

pub fn calculate_string_to_string(
    input_string: &str,
    parser_context: &mut kalk::parser::Context,
) -> Option<String> {
    if input_string.len() == 0 {
        return Some("".to_string());
    }
    if input_string.contains("i") {
        return None;
    }

    let result = kalk::parser::eval(
        parser_context,
        &preprocessor::preprocessor(input_string),
        53,
    );
    if result.is_err() {
        return None;
    }
    return Some(result.unwrap()?.to_string_clean().replace(" ", ","));
}
