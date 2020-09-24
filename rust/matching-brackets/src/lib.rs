const BRACKETS: &'static [char] = &['(', ')', '[', ']', '{', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for c in string.chars().filter(|c| BRACKETS.contains(c)).into_iter() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            close_bracket => match (stack.pop(), close_bracket) {
                (Some('('), ')') => {}
                (Some('['), ']') => {}
                (Some('{'), '}') => {}
                _ => return false,
            },
        }
    }
    stack.is_empty()
}
