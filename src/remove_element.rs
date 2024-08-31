#![allow(unused)]
#![allow(unused_imports)]

use std::mem::replace;

pub fn remove_element(nums: &mut Vec<i32>,val: i32) -> i32 {

    let mut k: usize = 0;
    for i in (0..nums.len()) {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let mut nums: Vec<i32> = vec![3,2,2,3];
        let val: i32 = 3;
        let expected_nums: Vec<i32> = vec![2,2];
        assert_eq!(2i32,remove_element(&mut nums,val));
        println!("{:?}",nums);
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }

    #[test]
    fn second_test() {
        let mut nums: Vec<i32> = vec![0,1,2,2,3,0,4,2];
        let val: i32 = 2;
        let expected_nums: Vec<i32> = vec![4,0,3,1,0];
        assert_eq!(5i32,remove_element(&mut nums,val));
        println!("{:?}",nums);
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }

    #[test]
    fn thrid_test() {
        let mut nums: Vec<i32> = vec![3,2,2,3,4,4,4,6];
        let val: i32 = 4;
        let expected_nums: Vec<i32> = vec![6,3,2,2,3];
        assert_eq!(5i32,remove_element(&mut nums,val));
        println!("{:?}",nums);
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }
    
}