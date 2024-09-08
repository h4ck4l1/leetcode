#![allow(unused)]

mod search_insert_position;
use search_insert_position::search_insert;

fn main() {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(2,search_insert(vec![1,3,5,6], 5));
    }

    #[test]
    fn second_test() {
        assert_eq!(2,search_insert(vec![1,3,5,6], 2));
    }

    #[test]
    fn third_test() {
        assert_eq!(4,search_insert(vec![1,3,5,6], 7));
    }
}





