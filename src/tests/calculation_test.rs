#[cfg(test)]

#[test]
#[rustfmt::skip]
pub fn test_calculation_outputs() {
    let mut kalk_context = crate::context::CalculatorContext::Kalker(kalk::parser::Context::new());
    let mut fend_context = crate::context::CalculatorContext::Fend(fend_core::Context::new());

    assert!(fend_context.calculate_string_to_string("").unwrap() == "".to_owned());
    assert!(kalk_context.calculate_string_to_string("").unwrap() == "".to_owned());
    assert!(fend_context.calculate_string_to_string("5393000*500").unwrap() == "2696500000".to_owned());
    assert!(kalk_context.calculate_string_to_string("5393000*500").unwrap() == "2696500000".to_owned());
    assert!(fend_context.calculate_string_to_string("1/3").unwrap() == "0.3333333333".to_owned());
    assert!(kalk_context.calculate_string_to_string("1/3").unwrap() == "0.3333333333333333".to_owned());
    assert!(fend_context.calculate_string_to_string("0.1+0.2").unwrap() == "0.3".to_owned());
    assert!(kalk_context.calculate_string_to_string("0.1+0.2").unwrap() == "0.3".to_owned());
    assert!(fend_context.calculate_string_to_string("√2").unwrap() == "1.4142135624".to_owned());
    assert!(kalk_context.calculate_string_to_string("√2").unwrap() == "1.4142135624".to_owned());
    assert!(fend_context.calculate_string_to_string("√") == None);
    assert!(kalk_context.calculate_string_to_string("√") == None);
    assert!(fend_context.calculate_string_to_string("10*1000000000000000").unwrap() == "10000000000000000".to_owned());
    assert!(kalk_context.calculate_string_to_string("10*1000000000000000").unwrap() == "10000000000000000".to_owned());
    assert!(fend_context.calculate_string_to_string("10*1000000000000000000000000000000000000000000000000000").unwrap() == "10000000000000000000000000000000000000000000000000000".to_owned());
    assert!(kalk_context.calculate_string_to_string("10*1000000000000000000000000000000000000000000000000000").unwrap() == "10^52".to_owned());
    assert!(fend_context.calculate_string_to_string("1234567890").unwrap() == "1234567890".to_owned());
    assert!(kalk_context.calculate_string_to_string("1234567890").unwrap() == "1234567890".to_owned());
    assert!(fend_context.calculate_string_to_string("0.0000000000000000000000000000000000000000000001/5").unwrap() == "0.00000000000000000000000000000000000000000000002".to_owned());
    assert!(kalk_context.calculate_string_to_string("0.0000000000000000000000000000000000000000000001/5").unwrap() == "0.00000000000000000000000000000000000000000000002".to_owned());
    assert!(fend_context.calculate_string_to_string("log₁₀(9)").unwrap() == "0.9542425094".to_owned());
    assert!(kalk_context.calculate_string_to_string("log₁₀(9)").unwrap() == "0.9542425094393249".to_owned());
    assert!(fend_context.calculate_string_to_string("2.7").unwrap() == "2.7".to_owned());
    assert!(kalk_context.calculate_string_to_string("2.7").unwrap() == "2.7".to_owned());
    assert!(fend_context.calculate_string_to_string("2.1").unwrap() == "2.1".to_owned());
    assert!(kalk_context.calculate_string_to_string("2.1").unwrap() == "2.1".to_owned());
    assert!(fend_context.calculate_string_to_string("0.78125-0.7874016").unwrap() == "-0.0061516".to_owned());
    assert!(kalk_context.calculate_string_to_string("0.78125-0.7874016").unwrap() == "-0.0061516".to_owned());
    assert!(fend_context.calculate_string_to_string("log10(10)").unwrap() == "1".to_owned());
    assert!(kalk_context.calculate_string_to_string("log₁₀(10)").unwrap() == "1".to_owned());
    assert!(kalk_context.calculate_string_to_string("log₃(3)").unwrap() == "1".to_owned());
}
