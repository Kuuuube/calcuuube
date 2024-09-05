#[cfg(test)]
use crate::calculate::calculate_string_to_string;

#[test]
#[rustfmt::skip]
pub fn test_calculation_outputs() {
    let mut parser_context = kalk::parser::Context::new();
    assert!(calculate_string_to_string("", &mut parser_context).unwrap() == "".to_owned());
    assert!(calculate_string_to_string("5393000*500", &mut parser_context).unwrap() == "2696500000".to_owned());
    assert!(calculate_string_to_string("1/3", &mut parser_context).unwrap() == "0.3333333333333333".to_owned());
    assert!(calculate_string_to_string("0.1+0.2", &mut parser_context).unwrap() == "0.3".to_owned());
    assert!(calculate_string_to_string("√2", &mut parser_context).unwrap() == "1.4142135624".to_owned());
    assert!(calculate_string_to_string("10*1000000000000000", &mut parser_context).unwrap() == "10000000000000000".to_owned());
    assert!(calculate_string_to_string("10*1000000000000000000000000000000000000000000000000000", &mut parser_context).unwrap() == "10^52".to_owned());
    assert!(calculate_string_to_string("1234567890", &mut parser_context).unwrap() == "1234567890".to_owned());
    assert!(calculate_string_to_string("0.0000000000000000000000000000000000000000000001/5", &mut parser_context).unwrap() == "0.00000000000000000000000000000000000000000000002".to_owned());
    assert!(calculate_string_to_string("log₁₀(9)", &mut parser_context).unwrap() == "0.9542425094393249".to_owned());
    assert!(calculate_string_to_string("2.7", &mut parser_context).unwrap() == "2.7".to_owned());
    assert!(calculate_string_to_string("2.1", &mut parser_context).unwrap() == "2.1".to_owned());
    assert!(calculate_string_to_string("0.1+0.2", &mut parser_context).unwrap() == "0.3".to_owned());
}
