# FLEK :crab: :ribbon:

![GitHub CI](https://github.com/angeldollface/flek/actions/workflows/rust.yml/badge.svg)

***A Rustacean implementation of my own algorithms to check for password security.:crab: :ribbon:***

## ABOUT :books:

This is my Rust implementation of a package I wrote in [Dart](https://github.com/angeldollface/securitycheck) roughly a year ago and implemented also in [ECMA Script](https://github.com/angeldollface/vulcheck) only a couple of days ago. These packages all do one thing: They provide functions for you to check whether your passwords are secure or not. My algorithm gives your password a score and if the score is higher than eight, then the password is deemed to be secure. Why ***Flek***? The name is a combination of the words ***Fl***aw and Ch***e(c)k***. All of these packages implement an algorithm of my own design and have all been optimized to be as fast as possible. Enjoy. :heart:

## INSTALLATION :inbox_tray:

### FOR RUST PROJECTS

To use ***Flek*** in your Rust project, add this line to your project's `Cargo.toml`:

```TOML
flek = { git = "https://github.com/angeldollface/flek", version = "1.2.0" }
```

### FOR THE COMMAND LINE

Execute these steps in order:

- 1.) Download the executable for your platform from the [Releases Section](https://github.com/angeldollface/flek/releases).
- 2.) Save it in a location that is on your system path.

If your platform's architecture isn't in the list of executables, please file an issue [here](https://github.com/angeldollface/flek/issues). If you encounter any other problems please also file an issue.

## USAGE :hammer:

### APIs

For usage instuctions on ***Flek***'s functions, please check out the `src` directory. The most important functions are in `src/modules/flek.rs`.

### COMMAND LINE

To get info about one of your passwords' security, make sure the `cf` command is available from the command line. If the command is available, you can run these commands from the command line:

- Get security info about your password:

```bash
cf -p 1969HoglinSteak_@
# or
cf --pwd 1969HoglinSteak_@
```

- Get version info:

```bash
cf -v
# or
cf --version
```

- Get help info:

```bash
cf -h
# or
cf --help
```

## LINKS :heart_on_fire:

There are other implementations of my algorithm in other languages:

- Dart: [VIEW](https://github.com/angeldollface/securitycheck)
- ECMA Script: [VIEW](https://github.com/angeldollface/vulcheck)
- A small web app showcasing the algorithm: [VIEW](https://github.com/angeldollface/vcheck)

## CHANGELOG :black_nib:

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

## NOTE :scroll:

- *Flek :crab: :ribbon:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
