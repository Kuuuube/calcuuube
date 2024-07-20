#[cfg(test)]
use crate::preprocessor::preprocessor;

#[test]
pub fn test_preprocessor() {
    assert!(preprocessor("√2*2") == "√(2)*2".to_owned());
    assert!(preprocessor("√2+2") == "√(2)+2".to_owned());
    assert!(preprocessor("√2-2") == "√(2)-2".to_owned());
    assert!(preprocessor("√2/2") == "√(2)/2".to_owned());
    assert!(preprocessor("√2^2") == "√(2)^2".to_owned());
    assert!(preprocessor("√(2*2)") == "√(2*2)".to_owned());
    assert!(preprocessor("√2*2√2*2") == "√(2)*2√(2)*2".to_owned());
    assert!(preprocessor("√2√2*2") == "√(2)√2*2".to_owned());
}