# `sophosd`

A CLI daemon for connecting to Sophos

## Dependencies

#### Windows Users
* [Git](https://git-scm.com)
* Microsoft Visual C++ build tools (see https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019)
* Rust compiler and Cargo Package manager (see https://www.rust-lang.org/tools/install)

#### MacOS users
* Rust compiler and Cargo Package manager (see https://www.rust-lang.org/tools/install)

#### For Linux users
* Rust compiler and Cargo Package manager (see https://www.rust-lang.org/tools/install)
* OpenSSL Development Package - this is `libssl-dev` on Ubuntu and `openssl-devel` on Fedora

## Installation and Usage

Clone this repository

To build the binaries and run ```sophosd ```, `cd` to base directory where `Cargo.toml` and type:

``` cargo run <username> <password> ```

## Contributing

This is an open source project and we would love your help. If you are having trouble regarding anything related to this project open a issue in the [issue section](github.com/ryzokuken/sophosd/issues)

## License

```sophosd``` is released under MIT License
