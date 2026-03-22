#[cfg(test)]
use crate::realtimeprocessor::realtimeprocess;

#[test]
pub fn test_realtimeprocessor_log_subscript() {
    assert!(realtimeprocess("log10") == "log₁₀".to_owned());
    assert!(realtimeprocess("log2") == "log₂".to_owned());
    assert!(realtimeprocess("log10(10)") == "log₁₀(10)".to_owned());
    assert!(realtimeprocess("log2(10)") == "log₂(10)".to_owned());
    assert!(realtimeprocess("log2(") == "log₂(".to_owned());
    assert!(realtimeprocess("log123") == "log₁₂₃".to_owned());
    assert!(realtimeprocess("log10(1") == "log₁₀(1".to_owned());
    assert!(realtimeprocess("log2(1") == "log₂(1".to_owned());
    assert!(realtimeprocess("log(10") == "log(10".to_owned());
    assert!(realtimeprocess("log(2") == "log(2".to_owned());
    assert!(realtimeprocess("log(10)") == "log(10)".to_owned());
    assert!(realtimeprocess("log(2)") == "log(2)".to_owned());
    assert!(realtimeprocess("loge") == "ln".to_owned());
    assert!(realtimeprocess("logₑ") == "ln".to_owned());
    assert!(realtimeprocess("loge(") == "ln(".to_owned());
    assert!(realtimeprocess("logₑ(") == "ln(".to_owned());
}

#[test]
pub fn test_realtimeprocessor_remove_currency_symbols() {
    assert!(realtimeprocess("$1000") == "1000".to_owned());
    assert!(realtimeprocess("$1000+$12") == "1000+12".to_owned());
    assert!(realtimeprocess("£££££££££") == "".to_owned());
    assert!(realtimeprocess("123฿₵¢₡$₫֏$€ƒ₲₴₭₾₺₼₦₱£⃁₽₹₨₪⃀₸₮₩￦¥￥¤456") == "123456".to_owned());
    assert!(realtimeprocess("฿₵¢₡$₫֏$€ƒ₲₴₭₾₺₼ ฿₵¢₡$₫֏$€ƒ₲₴₭₾₺₼ ฿₵¢₡$₫֏$€ƒ₲₴₭₾₺₼") == "  ".to_owned());
}
