pub fn brackets_are_balanced(string: &str) -> bool {
    let braces = [
        ('[', ']'),
        ('{', '}'),
        ('(', ')'),
    ];
    let mut stack = Vec::new();

    for c in string.chars() {
        for (open, close) in braces.iter() {
            if c == *open {
                stack.push(open);
            } else if c == *close && stack.pop() != Some(open) {
                return false;
            }
        }
    }
    stack.is_empty()
}
