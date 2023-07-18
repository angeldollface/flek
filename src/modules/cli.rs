/*
CLEASY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the main
/// Cliplyy API struct.
use cliply::App;

/// Importing the "is_secure"
/// method from the main Flek
/// API.
use super::flek::is_secure;

/// Importing the "password_strength"
/// method from the main Flek
/// API.
use super::flek::password_strength;

/// CLI for Flek.
pub fn cli() -> () {

    // Instantiating the "App" struct with the required
    // data.
    let mut cflek: App = App::new(
        &"Flek",
        &"1.3.0",
        &"Angel Dollface"
    );

    // Adding the password flag to accept user passwords.
    cflek.add_arg(
        &"pwd", 
        &"      password to check", 
        &"true"
    );

    // Was the version flag used?
    if cflek.version_is() == true {
        println!("{}", cflek.version_info());
    }

    // Was the help flag used?
    else if cflek.help_is() == true {
        println!("{}", cflek.help_info());
    }

    // Was the password flag used?
    else if cflek.arg_was_used(&"pwd") == true {
        let arg_data = cflek.get_arg_data(&"pwd");
        match arg_data{
            Ok(arg_data) => {
                let msg: String = format!(
                    "Security score: {:?}\nPassword secure: {:?}",
                    password_strength(&arg_data),
                    is_secure(&arg_data)
                );
                println!("{}", msg);
            },
            Err(e) => {
                println!("{}", &e.to_string());
            }
        };
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        println!("{}", cflek.help_info());
    }
}
