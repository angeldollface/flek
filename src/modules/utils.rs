/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "is_int"
/// method from the "coutils"
/// crate to check whether
/// a given character is
/// an integer.
use coutils::is_int;

/// Importing the "parse_int"
/// method from the "coutils"
/// crate to convert a given
/// character into an unsigend
/// integer.
use coutils::parse_int;

/// Importing the "get_index"
/// method from the "coutils"
/// crate to get the index
/// of an item in a vector.
use coutils::get_index;

/// Imports the "clean_split"
/// method from the "coutils" crate 
/// to split strings into a
/// vector of strings.
use coutils::clean_split;

/// Importing Flek's error-handling
/// data structure.
use super::error::FlekError;

/// Get the character position from a string if possible.
pub fn get_char_pos(char: &String) -> Result<usize,FlekError> {
    let mut result: usize = 0;
    let alphabet: String = String::from(
        "abcdefghijklmnopqrstuvwxyz"
    );
    let alphabet_list: Vec<String> = clean_split(&alphabet, &String::from(""));
    for letter in &alphabet_list {
        if letter == char {
            result = match get_index(&alphabet_list.to_owned(), &letter){
                Ok(result) => result,
                Err(e) => {
                    return Err::<usize, FlekError>(FlekError::new(&e.to_string()));
                }
            }
        }
        else {
            // Do nothing.
        }
    }
    return Ok(result);
}

/// Get the distance between two characters if possible.
pub fn get_char_space(char_one: &String, char_two: &String) -> Result<usize,FlekError> {
    let char_one_index = match get_char_pos(&char_one){
        Ok(char_one_index) => char_one_index,
        Err(e) => {
            return Err::<usize, FlekError>(FlekError::new(&e.to_string()));
        }
    };
    let char_two_index = match get_char_pos(&char_two) {
        Ok(char_two_index) => char_two_index,
        Err(e) => {
            return Err::<usize, FlekError>(FlekError::new(&e.to_string()));
        }
    };
    let mut result: usize = 0;
    if &char_one_index > &char_two_index {}
    else {
        result = char_two_index - char_one_index;
    }
    return Ok(result);
}

/// Get the space between two numbers.
pub fn get_num_space(num_one: &String, num_two: &String) -> usize {
    let mut result: usize = 0;
    if parse_int(&num_one) > parse_int(&num_two) {}
    else {
        result = parse_int(&num_two) - parse_int(&num_one);
    }
    return result;
}

/// Get the type of string.
pub fn string_type(char: &String) -> String {
    let mut result: String = String::from("int");
    if is_int(&char) == false {
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