#[cfg(test)]
use crate::preprocessor::preprocessor;

#[test]
pub fn test_preprocessor_sqrts() {
    let kalk_context = crate::context::CalculatorContext::Kalker(kalk::parser::Context::new());
    let fend_context = crate::context::CalculatorContext::Fend(fend_core::Context::new());
    assert!(preprocessor("√", &fend_context) == "@plain_number √".to_owned());
    assert!(preprocessor("√", &kalk_context) == "√".to_owned());
    assert!(preprocessor("√2", &fend_context) == "@plain_number sqrt(2)".to_owned());
    assert!(preprocessor("√2", &kalk_context) == "√(2)".to_owned());
    assert!(preprocessor("√22", &fend_context) == "@plain_number sqrt(22)".to_owned());
    assert!(preprocessor("√22", &kalk_context) == "√(22)".to_owned());
    assert!(
        preprocessor("√123123123.456456", &fend_context)
            == "@plain_number sqrt(123123123.456456)".to_owned()
    );
    assert!(preprocessor("√123123123.456456", &kalk_context) == "√(123123123.456456)".to_owned());
    assert!(preprocessor("√2*2", &fend_context) == "@plain_number sqrt(2)*2".to_owned());
    assert!(preprocessor("√2*2", &kalk_context) == "√(2)*2".to_owned());
    assert!(preprocessor("√2+2", &fend_context) == "@plain_number sqrt(2)+2".to_owned());
    assert!(preprocessor("√2+2", &kalk_context) == "√(2)+2".to_owned());
    assert!(preprocessor("√2-2", &fend_context) == "@plain_number sqrt(2)-2".to_owned());
    assert!(preprocessor("√2-2", &kalk_context) == "√(2)-2".to_owned());
    assert!(preprocessor("√2/2", &fend_context) == "@plain_number sqrt(2)/2".to_owned());
    assert!(preprocessor("√2/2", &kalk_context) == "√(2)/2".to_owned());
    assert!(preprocessor("√2^2", &fend_context) == "@plain_number sqrt(2)^2".to_owned());
    assert!(preprocessor("√2^2", &kalk_context) == "√(2)^2".to_owned());
    assert!(preprocessor("√(2*2)", &fend_context) == "@plain_number sqrt(2*2)".to_owned());
    assert!(preprocessor("√(2*2)", &kalk_context) == "√(2*2)".to_owned());
    assert!(
        preprocessor("√2*2√2*2", &fend_context) == "@plain_number sqrt(2)*2sqrt(2)*2".to_owned()
    );
    assert!(preprocessor("√2*2√2*2", &kalk_context) == "√(2)*2√(2)*2".to_owned());
    assert!(preprocessor("√2√2*2", &fend_context) == "@plain_number sqrt(2)sqrt(2)*2".to_owned());
    assert!(preprocessor("√2√2*2", &kalk_context) == "√(2)√(2)*2".to_owned());
    assert!(preprocessor("√-1", &fend_context) == "@plain_number sqrt(-1)".to_owned());
    assert!(preprocessor("√-1", &kalk_context) == "√(-1)".to_owned());
    assert!(preprocessor("√-1*2", &fend_context) == "@plain_number sqrt(-1)*2".to_owned());
    assert!(preprocessor("√-1*2", &kalk_context) == "√(-1)*2".to_owned());
    assert!(
        preprocessor("√(√(√(√(2))))", &fend_context)
            == "@plain_number sqrt(sqrt(sqrt(sqrt(2))))".to_owned()
    );
    assert!(preprocessor("√(√(√(√(2))))", &kalk_context) == "√(√(√(√(2))))".to_owned());
}

#[test]
pub fn test_preprocessor_ending_parentheses() {
    let kalk_context = crate::context::CalculatorContext::Kalker(kalk::parser::Context::new());
    let fend_context = crate::context::CalculatorContext::Fend(fend_core::Context::new());
    assert!(preprocessor("(2*5", &fend_context) == "@plain_number (2*5)".to_owned());
    assert!(preprocessor("(2*5", &kalk_context) == "(2*5)".to_owned());
    assert!(preprocessor("((2*5", &fend_context) == "@plain_number ((2*5))".to_owned());
    assert!(preprocessor("((2*5", &kalk_context) == "((2*5))".to_owned());
    assert!(preprocessor("2*5)", &fend_context) == "@plain_number 2*5)".to_owned());
    assert!(preprocessor("2*5)", &kalk_context) == "2*5)".to_owned());
    assert!(preprocessor("((((", &fend_context) == "@plain_number (((())))".to_owned());
    assert!(preprocessor("((((", &kalk_context) == "(((())))".to_owned());
    assert!(preprocessor("log(5", &fend_context) == "@plain_number log(5)".to_owned());
    assert!(preprocessor("log(5", &kalk_context) == "log(5)".to_owned());
    assert!(preprocessor("√(2", &fend_context) == "@plain_number sqrt(2)".to_owned());
    assert!(preprocessor("√(2", &kalk_context) == "√(2)".to_owned());
    assert!(preprocessor("sqrt(2", &fend_context) == "@plain_number sqrt(2)".to_owned());
    assert!(preprocessor("sqrt(2", &kalk_context) == "sqrt(2)".to_owned());
}

#[test]
pub fn test_preprocessor_remove_commas() {
    let kalk_context = crate::context::CalculatorContext::Kalker(kalk::parser::Context::new());
    let fend_context = crate::context::CalculatorContext::Fend(fend_core::Context::new());
    assert!(preprocessor("100,000", &fend_context) == "@plain_number 100000".to_owned());
    assert!(preprocessor("100,000", &kalk_context) == "100000".to_owned());
    assert!(preprocessor("10,000", &fend_context) == "@plain_number 10000".to_owned());
    assert!(preprocessor("10,000", &kalk_context) == "10000".to_owned());
    assert!(preprocessor(",", &fend_context) == "".to_owned());
    assert!(preprocessor(",", &kalk_context) == "".to_owned());
}
