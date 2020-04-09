pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' | '}' | ')' => {
                if !stack.pop().eq(&Some(matching(c))) {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn matching(c: char) -> char {
    match c {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        _ => c,
    }
}
