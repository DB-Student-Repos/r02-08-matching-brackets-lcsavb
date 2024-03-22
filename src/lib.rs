pub fn brackets_are_balanced(string: &str) -> bool {
    let mut haystack = String::new();
    for char in string.chars() {
        if let '[' | '{' | '(' = char {
            haystack.push(char);
        }

        if let ']' | '}' | ')' = char {
            if haystack.is_empty() {
                return false;
            }

            let last = haystack.pop().unwrap();
            if (last != '[' && char == ']') 
            || (last != '{' && char == '}') 
            || (last != '(' && char == ')') 
            {
                return false;
            }
        }
    }
    
    haystack.is_empty()
}

// Using recursion
// pub fn brackets_are_balanced(string: &str) -> bool {
//     fn helper(stack: &mut Vec<char>, chars: &[char]) -> bool {
//         match chars.split_first() {
//             Some((first, rest)) => match first {
//                 '[' | '{' | '(' => {
//                     stack.push(*first);
//                     helper(stack, rest)
//                 }
//                 ']' | '}' | ')' => match stack.pop() {
//                     Some(last) if (last == '[' && *first != ']')
//                         || (last == '{' && *first != '}')
//                         || (last == '(' && *first != ')') => false,
//                     _ => helper(stack, rest),
//                 },
//                 _ => helper(stack, rest),
//             },
//             None => stack.is_empty(),
//         }
//     }

//     helper(&mut Vec::new(), &string.chars().collect::<Vec<char>>())
// }

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


