# `sophosd`

Daemon for connecting to Sophos written in Rust.

# Installation and Usage

As there is no binary release you'd have to compile the source code on your own machine. For that the [Rust compiler and Cargo package manager](https://www.rust-lang.org/learn/get-started) are needed. You may also need to install an OpenSSL development package - this is `libssl-dev` on Ubuntu and `openssl-devel` on Fedora.

Clone this repository

## Using [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html)

`cd` to base directory where `Cargo.toml` is located and type:

```cargo run <username> <password> ```
