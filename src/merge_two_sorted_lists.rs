#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_imports)]



pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    } 
}


impl ListNode {
    fn new_from_list(list: &Vec<i32>) -> Self {
        ListNode::new(list.get(0).unwrap().to_owned())
    }

    fn append_to_list(&mut self, list: &Vec<i32>, index: usize) {
        self.next = Some(Box::new(ListNode::new(list.get(index).unwrap().to_owned())))
    }
}

pub fn make_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    
}