# Sophosd

Daemon for connecting to Sophos written in Rust.

# Installation and Usage

The [Rust compiler and Cargo package manager](https://www.rust-lang.org/learn/get-started) are needed. You may also need to install an OpenSSL development package - this is `libssl-dev` on Ubuntu and `openssl-devel` on Fedora.

Clone this repository and `cd` to base directory where `Cargo.toml` is located and use the `cargo` commands from here. For example to use use sophos  by using `cargo run` just type:

```$ cargo run <username> <password> ```

Alternatively you can also create a binary Debian package `.deb` file using the cargo-deb command. The resulting package would be placed in `/target/debian/`.

If you have installed sophosd using the `.deb` file you can use it in the command line by typing in:

```$ sophsosd <username> <password>```
