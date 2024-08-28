#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_imports)]


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {

    #[inline]
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

    #[inline]
    fn append_to_list(&mut self, list: &Vec<i32>, index: usize) {
        self.next = Some(Box::new(ListNode::new(list.get(index).unwrap().to_owned())))
    }
}

pub fn make_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    if list.len() == 0 {
        return None;
    }
    else if list.len() == 1 {
        return Some(Box::new(ListNode::new_from_list(&list)));
    }
    let mut result_node: ListNode = ListNode::new_from_list(&list);
    let mut itr: &mut ListNode = &mut result_node;
    for &i in &list[1..] {
        itr.append_to_list(&list,i as usize);
        itr = itr.next.as_mut().unwrap();
    }
    Some(Box::from(result_node))
}


pub fn merge_two_lists(list1: Option<Box<ListNode>>,list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(0)))
}