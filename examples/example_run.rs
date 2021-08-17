extern crate b2sum_rs;
use b2sum_rs::Blake2bSum;

fn main(){
    // Creates a new File Instance
    let context = Blake2bSum::new(64);
    
    // Outputs a Hexadecimal String
    let hash = context.read("example_file.txt");

    // Converts the hexadecimal string to a vector of bytes
    let _bytes = Blake2bSum::as_bytes(&hash);

    // Prints The Hexadecimal Representation
    println!("Hash: {}",hash);

    // Asserts That These Are Equal
    assert_eq!(hash,"33B20D15383F97EB46D4FA69442596170CCA01008963A7D0E47210C33AEEF991C78323850C012550C227954A40B3D7AD612568ABC73DB9233FAB9EA4F002B0CB");
}