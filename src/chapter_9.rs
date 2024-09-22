// Lint a line of code, ensuring every opening brace has a closing brace.
fn is_opening_brace(brace: char) -> bool {
    ['[', '{', '('].contains(&brace)
}

fn is_closing_brace(brace: char) -> bool {
    [']', '}', ')'].contains(&brace)
}

fn is_a_match(opening: char, closing: char) -> bool {
    match (opening, closing) {
        ('[', ']') | ('{', '}') | ('(', ')') => true,
        _ => false,
    }
}

fn lint(code: &str) -> Result<bool, String> {
    let mut store = Vec::new();
    for c in code.chars() {
        if is_opening_brace(c) {
            store.push(c);
            continue;
        }
        if is_closing_brace(c) {
            let open = match store.pop() {
                None => return Err(format!("No matching opening brace. `{}`", c)),
                Some(c) => c,
            };
            if !is_a_match(open, c) {
                return Err(format!("Mismatched opening brace. `{}`", c));
            }
        }
    }
    match store.is_empty() {
        true => Ok(true),
        false => Err(format!("No matching closing brace.")),
    }
}

fn rev_str(word: &str) -> String {
    let mut store = Vec::new();
    for c in word.chars() {
        store.push(c);
    }
    let mut word = String::new();
    for _ in 0..store.len() {
        word.push(store.pop().expect("Cannot be null"));
    }
    word
}

fn rev_str_idiomatic(word: &str) -> String {
    word.chars().rev().collect()
}
