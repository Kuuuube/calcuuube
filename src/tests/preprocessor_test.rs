#[cfg(test)]
use crate::preprocessor::preprocessor;

#[test]
pub fn test_preprocessor() {
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
}
