# FLEK

![GitHub CI](https://github.com/angeldollface/flek/actions/workflows/rust.yml/badge.svg)

***A Rustacean implementation of my own algorithms to check for password security.***

## ABOUT

This is my Rust implementation of a package I wrote in [Dart](https://github.com/angeldollface/securitycheck) roughly a year ago and implemented also in [ECMA Script](https://github.com/angeldollface/vulcheck). These packages all do one thing: They provide functions for you to check whether your passwords are secure or not. My algorithm gives your password a score and if the score is higher than eight, then the password is deemed to be secure. Why ***Flek***? The name is a combination of the words ***Fl***aw and Ch***e(c)k***. All of these packages implement an algorithm of my own design and have all been optimized to be as fast as possible. Enjoy.

## INSTALLATION

### FOR RUST PROJECTS

To use ***Flek*** in your Rust project, add this line to your project's `Cargo.toml`:

```TOML
flek = "1.9.0"
```

### FOR THE COMMAND LINE

You can install ***Flek*** via Cargo itself using this command:

```bash
cargo install flek
```

Alternatively, you can download a compiled binary for 64-bit systems from this repository's [Releases](https://github.com/angeldollface/flek/releases) section.

I recently uploaded ***Flek*** to the Arch User Repository so you can install ***Flek*** from there using this command:

```bash
paru -S flek-bin
```

The package's page on the AUR can be viewed [here](https://aur.archlinux.org/packages/flek-bin).

## USAGE

### APIs

For usage instuctions on ***Flek***'s functions and structures, please read the documentation [here](https://docs.rs/flek/1.9.0).

### COMMAND LINE

To get info about the security analysis of a password, make sure the `flek` command is available from the command line. If the command is available, you can run these commands from the command line:

- Get security info about your password:

```bash
flek -p 1969HoglinSteak_@
# OR
flek --pwd 1969HoglinSteak_@
# OR
flek pwd 1969HoglinSteak_@
```

- Generate a secure password from a specified length:

```bash
flek -g 19
# OR
flek --gen 19
# OR
flek gen 19
```

- Get version info:

```bash
flek -v
# OR
flek --version
# OR
flek version
```

- Get help info:

```bash
flek -h
# OR
flek --help
# OR
flek help
```

## LINKS

There are other implementations of my algorithm in other languages:

- Dart: [VIEW](https://github.com/angeldollface/securitycheck)
- ECMA Script: [VIEW](https://github.com/angeldollface/vulcheck)
- A small web app showcasing the algorithm: [VIEW](https://github.com/angeldollface/vcheck.rs)

## CHANGELOG

### Version 1.0.0

- Initial release.
- Upload to GitHub.

### Version 1.1.0

- The Doll Update.
- Updated, published, and uploaded under my new name.

### Version 1.2.0

- Split the code into modules.
- Added a small CLI command to the project.
- Added unit tests.

### Version 1.3.0

- Updated dependency for CLI building.
- Aggressive optimization of the executable.
- Aggressively better error handling.

### Version 1.4.0

- Added even better handling of errors.
- Removed redundant functions.
- Added a new dependency: `coutils`.
- Removed the `rand` dependency.

### Version 1.5.0

- Updated the version of the `cliply` crate.
- Updated the version of the `coutils` crate to work better with WASM builds.
- Updated documentation.

### Version 1.6.0

- Updated documentation.
- Updated the version of the CLI tool.
- Automatic generation of binaries for 64-bit desktop platforms.

### Version 1.7.0

- Updated documentation.
- Updated the version of the CLI tool.
- Added a method to generate a secure password.
- Added a flag to the CLI tool to generate a secure password.
- Added a data structure to hold information about a secure password.

### Version 1.8.0

- Updated documentation.
- Added an error case for when an invalid length is supplied to the API for generating a secure password!
- Added an error case for when an empty string is supplied to the API for password analysis.

### Version 1.9.0

- Updated documentation.
- Fixed typos and wrong instructions.

## NOTE

- *Flek* by Alexander Abraham a.k.a. *"Angel Dollface"*
- Licensed under the MIT license.
