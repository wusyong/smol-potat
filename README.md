# smol-potat
Proc macro for smol runtime.

![](https://i.redd.it/arnr6d62b9p21.jpg)

This is the proc macro to help you initializing `smol` runtime on your binary, test cases and benchmark.
Usage is similar to what you do in `tokio` and `async-std`:

## Example

```rust
#[smol_potat::main]
async fn main() {
    println!("Hello, world!");
}
```
