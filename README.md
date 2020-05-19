# smol-potat

[![][crates-badge]][crates-url] ![][license-badge]

[crates-badge]: https://img.shields.io/crates/v/smol-potat
[crates-url]: https://crates.io/crates/smol-potat
[license-badge]: https://img.shields.io/crates/l/smol-potat

Proc macro for [smol](https://crates.io/crates/smol) runtime.

![](https://i.redd.it/arnr6d62b9p21.jpg)

This is the macro to help you initializing `smol` runtime on your binary, test cases and benchmark.
Usage is similar to what you do in `tokio` and `async-std`.

## Usage

### Single thread

```rust
#[smol_potat::main]
async fn main() {
    println!("Hello, world!");
}
```

### Multi threads

```rust
#[smol_potat::main(threads=3)]
async fn main() {
    println!("Hello, world!");
}
```

### Auto thread generation

Enable `auto` feature and the rest is same as single thread:

```
smol_potat = { version = "0.3.0", features = ["auto"] }
```

```rust
#[smol_potat::main]
async fn main() {
    println!("Hello, world!");
}
```