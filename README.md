# b2sum

[![Crates.io](https://img.shields.io/crates/v/b2sum-rs?style=flat-square)](https://crates.io/crates/b2sum-rs)

A **Pure Rust Library** For Hashing Files using Blake2b with any given digest size and includes hashing with a key.

It takes advantage of the crates [filebuffer](https://github.com/ruuda/filebuffer) and [Blake2-rfc](https://crates.io/crates/blake2-rfc), with filebuffer providing more speed than the `std::io` primitive.

## Example

Make sure to add `b2sum-rs` to your cargo.toml

```rust
use b2sum_rs::Blake2bSum;

fn main(){
    // Creates a new File Instance with a digest size of 64 bytes
    let context = Blake2bSum::new(64);

    // Outputs a Hexadecimal String
    let hash = context.read("example_file.txt");

    // Converts the hexadecimal string to a vector of bytes
    let _bytes = Blake2bSum::as_bytes(&hash);

    // Prints The Hexadecimal Representation
    println!("Hash: {}",hash);
}
```

## Note

This crates name is not `b2sum`. That crate is a command-line hashing tool. This crate is a library that provides API for implementing hashing of files into other projects. This crates name is `b2sum-rs`.

## License

* MIT

* Apache-2.0
