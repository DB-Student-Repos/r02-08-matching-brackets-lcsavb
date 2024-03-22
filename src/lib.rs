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

// Using match --> I beleive this is the most elegant solution
pub fn brackets_are_balanced(string: &str) -> bool {

            let mut stack = Vec::new();
            for c in string.chars() {
                match c {
                    '(' | '[' | '{' => stack.push(c),

                    // using .unwrap() to get the value of stack.pop() and comparing
                    // directly do a string can be dangerous
                    // and can cause a panic if the value is None, so it is not recommended
                    ')' => if stack.pop() != Some('(') { return false; },
                    ']' => if stack.pop() != Some('[') { return false; },
                    '}' => if stack.pop() != Some('{') { return false; },
                    _ => (),
                }
            }
            stack.is_empty()

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


