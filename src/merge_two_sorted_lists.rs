#![allow(unused)]
#![allow(unused_imports)]

use std::result;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}


impl ListNode {
    
    #[inline]
    fn new(val:i32) -> Self {
        ListNode {val,next:None}
    }
}

pub fn make_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    if list.len() == 0 {
        return None;
    }
    if list.len() == 1 {
        return Some(Box::from(ListNode { val: list.get(0).unwrap().to_owned() , next: None}));
    }
    let mut result_node: Box<ListNode> = Box::new(ListNode { val: list.get(0).unwrap().to_owned(), next: None });
    let mut index:usize = 0;
    let mut itr = &mut result_node;
    for &val in &list[1..] {
        itr.next = Some(Box::new(ListNode { val, next:None }));
        itr = itr.next.as_mut().unwrap();
    }
    Some(result_node)
}


pub fn merge_two_sorted_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if 
    Some(Box::new(ListNode{val:0,next:None}))
}