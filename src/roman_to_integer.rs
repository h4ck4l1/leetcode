#![allow(unused)]

use std::collections::HashMap;
use std::time::{Instant,Duration};

pub fn roman_to_int(s: String) -> i32 {
    let start = Instant::now();
    // let roman_dict: HashMap<&str,i32> = HashMap::from([
    //     ("Z",0),
    //     ("I",1),
    //     ("V",5),
    //     ("X",10),
    //     ("L",50),
    //     ("C",100),
    //     ("D",500),
    //     ("M",1000)
    // ]);
    let mut roman_dict: HashMap<&str,i32> = HashMap::new();
    roman_dict.insert("Z", 0);
    roman_dict.insert("I", 1);
    roman_dict.insert("V", 5);
    roman_dict.insert("X", 10);
    roman_dict.insert("L", 50);
    roman_dict.insert("C", 100);
    roman_dict.insert("D", 500);
    roman_dict.insert("M", 1000);
    let mut ind: usize = 0;
    let mut total: i32 = 0;
    let full_string: &str = &(s+"ZZ");
    while ind < full_string.len() - 2 {
        let current_char: &str = &full_string[ind..ind+1];
        let current_num: &i32 = roman_dict.get(&current_char).unwrap();
        let next_char: &str = &full_string[ind+1..ind+2];
        let next_num: &i32 = roman_dict.get(&next_char).unwrap();
        if current_num <  next_num {
            total += (next_num - current_num);
            ind += 1;
        } else {
            total += current_num;
        }
        ind += 1;
    }
    println!("total time duration: {:?}",start.elapsed());
    total
}