/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "is_secure"
/// method from the main API.
use super::flek::is_secure;

/// Importing the "SecurityInfo"
/// struct from from the main API.
use super::flek::SecurityInfo;

/// Importing the "security_info"
/// method from from the main API.
use super::flek::security_info;

/// Importing the "password_strength"
/// method from from the main API.
use super::flek::password_strength;

/// Testing the "is_secure" method.
#[test]
pub fn test_is_secure() -> () {
    let pwd: &String = &String::from("1969HoglinSteak_@@");
    match is_secure(pwd){
        Ok(res) => {
            assert_eq!(true, res);
        },
        Err(e) => {
            println!("{}", &e.to_string());
        }
    };
}

/// Testing the "password_strength" method.
#[test]
pub fn test_pwd_strength() -> () {
    let pwd: &String = &String::from("1969HoglinSteak_@@");
    match password_strength(pwd){
        Ok(res) => {
            assert_eq!(12, res);
        },
        Err(e) => {
            println!("{}", &e.to_string());
        }
    };
}

/// Testing the "security_info" method.
#[test]
pub fn test_security_info() -> () {
    let pwd: &String = &String::from("1969HoglinSteak_@@");
    match security_info(pwd){
        Ok(res) => {
            let sample: SecurityInfo = SecurityInfo::new(&pwd, &12, &true);
            assert_eq!(sample, res);
        },
        Err(e) => {
            println!("{}", &e.to_string());
        }
    };
}