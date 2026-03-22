#[cfg(test)]
use crate::preprocessor::preprocessor;

#[test]
pub fn test_preprocessor_sqrts() {
    assert!(preprocessor("√") == "@plain_number √".to_owned());
    assert!(preprocessor("√2") == "@plain_number sqrt(2)".to_owned());
    assert!(preprocessor("√22") == "@plain_number sqrt(22)".to_owned());
    assert!(preprocessor("√123123123.456456") == "@plain_number sqrt(123123123.456456)".to_owned());
    assert!(preprocessor("√2*2") == "@plain_number sqrt(2)*2".to_owned());
    assert!(preprocessor("√2+2") == "@plain_number sqrt(2)+2".to_owned());
    assert!(preprocessor("√2-2") == "@plain_number sqrt(2)-2".to_owned());
    assert!(preprocessor("√2/2") == "@plain_number sqrt(2)/2".to_owned());
    assert!(preprocessor("√2^2") == "@plain_number sqrt(2)^2".to_owned());
    assert!(preprocessor("√(2*2)") == "@plain_number sqrt(2*2)".to_owned());
    assert!(preprocessor("√2*2√2*2") == "@plain_number sqrt(2)*2sqrt(2)*2".to_owned());
    assert!(preprocessor("√2√2*2") == "@plain_number sqrt(2)sqrt(2)*2".to_owned());
    assert!(preprocessor("√-1") == "@plain_number sqrt(-1)".to_owned());
    assert!(preprocessor("√-1*2") == "@plain_number sqrt(-1)*2".to_owned());
    assert!(preprocessor("√(√(√(√(2))))") == "@plain_number sqrt(sqrt(sqrt(sqrt(2))))".to_owned());
}

#[test]
pub fn test_preprocessor_ending_parentheses() {
    assert!(preprocessor("(2*5") == "@plain_number (2*5)".to_owned());
    assert!(preprocessor("((2*5") == "@plain_number ((2*5))".to_owned());
    assert!(preprocessor("2*5)") == "@plain_number 2*5)".to_owned());
    assert!(preprocessor("((((") == "@plain_number (((())))".to_owned());
    assert!(preprocessor("log(5") == "@plain_number log(5)".to_owned());
    assert!(preprocessor("√(2") == "@plain_number sqrt(2)".to_owned());
    assert!(preprocessor("sqrt(2") == "@plain_number sqrt(2)".to_owned());
}

#[test]
pub fn test_preprocessor_remove_commas() {
    assert!(preprocessor("100,000") == "@plain_number 100000".to_owned());
    assert!(preprocessor("10,000") == "@plain_number 10000".to_owned());
    assert!(preprocessor(",") == "".to_owned());
}
