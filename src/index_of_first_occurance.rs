#![allow(unused)]
#![allow(unused_imports)]

use std::str::Bytes;


pub fn str_str(haystack: String, needle: String) -> i32 {
    // haystack: sadbutsad      needle: sad
    let mut is_present: i32 = 0;
    let mut where_present: i32 = 0;
    let mut haystack_iter: Bytes<'_>  = haystack.bytes();
    let haystack_vector: Vec<u8> = Vec::from_iter(haystack.bytes());
    let needle_vector: Vec<u8> = Vec::from_iter(needle.bytes());

    // println!("haystack numbers");
    println!("{:?}",haystack_vector);

    // println!("needle nubmers");
    println!("{:?}",needle_vector);
    0

}