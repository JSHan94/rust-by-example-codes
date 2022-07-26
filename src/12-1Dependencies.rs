// Rust manages dependencies with Cargo

fn main() {
    // Create Rust Project
    // $ cargo new foo // binary
    // $ cargo new --lib foo // library

    // foo
    // ├── Cargo.toml // config file for cargo
    // └── src
    //     └── main.rs

    // cargo supports dependencies like below
    // [package]
    // name = "foo"
    // version = "0.1.0"
    // authors = ["mark"]

    // [dependencies]
    // clap = "2.27.1" # from crates.io
    // rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
    // bar = { path = "../bar" } # from a path in the local filesystem
}
