/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// The command-line interface
/// module.
pub mod cli;

/// Exporting the testing module
// and setting up tests.
#[cfg(test)]
pub mod tests;

/// The module with Flek's
/// main functions.
pub mod flek;

/// Tools for Flek.
pub mod utils;

/// Security weights.
pub mod weights;