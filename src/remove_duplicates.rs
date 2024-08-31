#![allow(unused)]
#![allow(unused_imports)]

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut current_index: usize = 0;
    let mut comparing_index: usize = 1;

    while comparing_index < nums.len() {
        if nums.get(current_index).unwrap().to_owned() != nums.get(comparing_index).unwrap().to_owned() {
            nums.insert(current_index+1, nums.get(comparing_index).unwrap().to_owned());
            current_index += 1;
        }
        comparing_index +=1;
    }
    current_index as i32 + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let mut nums: Vec<i32> = vec![1,1,2];
        let expected_nums: Vec<i32> = vec![1,2];
        assert_eq!(2i32,remove_duplicates(&mut nums));
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }

    #[test]
    fn second_test() {
        let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
        let expected_nums: Vec<i32> = vec![0,1,2,3,4];
        assert_eq!(5i32,remove_duplicates(&mut nums));
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }


    #[test]
    fn third_test() {
        let mut nums: Vec<i32> = vec![0,0,0,0,1,2,3,3,3,4,4];
        let expected_nums: Vec<i32> = vec![0,1,2,3,4];
        assert_eq!(5i32,remove_duplicates(&mut nums));
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }

    #[test]
    fn fourth_test() {
        let mut nums: Vec<i32> = vec![0];
        let expected_nums: Vec<i32> = vec![0];
        assert_eq!(1,remove_duplicates(&mut nums));
        for (first_num,second_num) in nums.iter().zip(&expected_nums) {
            assert_eq!(first_num,second_num);
        }
    }
}