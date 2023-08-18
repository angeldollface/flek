/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method
/// to generate a SHA 256
/// digest of a secure
/// password.
use sha256::digest;

/// Importing the "Serialize"
/// trait macro to serialize a 
/// Rust data structure.
use serde::Serialize;

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

/// Imports the "get_rand_item"
/// method from the "coutils" crate to
/// get random items from vectors.
use coutils::get_rand_item;

/// Importing Flek's error-handling
/// data structure.
use super::error::FlekError;

/// Imports the "string_type"
/// method from "utils" to
/// determine the string type.
use super::utils::string_type;

/// Imports the "get_num_space"
/// method from "utils" to
/// get the space between two numbers.
use super::utils::get_num_space;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SecurityInfo {
    pub password: String,
    pub score: usize,
    pub is_secure: bool
}

/// Implements standard 
/// methods for the "SecurityInfo" struct.
impl SecurityInfo {

    /// Creates a new instance 
    /// of the "SecurityInfo" struct.
    pub fn new(password: &String, score: &usize, is_secure: &bool) -> SecurityInfo {
        return SecurityInfo {
            password: password.to_owned(),
            score: score.to_owned(),
            is_secure: is_secure.to_owned()
        }
    }

    /// Returns a string representation of the
    /// password's security information.
    pub fn to_string(&self) -> String {
        return format!(
            "Password: {}\nScore: {}\nStatus: {}",
            &self.password, 
            &self.score, 
            &self.is_secure
        );
    }

}

/// A struct to represent password security
/// information.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PasswordInfo {
    pub password: String,
    pub sha_sum: String
}

/// Implements standard 
/// methods for the "PasswordInfo" struct.
impl PasswordInfo{

    /// Creates a new instance 
    /// of the "PasswordInfo" struct.
    pub fn new(password: &String) -> PasswordInfo {
        let digest: String = digest(password);
        return PasswordInfo {
            password: password.to_owned(),
            sha_sum: digest
        }
    }

    /// Returns a string representation of the
    /// password.
    pub fn to_string(&self) -> String {
        return format!(
            "Password: {}\nSHA 256: {}",
            &self.password, 
            &self.sha_sum, 
        );
    }

}

/// Compute the strength of a password.
pub fn password_strength(password: &String) -> Result<usize, FlekError> {
    let mut result: usize = 0;
    let char_list: Vec<String> = clean_split(&password, &String::from(""));
    for item in &char_list {
        let current_item: &String = &item;
        let current_item_type: String = string_type(&item);
        let current_index: usize = match get_index(&char_list, &item){
            Ok(current_index) => current_index,
            Err(e) => {
                return Err::<usize, FlekError>(FlekError::new(&e.to_string()));
            }
        };
        if &current_index == &0 {
            // Do nothing.
        }
        else {
            let previous_item_index = current_index - 1;
        let previous_item: &String = &char_list.clone()[previous_item_index].clone();
        let previous_item_type = string_type(&previous_item);
        if current_item_type == String::from("normChar") && previous_item_type == String::from("normChar") {
            let item_space = match get_char_space(&current_item, &previous_item){
                Ok(item_space) => item_space,
                Err(e) => {
                    return Err::<usize, FlekError>(FlekError::new(&e.to_string()));
                }
            };
            if item_space > SECURITY_WEIGHT {
                result = result + ARABIC_CHARACTER_WEIGHT;
            } else {}
        } else if current_item_type == String::from("specialChar") &&
            previous_item_type == String::from("specialChar") {
                result = result + SPECIAL_CHAR_WEIGHT;
        } else if current_item_type == String::from("int") && previous_item_type == String::from("int") {
            let item_space: usize = get_num_space(&current_item, &previous_item);
            if item_space > SECURITY_WEIGHT {
                result = result + ARABIC_CHARACTER_WEIGHT;
            } else {}
        }
        else {}
        }
    }
    return Ok(result);
}

/// Simplified strength measure.
pub fn is_secure(password: &String) -> Result<bool, FlekError> {
    let mut result: bool = false;
    let score: usize = match password_strength(&password){
        Ok(score) => score,
        Err(e) => {
            return Err::<bool, FlekError>(FlekError::new(&e.to_string()));
        }
    };
    if score > 8 {
        result = true;
    }
    else {}
    return Ok(result);
}

/// Returns an instance of the "SecurityInfo" struct of a
/// password.
pub fn security_info(password: &String) -> Result<SecurityInfo, FlekError> {
    let score: usize = match password_strength(&password){
        Ok(score) => score,
        Err(e) => {
            return Err::<SecurityInfo, FlekError>(FlekError::new(&e.to_string()));
        }
    };
    let status: bool = match is_secure(&password){
        Ok(status) => status,
        Err(e) => {
            return Err::<SecurityInfo, FlekError>(FlekError::new(&e.to_string()));
        }
    };
    return Ok(
        SecurityInfo::new(
            &password,
            &score,
            &status
        )
    );
}

/// Generate a random password of specified length.
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

/// Generate a password that has been checked
/// by my algorithm. In the event of this not
/// being possible, an error is returned.
pub fn generate_secure_password(
    length: &usize
) -> Result<String, FlekError> {
    let mut result: String = String::from("");
    let mut security_tracker: bool = true;
    while security_tracker {
        let pwd = generate_password(length);
        match is_secure(&pwd){
            Ok(status) => {
                if status {
                    result = pwd;
                    security_tracker = false;
                }
                else {}
            }
            Err(e) => {
                return Err::<String, FlekError>(FlekError::new(&e.to_string()));
            }
        }
    }
    return Ok(result);
}

/// A method to generate an instance of a
/// data structure that holds information about
/// a secure password.
pub fn secure_password_info(
    length: &usize
) -> Result<PasswordInfo, FlekError> {
    match generate_secure_password(length){
        Ok(pwd) => return Ok(PasswordInfo::new(&pwd)),
        Err(e) => return Err::<PasswordInfo, FlekError>(FlekError::new(&e.to_string()))
    };
}