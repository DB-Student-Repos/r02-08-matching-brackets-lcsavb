// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut haystack = String::new();
//     for char in string.chars() {
//         if let '[' | '{' | '(' = char {
//             haystack.push(char);
//         }

//         if let ']' | '}' | ')' = char {
//             if haystack.is_empty() {
//                 return false;
//             }

//             let last = haystack.pop().unwrap();
//             if (last != '[' && char == ']') 
//             || (last != '{' && char == '}') 
//             || (last != '(' && char == ')') 
//             {
//                 return false;
//             }
//         }
//     }
    
//     haystack.is_empty()
// }

// Using recursion
pub fn brackets_are_balanced(string: &str) -> bool {
    fn is_matching_pair(open: char, close: char) -> bool {
        (open == '(' && close == ')') || (open == '[' && close == ']') || (open == '{' && close == '}')
    }

    fn helper(stack: &mut Vec<char>, chars: &[char]) -> bool {
        if chars.is_empty() {
            return stack.is_empty();
        }

        let first = chars[0];
        let rest = &chars[1..];

        match first {
            '[' | '{' | '(' => {
                stack.push(first);
                helper(stack, rest)
            },
            ']' | '}' | ')' => {
                if stack.is_empty() || !is_matching_pair(stack.pop().unwrap(), first) {
                    false
                } else {
                    helper(stack, rest)
                }
            },
            _ => helper(stack, rest),
        }
    }

    let chars: Vec<char> = string.chars().collect();
    helper(&mut Vec::new(), &chars)
}


// Using Regex
// use regex::Regex;

// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut haystack = String::new();
//     for char in string.chars() {
//         if let '[' | ']' | '{' | '}' | '(' | ')' = char {
//             haystack.push(char);
//         }
//     }
    
//     let re = Regex::new(r"\(\)|\[\]|\{\}").unwrap();
//     while re.is_match(&haystack) {
//         haystack = re.replace_all(&haystack, "").to_string();
//     }

//     haystack.is_empty()
// }


