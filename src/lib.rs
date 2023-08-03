/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module that
/// contains Flek's CLI.
pub use modules::cli::*;

/// Re-exporting the module 
/// with Flek's main functions.
pub use modules::flek::*;

/// Re-exporting the module that contains
/// utility functions.
pub use modules::utils::*;

/// Re-exporting the module that contains
/// security weights.
pub use modules::weights::*;