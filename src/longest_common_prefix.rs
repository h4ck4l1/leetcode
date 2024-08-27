#![allow(unused)]

pub fn longest_common_prefix(strs: Vec<String>) -> String {

    if strs.len() == 1{
        return strs[0].clone();
    }
    for s in &strs {
        if s.len() == 0 {
            return s.clone();
        }
    }
    let mut common_prefex = String::from("");
    let mut char_index = 0usize;
    let mut word_index = 0usize;
    let max_word_index = strs.len() - 2;
    loop {
        let current_word = &strs[word_index];
        let next_word = &strs[word_index + 1];
        if (current_word.len() - 1 < char_index) | (next_word.len() - 1 < char_index) {
            return common_prefex;
        }
        if &current_word[char_index..char_index+1] != &next_word[char_index..char_index+1] {
            return common_prefex;
        }
        word_index += 1;
        if word_index > max_word_index {
            common_prefex.push_str(&current_word[char_index..char_index+1]);
            word_index = 0;
            char_index += 1
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(
            String::from("fl"),
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ])
        );
    }

    #[test]
    fn second_test() {
        assert_eq!(
            String::from(""),
            longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ])
        );
    }

    #[test]
    fn third_test() {
        assert_eq!(
            String::from("abc"),
            longest_common_prefix(vec![
                String::from("abcde"),
                String::from("abcdf"),
                String::from("abcdg"),
                String::from("abcdh"),
                String::from("abcii")
            ])
        );
    }


    #[test]
    fn fourth_test() {
        assert_eq!(
            String::from("a"),
            longest_common_prefix(vec![
                String::from("abcde"),
                String::from("abcd"),
                String::from("abc"),
                String::from("ab"),
                String::from("a")
            ])
        );
    }


    #[test]
    fn fifth_test() {
        assert_eq!(
            String::from("ab"),
            longest_common_prefix(vec![
                String::from("abab"),
                String::from("aba"),
                String::from("abc")
            ])
        );
    }


    #[test]
    fn sixth_test() {
        assert_eq!(
            String::from("a"),
            longest_common_prefix(vec![
                String::from("ab"),
                String::from("a")
            ])
        );
    }


    #[test]
    fn seventh_test() {
        assert_eq!(
            String::from("aa"),
            longest_common_prefix(vec![
                String::from("aaa"),
                String::from("aa"),
                String::from("aaa")
            ])
        );
    }

    #[test]
    fn eight_test() {
        assert_eq!(
            String::from(""),
            longest_common_prefix(vec![
                String::from(""),
                String::from("")
            ])
        );
    }

}