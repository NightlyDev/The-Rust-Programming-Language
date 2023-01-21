## Notes for 1. Getting Started

- compiling with `rustc [filename]`
- main-function similar to other programming languages like C++ and C
- Rust own automatic formatter tool called `rustfmt [filename]` is very helpful
- a function ending with `!` is called *macro*. Will be discussed later in chapter 19.
- `cargo build` compiles a dev build of a cargo built project
- `cargo build --release` compiles the project with optimizations
- `cargo run` compiles and **runs** a dev build of a cargo built project
- `cargo check` checks if the cargo built project is compileable, but doesn't compile it
- `Cargo.toml` keeps dependencies, name, version and Rust edition of the project while `Cargo.lock` keeps the used dependencies of that project, similar to JavaScript