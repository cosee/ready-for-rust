# ready-for-rust techtalk

## Get started with Rust & CLion
1) Install Rust (https://www.rust-lang.org/tools/install). 
   For macOS, Linux you can start the installer with
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
2) Install CLion & the open-source Rust plugin (https://intellij-rust.github.io/).
3) Open this folder with CLion. 
   It should should automatically detect your Rust toolchain. 
   You can download and set the standard library with a click.
4) Profit. You should now be able to start tests in this project from CLion (e.g. in [slide2.rs](src/slides/slide2.rs)).

## How to run on shell
`make help` documents the relevant cargo targets.