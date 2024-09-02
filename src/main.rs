#![allow(unused)]

mod index_of_first_occurance;
use index_of_first_occurance::str_str;

fn main() {

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(0,str_str("sadbutsad".to_string(), "sad".to_string()));
    }


    #[test]
    fn second_test() {
        assert_eq!(-1,str_str("leetcode".to_string(), "leeto".to_string()));
    }


    #[test]
    fn third_test() {
        assert_eq!(4,str_str("mississippi".to_string(), "issip".to_string()));
    }


}




