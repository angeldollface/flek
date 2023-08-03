/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the main
/// Cliply API struct.
use cliply::App;

/// Importing the "security_info"
/// method from the main Flek module
/// to retrieve analysis info of 
/// a password.
use super::flek::security_info;

/// CLI for Flek.
pub fn cli() -> () {

    // Instantiating the "App" struct with the required
    // data.
    let mut app: App = App::new(
        &"Flek",
        &"1.4.0",
        &"Angel Dollface"
    );

    // Adding the password flag to accept user passwords.
    app.add_arg(
        &"pwd", 
        &"      password to check", 
        &"true"
    );

    // Was the version flag used?
    if app.version_is() {
        println!("{}", app.version_info());
    }

    // Was the help flag used?
    else if app.help_is() {
        println!("{}", app.help_info());
    }

    // Was the password flag used?
    else if app.arg_was_used(&"pwd") == true {
        let arg_data = app.get_arg_data(&"pwd");
        match arg_data{
            Ok(arg_data) => {
                match security_info(&arg_data){
                    Ok(info) => {
                        println!("{}", info.to_string());
                    },
                    Err(e) => {
                        println!("{}", &e.to_string());
                    }
                };
            },
            Err(e) => {
                println!("{}", &e.to_string());
            }
        };
    }

    // If user-supplied flags are invalid, show
    // them the app's help message.
    else {
        println!("{}", app.help_info());
    }
}
