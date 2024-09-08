#![allow(unused)]
#![allow(unused_imports)]

/*

    M I S S I S S I P P I
    ISSIS
    ISSIP
    ISSSSIP
    ISSSSIS
    MISSSSISSSSP

*/

use std::collections::HashMap;

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.bytes().len() == 0 {
        return 0;
    }

    if needle.bytes().len() > haystack.bytes().len() {
        return -1;
    }


    let mut last_occurance: HashMap<u8,isize> = HashMap::new();
    for (last_occurred, needle_byte) in needle.bytes().enumerate() {
        last_occurance.insert(needle_byte, last_occurred as isize);
    }
    let haystack_vector: Vec<u8> = Vec::from_iter(haystack.bytes());
    let needle_vector: Vec<u8> = Vec::from_iter(needle.bytes());
    let mut i: isize = needle_vector.len() as isize - 1;
    let mut j: isize = needle_vector.len() as isize - 1;
    while (i >= 0) & (j < haystack_vector.len() as isize) {
        if needle_vector[i as usize] == haystack_vector[j as usize] {
            i -= 1;
            j -= 1;
        } else {
            match last_occurance.get(&haystack_vector[j as usize]) {
                Some(last_occurance_of_mismatched_char) => {
                    j += (i - last_occurance_of_mismatched_char);
                    
                },
                None => {
                    j += needle_vector.len() as isize;
                }
            };
            i = needle_vector.len() as isize - 1;
        }
    }
    if i == 0 {
        return j as i32 - needle_vector.len() as i32 + 1;
    } else {
        return -1;
    }
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(0,str_str("sadbutsad".to_string(), "sad".to_string()));
    }


    #[test]
    fn second_test() {
        assert_eq!(-1,str_str("leetcode".to_string(), "leeto".to_string()));
    }


    #[test]
    fn third_test() {
        assert_eq!(4,str_str("mississippi".to_string(), "issip".to_string()));
    }


}