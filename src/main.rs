//Arrays and Strings

// Q1
// Implement an algorithm to determine if a string has all unique chars.

use itertools::Itertools;

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

fn one_away(s1: &str, s2: &str) -> bool {
    false
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
    fn words_one_away() {
        let s1 = "abm";
        let s2 = "abx";
        assert_eq!(one_away(s1, s2), true);
    }
}
