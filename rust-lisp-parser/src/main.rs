#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum LispValue {
    LispString(String),
    LispInt(u8),
    LispList(Vec<LispValue>),
}

fn convert_token(input: String) -> LispValue {
    if let Ok(n) = input.parse::<u8>() {
        LispValue::LispInt(n)
    } else {
        LispValue::LispString(input)
    }
}

fn parse(input: &str) -> Vec<LispValue> {
    let mut stack: Vec<Vec<LispValue>> = Vec::new();
    let mut current_string = String::new();

    for token in input.chars() {
        if token == '(' {
            stack.push(Vec::new());
        } else if token == ' ' {
            if !current_string.is_empty() {
                stack
                    .last_mut()
                    .unwrap()
                    .push(convert_token(current_string.clone()));
                current_string.clear();
            }
        } else if token != '(' && token != ')' && token != ' ' {
            current_string.push(token);
        } else if token == ')' {
            if !current_string.is_empty() {
                stack
                    .last_mut()
                    .unwrap()
                    .push(convert_token(current_string.clone()));
                current_string.clear();
            }
            let completed = stack.pop().unwrap();
            if !stack.is_empty() {
                stack
                    .last_mut()
                    .unwrap()
                    .push(LispValue::LispList(completed));
            } else {
                return completed;
            }
        }
    }

    // TODO: return the completed AST
    Vec::new()
}

fn main() {
    let result = parse("(first (list 1 (+ 2 3) 9))");
    println!("{:?}", result);
}
