#![allow(unused)]

use std::{collections::HashMap, iter::zip, vec};


/*
def two_sum(nums:list[int],target:int) -> int:
    temp_dict = dict()
    for count,number in enumerate(nums):
        print(f"current count: {count} and number: {number} and target-number: {target-number}")
        print(temp_dict.get(target-number,0))
        if target-number in temp_dict:
            return [temp_dict[target-number],count]
        else:
            temp_dict[number] = count

*/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut dict: HashMap<i32,i32> = HashMap::new();
    let mut target_vector: Vec<i32> = Vec::new();

    for (count,number) in nums.iter().enumerate() {
        let count: i32 = count as i32;
        match dict.get(&(target - number)) {
            Some(n) => {target_vector.push(*n);target_vector.push(count);},
            None => {dict.insert(*number, count);}
        }
    }
    target_vector
}