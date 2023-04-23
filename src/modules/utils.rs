/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the API for random numbers.
use rand::Rng;

/// Importing core traits:
/// "Debug".
use core::fmt::Debug;

/// Importing core traits:
/// "PartialEq".
use std::cmp::PartialEq;

/// Get a random item from a string vector.
pub fn get_rand_item(subject: &Vec<String>) -> String {
    let mut range = rand::thread_rng();
    return subject[range.gen_range(0..subject.len())].clone();
}

/// Convert string to a number.
pub fn conv_to_num(char: &String) -> usize {
    let mut result: usize = 0;
    if is_num(&char) == true {
        result = char.parse::<usize>().unwrap();
    }
    else {}
    return result;
}

/// Get index of a list item and return it.
pub fn get_item_index<T: Debug + Clone + PartialEq>(subject: &Vec<T>, item: &T) -> usize {
    return subject.iter().position(|r| r == item).unwrap();
}

/// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
pub fn clean_split(subject: &String, split_char: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&*split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

/// Get the character position from a list.
pub fn get_char_pos(char: &String) -> usize {
    let mut result: usize = 0;
    let alphabet: String = String::from(
        "abcdefghijklmnopqrstuvwxyz"
    );
    let alphabet_list: Vec<String> = clean_split(&alphabet, &String::from(""));
    for letter in &alphabet_list {
        if letter == char {
            result = get_item_index(&alphabet_list.to_owned(), &letter);
        }
        else {
            // Do nothing.
        }
    }
    return result;
}

/// Get the distance between two characters.
pub fn get_char_space(char_one: &String, char_two: &String) -> usize {
    let char_one_index = get_char_pos(&char_one);
    let char_two_index = get_char_pos(&char_two);
    let mut result: usize = 0;
    if &char_one_index > &char_two_index {
        // Do nothing.
    }
    else {
        result = char_two_index - char_one_index;
    }
    return result;
}

/// Get the space between two numbers.
pub fn get_num_space(num_one: &String, num_two: &String) -> usize {
    let mut result: usize = 0;
    if conv_to_num(&num_one) > conv_to_num(&num_two) {
        // Do nothing.
    }
    else {
        result = conv_to_num(&num_two) - conv_to_num(&num_one);
    }
    return result;
}

/// Get the type of string.
pub fn string_type(char: &String) -> String {
    let mut result: String = String::from("int");
    if is_num(&char) == false {
        let alphabet: String = String::from(
            "abcdefghijklmnopqrstuvwxyz"
        );
        let alphabet_list: Vec<String> = clean_split(&alphabet, &String::from(""));
        if alphabet_list.contains(&char) == true {
            result = String::from("normChar");
        }
        else {
            result = String::from("specialChar");
        }
    }
    else {}
    return result;
} 

/// Check if a number is a number or not.
pub fn is_num(char: &String) -> bool {
    let mut result: bool = false;
    let match_op = char.parse::<usize>();
    match match_op {
        Ok(_x) => {
            result = true;
        },
        Err(_e) => {}
    };
    return result;
}