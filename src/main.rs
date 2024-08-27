#![allow(unused_imports)]

mod merge_two_sorted_lists;
use merge_two_sorted_lists::make_list;

fn main() {
    let some_node = make_list(vec![1,2,4]);
    println!("{:?}",some_node.as_ref().unwrap().val);
    println!("{:?}",some_node.as_ref().unwrap().next.as_ref().unwrap().val);
    println!("{:?}",some_node.unwrap().next.unwrap().next.unwrap().val);
}

#[cfg(test)]
mod tests {
    use merge_two_sorted_lists::{make_list, merge_two_sorted_lists, ListNode};

    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(
            make_list(vec![1,1,2,3,4,4]),
            merge_two_sorted_lists(make_list(vec![1,2,4]),make_list(vec![1,3,4]))
        );
    }

    #[test]
    fn second_test() {
        assert_eq!(
            make_list(vec![]),
            merge_two_sorted_lists(make_list(vec![]),make_list(vec![]))
        );
    }

    #[test]
    fn third_test() {
        assert_eq!(
            make_list(vec![0]),
            merge_two_sorted_lists(make_list(vec![]),make_list(vec![0]))
        );
    }

    #[test]
    fn fourth_test() {
        assert_eq!(
            make_list(vec![1,2]),
            merge_two_sorted_lists(make_list(vec![2]),make_list(vec![1]))
        );
    }
}