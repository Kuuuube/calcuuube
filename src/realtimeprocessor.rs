pub fn realtimeprocess(equation: &str) -> String {
    return loge_to_ln(log_subscript(equation.to_string()));
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
