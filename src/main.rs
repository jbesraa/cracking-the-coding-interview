//Arrays and Strings

// Q1
// Implement an algorithm to determine if a string has all unique chars.

use itertools::Itertools;

#[allow(dead_code)]
fn is_all_chars_unique(s: &str) -> bool {
    let mut cpy: String = "".to_owned();
    for c in s.chars() {
        if cpy.contains(c) {
            return false;
        }
        cpy.push(c);
    }
    true
}

// Q2
// Given two strings, write a method to decide if one is a permutation of the other

#[allow(dead_code)]
fn is_premutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        false
    } else {
        let ordered_s1 = s1.chars().sorted().collect::<String>();
        let ordered_s2 = s2.chars().sorted().collect::<String>();

        ordered_s1 == ordered_s2
    }
}

// Q4
// Given a string, write a function to check if it is a premutation of a palindrome

#[allow(dead_code)]
fn is_palindrome(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>();

    s == reversed
}

// Q5
//  There are three types of edits that can be performed on strings:
// insert a character, remove a character, or replace a character.
// Given two strings, write a function to check if they are one edit(or zero edits) away.
// examples:
// pale, ple -> true || pales, pale -> true || pale, bale -> true || pale, bake -> false
#[allow(dead_code)]
fn one_away(s1: &str, s2: &str) -> bool {
    let mut distance: usize = 0;
    if s1 == s2 || s1.is_empty() && s2.is_empty() {
        distance = 0;
    } else if s1.is_empty() {
        distance = s2.len() - 1;
    } else if s2.is_empty() {
        distance = s1.len() - 1;
    } else {
        let s1_vec: Vec<char> = s1.chars().collect();
        let s2_vec: Vec<char> = s2.chars().collect();
        let longer_vec: Vec<char>;
        let shorter_vec: Vec<char>;
        if s1_vec.len() >= s2_vec.len() {
            longer_vec = s1_vec;
            shorter_vec = s2_vec;
        } else {
            longer_vec = s2_vec;
            shorter_vec = s1_vec;
        }
        for n in 0..longer_vec.len() {
            dbg!(&shorter_vec);
            dbg!(&longer_vec);
            if longer_vec[n] != shorter_vec[n] {
                distance += 1;
            }
            if shorter_vec.len() - 1 == n {
                distance += longer_vec.len() - shorter_vec.len();
                break;
            }
        }
    }
    dbg!(distance);
    distance < 2
}

// Q6 String Compression
// Implement a method to perform basic string compression using the
// counts of repeated charts.
// if the compressed string is not smaller than the original string, return original string
// assume string is only Aa-Zz
// (aabcccccaaa) -> a2b1c5a3

#[allow(dead_code)]
fn compress_string(s: &str) -> String {
    let mut result: String = "".to_string();
    let char_vec: Vec<char> = s.chars().collect();
    let mut current_char = char_vec[0];
    let mut current_count: i32 = 0;

    let word_length = char_vec.len();
    for n in 0..word_length {
        //  handle last char
        if char_vec[n] == current_char {
            current_count += 1;
            continue;
        }
        result.push_str(&format!("{}{}", current_char, current_count));
        current_char = char_vec[n];
        current_count = 1;
    }

    result.push_str(&format!("{}{}", current_char, current_count));
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn unique_word() {
        let s = "abcd";
        assert_eq!(is_all_chars_unique(s), true);
    }
    #[test]
    fn not_unique_word() {
        let s = "aca";
        assert_eq!(is_all_chars_unique(s), false);
    }

    #[test]
    fn premutation_words() {
        let s1 = "ab";
        let s2 = "ba";
        assert_eq!(is_premutation(s1, s2), true);
    }
    #[test]
    fn not_premutation_words() {
        let s1 = "aba";
        let s2 = "ba";
        assert_eq!(is_premutation(s1, s2), false);
    }
    #[test]
    fn not_premutation_words2() {
        let s1 = "Ab";
        let s2 = "ba";
        assert_eq!(is_premutation(s1, s2), false);
    }

    #[test]
    fn palindrome() {
        let s1 = "aba";
        assert_eq!(is_palindrome(s1), true);
    }

    #[test]
    fn not_palindrome() {
        let s1 = "abm";
        assert_eq!(is_palindrome(s1), false);
    }

    #[test]
    fn compressed_string() {
        let s1 = "aacccbC";
        assert_eq!(compress_string(s1), "a2c3b1C1");
    }
    #[test]
    fn words_not_one_away() {
        let s1 = "abm";
        let s2 = "abxx";
        assert_eq!(one_away(s1, s2), false);
    }
    #[test]
    fn words_not_one_away1() {
        let s1 = "abm";
        let s2 = "aamx";
        assert_eq!(one_away(s1, s2), false);
    }
    #[test]
    fn words_one_away() {
        let s1 = "abm";
        let s2 = "abx";
        assert_eq!(one_away(s1, s2), true);
    }
    #[test]
    fn words_one_away1() {
        let s1 = "aaa";
        let s2 = "aaa";
        assert_eq!(one_away(s1, s2), true);
    }
    #[test]
    fn words_one_away2() {
        let s1 = "";
        let s2 = "";
        assert_eq!(one_away(s1, s2), true);
    }
    #[test]
    fn words_one_away3() {
        let s1 = "aaa";
        let s2 = "";
        assert_eq!(one_away(s1, s2), false);
    }
}
