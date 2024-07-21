#[cfg(test)]
use crate::preprocessor::preprocessor;

#[test]
pub fn test_preprocessor() {
    assert!(preprocessor("√2") == "math::sqrt(2)".to_owned());
    assert!(preprocessor("√22") == "math::sqrt(22)".to_owned());
    assert!(preprocessor("√123123123.456456") == "math::sqrt(123123123.456456)".to_owned());
    assert!(preprocessor("√2*2") == "math::sqrt(2)*2".to_owned());
    assert!(preprocessor("√2+2") == "math::sqrt(2)+2".to_owned());
    assert!(preprocessor("√2-2") == "math::sqrt(2)-2".to_owned());
    assert!(preprocessor("√2/2") == "math::sqrt(2)/2".to_owned());
    assert!(preprocessor("√2^2") == "math::sqrt(2)^2".to_owned());
    assert!(preprocessor("√(2*2)") == "math::sqrt(2*2)".to_owned());
    assert!(preprocessor("√2*2√2*2") == "math::sqrt(2)*2math::sqrt(2)*2".to_owned());
    assert!(preprocessor("√2√2*2") == "math::sqrt(2)math::sqrt2*2".to_owned());
}
