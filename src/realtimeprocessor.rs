pub fn realtimeprocess(equation: &str) -> String {
    return loge_to_ln(log_subscript(remove_currency_markers(equation.to_string())));
}

fn log_subscript(equation: String) -> String {
    let equation_chars: Vec<char> = equation.chars().collect();
    let mut new_equation_chars: Vec<char> = Default::default();
    let mut found_log = false;
    let mut i = 0;

    while i < equation_chars.len() {
        let current_char = equation_chars.get(i).unwrap().to_owned();
        let next_char = equation_chars.get(i + 1).unwrap_or(&'\0').to_owned();
        let next_next_char = equation_chars.get(i + 2).unwrap_or(&'\0').to_owned();
        if current_char == 'l' && next_char == 'o' && next_next_char == 'g' {
            found_log = true;
            new_equation_chars.push(current_char);
            new_equation_chars.push(next_char);
            new_equation_chars.push(next_next_char);
            i += 2;
        } else if current_char == '(' {
            found_log = false;
            new_equation_chars.push(current_char);
        } else if found_log {
            new_equation_chars.push(char_to_subscript(current_char));
        } else {
            new_equation_chars.push(current_char);
        }
        i += 1;
    }
    return new_equation_chars.into_iter().collect();
}

fn char_to_subscript(char: char) -> char {
    return match char {
        '0' => '₀',
        '1' => '₁',
        '2' => '₂',
        '3' => '₃',
        '4' => '₄',
        '5' => '₅',
        '6' => '₆',
        '7' => '₇',
        '8' => '₈',
        '9' => '₉',
        _ => char,
    };
}

fn loge_to_ln(equation: String) -> String {
    return equation.replace("loge", "ln").replace("logₑ", "ln");
}

fn remove_currency_markers(equation: String) -> String {
    // https://en.wikipedia.org/wiki/Currency_symbol#List_of_currency_symbols_currently_in_use
    let currency_markers = [
        "\u{060B}", // AFGHANI SIGN
        "\u{0E3F}", // THAI CURRENCY SYMBOL BAHT
        "\u{20B5}", // CEDI SIGN
        "\u{00A2}", // CENT SIGN
        "\u{20A1}", // COLON SIGN
        "\u{0024}", // DOLLAR SIGN
        "\u{20AB}", // DONG SIGN
        "\u{058F}", // ARMENIAN DRAM SIGN
        "\u{0024}", // DOLLAR SIGN with a suitable font
        "\u{20AC}", // EURO SIGN
        "\u{0192}", // LATIN SMALL LETTER F WITH HOOK
        "\u{20B2}", // GUARANI SIGN
        "\u{20B4}", // HRYVNIA SIGN
        "\u{20AD}", // KIP SIGN
        "\u{20BE}", // LARI SIGN
        "\u{20BA}", // TURKISH LIRA SIGN
        "\u{20BC}", // MANAT SIGN
        "\u{20A6}", // NAIRA SIGN
        "\u{20B1}", // PESO SIGN
        "\u{00A3}", // POUND SIGN
        "\u{FDFC}", // RIAL SIGN
        "\u{20C1}", // SAUDI RIYAL SIGN
        "\u{17DB}", // KHMER CURRENCY SYMBOL RIEL
        "\u{20BD}", // RUBLE SIGN
        "\u{20B9}", // INDIAN RUPEE SIGN
        "\u{20A8}", // RUPEE SIGN
        "\u{20AA}", // NEW SHEQEL SIGN
        "\u{20C0}", // SOM SIGN
        "\u{09F3}", // BENGALI RUPEE SIGN
        "\u{20B8}", // TENGE SIGN
        "\u{20AE}", // TUGRIK SIGN
        "\u{20A9}", // WON SIGN
        "\u{FFE6}", // FULLWIDTH WON SIGN
        "\u{00A5}", // YEN SIGN
        "\u{FFE5}", // FULLWIDTH YEN SIGN
        "\u{00A4}", // CURRENCY SIGN
    ];
    let mut new_equation = equation;
    for currency_marker in currency_markers {
        new_equation = new_equation.replace(currency_marker, "");
    }
    return new_equation;
}
