# Running the Repository

git clone `https://github.com/Adidev-KGP/POC_ChaCha20_Crate.git`

`cargo build` to build the project

`cargo test` to run all the tests

# Publishing as a separate crate on crate.io

Modify the following in the Cargo.toml file of the root directory:

`name`

`version`

`edition`

`description`

`authors`

After modifying the `Cargo.toml` file run `cargo publish` to publish the new crate on `cargo.io`

# Folder Structure

The folder `src` 1 file and 1 folder :

`chacha20/chacha20.rs` -> Has all the code for `ChaCha20` algorithm

`fschacha20.rs` -> Has all the code to use the chacha20 algorithm to produce keystreams and hence produce the encrypted data.

`lib.rs` -> It is a default file in rust that says it's a `library crate`. It exports all the code in `chacha20/chacha20.rs` and `fschacha20.rs` so that they can be used in other files as well.

`tests/fschacha20.rs` -> Has unit test for `FsChaCha20` implementation against a standard test vector. You may find the test vector in the following commit :https://github.com/bitcoin/bitcoin/pull/25361/commits/acd664e8055954ecf8be8bd526455e88bf88bf2c

# The .github/workflows/rust-ci.yml

This workflow runs on every push to the `main` branch and every pull request into the main branch. It installs the stable Rust toolchain, Rustfmt, and Clippy. It then runs Clippy and Rustfmt checks using the stable toolchain, and builds and tests the code with the nightly toolchain.

# Contributions

Make changes and raise PR on a separate branch. IF they pass the workflows with the logic, they will be merged.

# THANKS