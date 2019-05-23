enum Op {
    Square,
    Curly,
    Round,
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '[' => stack.push(Op::Square),
            ']' => match stack.pop() {
                Some(Op::Square) => continue,
                _ => return false,
            },

            '{' => stack.push(Op::Curly),
            '}' => {
                if let Some(Op::Curly) = stack.pop() {
                    continue;
                }
                return false;
            },

            '(' => stack.push(Op::Round),
            ')' => {
                if let Some(Op::Round) = stack.pop() {
                    continue;
                }
                return false;
            },

            _ => continue,
        }
    }
    stack.is_empty()
}
