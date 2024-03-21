use regex::Regex;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut haystack = String::new();
    for char in string.chars() {
        if let '[' | ']' | '{' | '}' | '(' | ')' = char {
            haystack.push(char);
        }
    }
    
    let re = Regex::new(r"\(\)|\[\]|\{\}").unwrap();
    while re.is_match(&haystack) {
        haystack = re.replace_all(&haystack, "").to_string();
    }

    haystack.is_empty()
}

