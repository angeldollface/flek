/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Exporting the module that
/// contains Flek's CLI.
pub mod cli;

/// Exporting the testing module
// and setting up tests.
#[cfg(test)]
pub mod tests;

/// Exporting the module 
/// with Flek's main functions.
pub mod flek;

/// Exporting the module that contains
/// utility functions.
pub mod utils;

/// Exporting the error
/// module.
pub mod error;

/// Exporting the module that contains
/// security weights.
pub mod weights;