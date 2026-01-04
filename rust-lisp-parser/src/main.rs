enum LispValue {
    LispString(String),
    LispInt(u8),
    LispList(Vec<LispValue>),
}

fn parse(input: &str) -> Vec<LispValue> {
    let mut stack: Vec<Vec<LispValue>> = Vec::new();
    let mut current_string = String::new();

    for token in input.chars() {
        if token == '(' {
            stack.push(Vec::new());
        }
        // TODO: handle ' ', other characters, and ')'
    }

    // TODO: return the completed AST
    Vec::new()
}

fn main() {
    let result = parse("(first (list 1 (+ 2 3) 9))");
    println!("Hello, world!");
}
