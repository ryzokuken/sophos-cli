# `sophosd`

Daemon for connecting to Sophos written in Rust.

## Dependencies

For Windows and MacOS users

* Rust compiler and Cargo Package manager (see https://www.rust-lang.org/tools/install)

For Linux users

* Rust compiler and Cargo Package manager (see https://www.rust-lang.org/tools/install)
* OpenSSL 1.0.1, 1.0.2, or 1.1.0 with headers (see https://github.com/sfackler/rust-openssl)
* OpenSSL Development Package - this is `libssl-dev` on Ubuntu and `openssl-devel` on Fedora

## Installation and Usage

Clone this repository

`cd` to base directory where `Cargo.toml` is located and type:

```cargo run <username> <password> ```

