use std::collections::{HashMap, HashSet};

fn word_pattern(pattern: &str, str_value: &str) -> bool {

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let word: Vec<&str> = str_value.split_whitespace().collect();

    if pattern_chars.len() != word.len(){
        return false;
    }

}

fn main (){
}