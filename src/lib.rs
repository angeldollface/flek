/*
FLEK by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Decalring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the CLI
/// module.
pub use modules::cli::*;

/// Re-exporting the main
/// Flek module.
pub use modules::flek::*;

/// Re-exporting the tools
/// for the Flek module.
pub use modules::utils::*;

/// Re-exporting the weights
/// for the Flek module.
pub use modules::weights::*;
