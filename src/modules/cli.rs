/*
CLEASY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the main
/// Cleasy API struct.
use cleasy::App;

use super::flek::is_secure;
use super::flek::password_strength;

/// CLI for Flek.
pub fn cli() -> () {

    // Instantiating the "App" struct with the required
    // data.
    let mut cflek: App = App::new(
        &"CFlek",
        &"1.0.0",
        &"Alexander Abraham"
    );

    // Adding the password flag to accept user passwords.
    cflek.add_arg(
        &"pwd", 
        &"password to check", 
        &"true"
    );

    // Was the version flag used?
    if cflek.version_is() == true {
        println!("{}", cflek.version());
    }

    // Was the help flag used?
    else if cflek.help_is() == true {
        println!("{}", cflek.help());
    }

    // Was the password flag used?
    else if cflek.arg_was_used(&"pwd") == true {
        let arg_data: String = cflek.get_arg_data(&"pwd");
        let msg: String = format!(
            "Security score: {:?}\nPassword secure: {:?}",
            password_strength(&arg_data),
            is_secure(&arg_data)
        );
        println!("{}", msg);
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        println!("{}", cflek.help());
    }
}