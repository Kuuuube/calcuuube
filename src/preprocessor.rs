pub fn preprocessor(equation: &str) -> String {
    return replace_sqrt_symbols(inject_sqrt_parentheses(equation.to_string()));
}

fn inject_sqrt_parentheses(equation: String) -> String {
    let equation_chars: Vec<char> = equation.chars().collect();
    let mut new_equation_chars: Vec<char> = Default::default();
    let mut found_sqrt = false;
    let mut i = 0;

    let operators = ['+', '-', '*', '/', '√', '^'];

    while i < equation_chars.len() {
        let current_char = equation_chars.get(i).unwrap().to_owned();
        let next_char = equation_chars.get(i + 1).unwrap_or(&'\0').to_owned();
        if next_char == '\0' {
            new_equation_chars.push(current_char);
            break;
        } else if current_char == '√' && next_char != '(' && !found_sqrt {
            new_equation_chars.push(current_char);
            new_equation_chars.push('(');
            found_sqrt = true;
        } else if found_sqrt && operators.contains(&current_char) {
            new_equation_chars.push(')');
            new_equation_chars.push(current_char);
            found_sqrt = false;
        } else {
            new_equation_chars.push(current_char);
        }
        i += 1;
    }
    if found_sqrt {
        new_equation_chars.push(')');
    }
    return new_equation_chars.into_iter().collect();
}

fn replace_sqrt_symbols(equation: String) -> String {
    return equation.replace("√", "math::sqrt");
}
