# `sophosd`

Daemon for connecting to Sophos written in Rust.

# Installation and Usage

As there is no binary release you'd have to compile the source code on your own machine. For that the [Rust compiler and Cargo package manager](https://www.rust-lang.org/learn/get-started) are needed. You may also need to install an OpenSSL development package - this is `libssl-dev` on Ubuntu and `openssl-devel` on Fedora.

Clone this repository

## Method 1: Using [`cargo run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html)

`cd` to base directory where `Cargo.toml` is located and type:

```cargo run <username> <password> ```

## Method 2: Using [`cargo deb`](https://docs.rs/cargo-deb/1.21.1/cargo_deb/) (only for Debian based Linux distributions)

If you don't want to `cd` to the base directory of `sophosd` every time you want to use it, you can create a binary Debian package `.deb` using the `cargo-deb` command and install it.

First you would need to install the `cargo deb` command using:

```cargo install cargo-deb```

Now, run this in your Cargo project directory

```cargo deb```

The resulting package would be placed in `/target/debian/`. Install this and now you can use `sophosd` by typing

```sophsosd <username> <password>```
