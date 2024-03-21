use regex::Regex;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut haystack = String::new();
    for char in string.chars() {
        match char {
            '[' | ']' | '{' | '}' | '(' | ')' => haystack.push(char),
            _ => {}
        }
    }
    
    let re = Regex::new(r"\(\)|\[\]|\{\}").unwrap();
    while re.is_match(&haystack) {
        haystack = re.replace_all(&haystack, "").to_string();
    }

    haystack.is_empty()
}

