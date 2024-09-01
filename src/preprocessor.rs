pub fn preprocessor(equation: &str) -> String {
    return remove_commas(inject_ending_parentheses(inject_sqrt_parentheses(equation.to_string())));
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
            if operators.contains(&next_char) {
                new_equation_chars.push(next_char);
                i += 1;
            }
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

fn inject_ending_parentheses(equation: String) -> String {
    let mut equation_chars: Vec<char> = equation.chars().collect();
    let mut parentheses = 0;
    for equation_char in &equation_chars {
        if equation_char == &'(' {
            parentheses += 1;
        } else if equation_char == &')' {
            parentheses -= 1;
        }
    }
    for _ in 0..parentheses {
        equation_chars.push(')');
    }

    return equation_chars.into_iter().collect();
}

fn remove_commas(equation: String) -> String {
    return equation.replace(",", "");
}
