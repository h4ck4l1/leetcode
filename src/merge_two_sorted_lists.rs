#![allow(unused)]
#![allow(unused_imports)]

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ListNode {
    val:i32,
    next: Option<Box<ListNode>>
}

impl ListNode {

    #[inline]
    pub fn new(val:i32) -> Self {
        ListNode {
            val,
            next:None
        }
    }

    pub fn make_list(list: &Vec<i32>) -> Option<Box<ListNode>> {
        if list.len() == 0 {
            return None;
        }
        if list.len() == 1 {
            return Some(Box::new(ListNode {val:list.get(0).unwrap().to_owned(),next:None}));
        }
        let mut list_node: ListNode = ListNode {val:list.get(0).unwrap().to_owned(),next:None};
        let mut current: &mut ListNode = &mut list_node;
        for num in &list[1..] {
            current.next = Some(Box::new(ListNode {val:num.to_owned(),next:None}));
            current = current.next.as_mut().unwrap();
        }
        Some(Box::from(list_node))
    }
}


pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    

    match &list1 {
        Some(_) => (),
        None => return list2,
    }

    match &list2 {
        Some(_) => (),
        None => return list1,
    }

    let mut first_list_done = false;
    let mut second_list_done= false;
    let mut current_first_list: Option<Box<ListNode>> = list1;
    let mut current_second_list: Option<Box<ListNode>> = list2;
    let mut new_list_node: Box<ListNode> = Box::new(ListNode::new(0));
    let mut current_new_list: &mut Box<ListNode> = &mut new_list_node;

    while (!first_list_done) | (!second_list_done) {
        println!("first: {} second: {}",current_first_list.as_ref().unwrap().val,current_second_list.as_ref().unwrap().val);
        if current_first_list.as_ref().unwrap().val > current_second_list.as_ref().unwrap().val {
            current_new_list.next = current_second_list.clone();
            current_second_list = current_second_list.unwrap().next;
        } else {
            current_new_list.next = current_first_list.clone();
            current_first_list = current_first_list.unwrap().next;
        }
        current_new_list = current_new_list.next.as_mut().unwrap();
        match current_first_list {
            Some(_) => (),
            None => {
                first_list_done = true;
                current_first_list = Some(Box::new(ListNode::new(100)));
            }
        }
        match current_second_list {
            Some(_) => (),
            None => {
                second_list_done = true;
                current_second_list = Some(Box::new(ListNode::new(100)));
            }
        }
    }
    new_list_node.next

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(
            ListNode::make_list(&vec![1,1,2,3,4,4]),
            merge_two_lists(ListNode::make_list(&vec![1,2,4]),ListNode::make_list(&vec![1,3,4]))
        );
    }

    #[test]
    fn second_test() {
        assert_eq!(
            ListNode::make_list(&vec![]),
            merge_two_lists(ListNode::make_list(&vec![]),ListNode::make_list(&vec![]))
        );
    }

    #[test]
    fn third_test() {
        assert_eq!(
            ListNode::make_list(&vec![0]),
            merge_two_lists(ListNode::make_list(&vec![]),ListNode::make_list(&vec![0]))
        );
    }

    #[test]
    fn fourth_test() {
        assert_eq!(
            ListNode::make_list(&vec![1,2]),
            merge_two_lists(ListNode::make_list(&vec![2]),ListNode::make_list(&vec![1]))
        );
    }
}