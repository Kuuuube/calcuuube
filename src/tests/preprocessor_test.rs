#[cfg(test)]
use crate::preprocessor::preprocessor;

#[test]
pub fn test_preprocessor_sqrts() {
    assert!(preprocessor("√2") == "√(2)".to_owned());
    assert!(preprocessor("√22") == "√(22)".to_owned());
    assert!(preprocessor("√123123123.456456") == "√(123123123.456456)".to_owned());
    assert!(preprocessor("√2*2") == "√(2)*2".to_owned());
    assert!(preprocessor("√2+2") == "√(2)+2".to_owned());
    assert!(preprocessor("√2-2") == "√(2)-2".to_owned());
    assert!(preprocessor("√2/2") == "√(2)/2".to_owned());
    assert!(preprocessor("√2^2") == "√(2)^2".to_owned());
    assert!(preprocessor("√(2*2)") == "√(2*2)".to_owned());
    assert!(preprocessor("√2*2√2*2") == "√(2)*2√(2)*2".to_owned());
    assert!(preprocessor("√2√2*2") == "√(2)√2*2".to_owned());
    assert!(preprocessor("√-1") == "√(-1)".to_owned());
    assert!(preprocessor("√-1*2") == "√(-1)*2".to_owned());
}

#[test]
pub fn test_preprocessor_ending_parentheses() {
    assert!(preprocessor("(2*5") == "(2*5)".to_owned());
    assert!(preprocessor("((2*5") == "((2*5))".to_owned());
    assert!(preprocessor("2*5)") == "2*5)".to_owned());
    assert!(preprocessor("((((") == "(((())))".to_owned());
    assert!(preprocessor("log(5") == "log(5)".to_owned());
    assert!(preprocessor("√(2") == "√(2)".to_owned());
}

#[test]
pub fn test_preprocessor_remove_commas() {
    assert!(preprocessor("100,000") == "100000".to_owned());
    assert!(preprocessor("10,000") == "10000".to_owned());
    assert!(preprocessor(",") == "".to_owned());
}
