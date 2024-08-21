#![allow(unused_imports)]
mod roman_to_integer;
use roman_to_integer::roman_to_int;

fn main() {
    println!("{}",roman_to_int("MCMXCIV".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(3,roman_to_int(String::from("III")));
    }

    #[test]
    fn second_test() {
        assert_eq!(58,roman_to_int(String::from("LVIII")));
    }

    #[test]
    fn third_test() {
        assert_eq!(1994,roman_to_int(String::from("MCMXCIV")));
    }
}