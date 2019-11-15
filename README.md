# b2sum

A **Pure Rust Library** For Hashing Files using Blake2b with any given digest size.

It takes advantage of the crates [filebuffer](https://github.com/ruuda/filebuffer) and [Blake2-rfc](https://crates.io/crates/blake2-rfc), with filebuffer providing more speed than the `std::io` primitive.

## Example

```rust
use b2sum::Blake2bSum;

fn main(){
    // Creates a new File Instance with a digest size of 64 bytes
    let context = Blake2bSum::new(64);

    // Outputs a Hexadecimal String
    let hash = context.read("example_file.txt");

    // Converts the hexadecimal string to a vector of bytes
    let _bytes = Blake2bSum::to_bytes(&hash);

    // Prints The Hexadecimal Representation
    println!("Hash: {}",hash);
}
```

## License

* MIT

* Apache-2.0
