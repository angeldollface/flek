/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Imports the "clean_split"
/// method from "utils" to split
/// strings.
use super::utils::clean_split;

/// Imports the "string_type"
/// method from "utils" to
/// determine the string type.
use super::utils::string_type;

/// Imports the "get_rand_item"
/// method from "utils" to
/// get random items from vectors.
use super::utils::get_rand_item;

/// Imports the "get_num_space"
/// method from "utils" to
/// get the space between two numbers.
use super::utils::get_num_space;

/// Imports the "get_item_index"
/// method from "utils" to
/// get the index of an item
/// in a vector.
use super::utils::get_item_index;

/// Imports the "get_char_space"
/// method from "utils" to
/// get the space between two chars.
use super::utils::get_char_space;

/// Imports the "SECURITY_WEIGHT"
/// variable from "weights".
use super::weights::SECURITY_WEIGHT;

/// Imports the "special_char_weight"
/// variable from "weights".
use super::weights::SPECIAL_CHAR_WEIGHT;

/// Imports the "arabic_character_weight"
/// variable from "weights".
use super::weights::ARABIC_CHARACTER_WEIGHT;

/// A struct to represent password security
/// information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SecurityInfo {
    pub score: usize,
    pub is_secure: bool
}

/// Implements standard methods for this struct.
impl SecurityInfo {

    /// Creates a new instance of the struct.
    pub fn new(score: &usize, is_secure: &bool) -> SecurityInfo {
        return SecurityInfo {
            score: score.to_owned(),
            is_secure: is_secure.to_owned()
        }
    }

    /// Returns a string representation of the
    /// password's security information.
    pub fn to_string(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        result.push(self.score.to_string());
        result.push(self.is_secure.to_string());
        return result;
    }
}

/// Compute the strength of a password.
pub fn password_strength(password: &String) -> usize {
    let mut result: usize = 0;
    let char_list: Vec<String> = clean_split(&password, &String::from(""));
    for item in &char_list {
        let current_item: &String = &item;
        let current_item_type: String = string_type(&item);
        let current_index: usize = get_item_index(&char_list, &item);
        if &current_index == &0 {
            // Do nothing.
        }
        else {
            let previous_item_index = current_index - 1;
        let previous_item: &String = &char_list.clone()[previous_item_index].clone();
        let previous_item_type = string_type(&previous_item);
        if current_item_type == String::from("normChar") && previous_item_type == String::from("normChar") {
            let item_space = get_char_space(&current_item, &previous_item);
            if item_space > SECURITY_WEIGHT {
                result = result + ARABIC_CHARACTER_WEIGHT;
            } else {
                // Do nothing.
            }
        } else if current_item_type == String::from("specialChar") &&
            previous_item_type == String::from("specialChar") {
                result = result + SPECIAL_CHAR_WEIGHT;
        } else if current_item_type == String::from("int") && previous_item_type == String::from("int") {
            let item_space: usize = get_num_space(&current_item, &previous_item);
            if item_space > SECURITY_WEIGHT {
                result = result + ARABIC_CHARACTER_WEIGHT;
            } else {
                // Do nothing.
            }
        }
        else {}
        }
    }
    return result;
}

/// Simplified strenght measure.
pub fn is_secure(password: &String) -> bool {
    let mut result: bool = false;
    if password_strength(password) > 8 {
        result = true;
    }
    else {}
    return result;
}

/// Returns an instance of the "SecurityInfo" struct of a
/// password.
pub fn security_info(password: &String) -> SecurityInfo {
    return SecurityInfo::new(
        &password_strength(password),
        &is_secure(password)
    );
}

/// Generate a random password.
pub fn generate_password(length:&usize) -> String {
    let mut result_list: Vec<String> = Vec::new();
    let alphabet_string: String = String::from(
        "abcdefghijklmnopqrstuvwxyz1234567890@_;.:"
    );
    let alphabet_list: Vec<String> = clean_split(
        &alphabet_string, &String::from("")
    );
    let range_end: usize = length + 1;
    for _i in 1..range_end {
        let rand_char: String = get_rand_item(&alphabet_list.clone());
        result_list.push(rand_char);
    }
    let result: String = result_list.join("");
    return result;
}