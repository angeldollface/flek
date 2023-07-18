/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "is_secure"
/// method from "flek".
use super::flek::is_secure;

/// Importing the "SecurityInfo"
/// struct from "flek".
use super::flek::SecurityInfo;

/// Importing the "security_info"
/// method from "flek".
use super::flek::security_info;

/// Importing the "password_strength"
/// method from "flek".
use super::flek::password_strength;

/// Testing the "is_secure" method.
#[test]
pub fn test_is_secure() -> () {
    let res: bool = is_secure(&String::from("1969HoglinSteak_@@"));
    assert_eq!(true, res);
}

/// Testing the "password_strength" method.
#[test]
pub fn test_pwd_strength() -> () {
    let res: usize = password_strength(&String::from("1969HoglinSteak_@@"));
    assert_eq!(12, res);
}

/// Testing the "security_info" method.
#[test]
pub fn test_security_info() -> () {
    let res: SecurityInfo = security_info(&String::from("1969HoglinSteak_@@"));
    let sample: SecurityInfo = SecurityInfo::new(&12, &true);
    assert_eq!(sample, res);
}