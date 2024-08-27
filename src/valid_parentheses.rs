#![allow(unused)]
#![allow(unused_imports)]

use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {

    if s.len().rem_euclid(2) != 0 {return false;}
    let mut vector: Vec<char> = Vec::new();
    for c in s.chars() {
        if (c == '(') | (c == '{') | (c == '[') {
            vector.push(c);
        } else {
            let popped = match vector.pop() {
                Some(n) => n,
                None => return false
            };
            if (c == ')') & (popped != '(') {return false;}
            if (c == '}') & (popped != '{') {return false;}
            if (c == ']') & (popped != '[') {return false;}
        }
    }
    if vector.len() == 0 {return true;} else {return false;}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(true,is_valid("()".to_string()));
    }

    #[test]
    fn second_test() {
        assert_eq!(true,is_valid("()[]{}".to_string()));
    }

    #[test]
    fn third_test() {
        assert_eq!(false,is_valid("(]".to_string()));
    }

    #[test]
    fn fourth_test() {
        assert_eq!(false,is_valid("([)]".to_string()));
    }

    #[test]
    fn fifth_test() {
        assert_eq!(false,is_valid("([([())])]".to_string()));
    }

    #[test]
    fn sixth_test() {
        assert_eq!(false,is_valid("[".to_string()));
    }

    #[test]
    fn seventh_test() {
        assert_eq!(false,is_valid("]".to_string()));
    }

}