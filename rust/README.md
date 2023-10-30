# Rust based Raytracer

The same raytracer as in [cpp/README.md](../cpp/README.md), but implemented in `rust`.
The plan is to take this implementation further as I find developing the raytracer in rust to be a much more enjoyable time (compared to cpp).

This was a good reason and project to learn rust and implement something somewhat complicated too.
Also, the fact that the project requires no API, means it is lightweight and easy build. (It also helps that it is quite easy to cross-compile pure rust in multiple platforms)

## Quickstart

Requires:

- `rustup`/`cargo`
- clang (Or any linker)

To build and run:

- `cargo run -r`
  - Runs the `main.rs` file and has the ppm file contents write to `stdout`
  - A better command to run would be `cargo run -r > test.ppm`
- `cargo build -r` and run the binary at `target/release/rust-simple-raytracer(.exe)`

## Development

What I used to make this (Besides `rust`):

- `llvm-cov`
  - For looking at code coverage of tests
- Tarpulin
  - For windows

## Acknowledgements, References and Related Projects

I used quite a bit of references for this project, particularly to see how to implement the same things the `"rust"` way to help me in understand and solidify the harder concepts of rust.
In no particular order:

- [dps rust raytracer](https://github.com/dps/rust-raytracer)
  - First result you see on google
  - [His accompanying blog post](https://blog.singleton.io/posts/2022-01-02-raytracing-with-rust/)
- [Chris Biscardi's Rust Adventure raytracer project](https://github.com/rust-adventure/raytracing-in-one-weekend/tree/f69c57162de6b3e7538bc26f5611f4e1142414b5)
  - [The accompanying youtube video](https://www.youtube.com/watch?v=6D8WVYm1YwY)
    - Very usefully as he talks about implementing the raytracer along with the book, interjecting insights and reasoning for his code in rust (Compared to cpp)
