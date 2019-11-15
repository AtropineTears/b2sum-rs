use blake2_rfc::blake2b::Blake2b;
use filebuffer::FileBuffer;
use std::path::Path;

/// ## Blake2b File Hash Constructor
/// 
/// This is the official constructor used to call the new() function with the parameter of the intended digest size.
/// 
/// ## Example
/// 
/// ```
/// use b2sum::Blake2bSum;
/// 
/// fn main(){
///     // Creates a new File Instance
///     let context = Blake2bSum::new(64);
///     
///     // Outputs a Hexadecimal String
///     let hash = context.read("example_file.txt");
/// 
///     // Converts the hexadecimal string to a vector of bytes
///     let _bytes = Blake2bSum::to_bytes(&hash);
/// 
///     // Prints The Hexadecimal Representation
///     println!("Hash: {}",hash);
/// 
///     // Asserts That These Are Equal
///     assert_eq!(hash,"33B20D15383F97EB46D4FA69442596170CCA01008963A7D0E47210C33AEEF991C78323850C012550C227954A40B3D7AD612568ABC73DB9233FAB9EA4F002B0CB");
/// }
/// 
/// ```
#[derive(Debug)]
pub struct Blake2bSum {
    digest_size: usize,
}

impl Blake2bSum {
    pub fn new(digest: usize) -> Self {
        if digest > 0 && digest <= 64 {
            return Blake2bSum {
                digest_size: digest,
            }
        }
        else {
            panic!("Digest Size is either too large or too small")
        }
    }
    /// ## Hash File
    /// This is a function that hashes a file using **Blake2b** and returns the **Hexadecimal Representation** of it as a String. It takes as input any reference to Path.
    /// 
    /// It should be noted that changes to the file during hashing, such as truncating the file can cause problems.
    pub fn read<T: AsRef<Path>>(&self, path: T) -> String {

        // Opens File Using File Buffer
        let fbuffer = FileBuffer::open(path).expect("failed to open file");
        
        // Sets Blake2b Context at the given digest size
        let mut context = Blake2b::new(self.digest_size);
        context.update(&fbuffer);
        let hash = context.finalize();
        
        // Return as Hexadecimal Encoded String
        return hex::encode_upper(hash.as_bytes());
    }
    pub fn to_bytes(s: &str) -> Vec<u8> {
        return hex::decode(s).unwrap()
    }
}