use blake2_rfc::blake2b::Blake2b;
use filebuffer::FileBuffer;
use std::path::Path;

/// ## Blake2b File Hash Constructor
/// 
/// This is the official constructor used to call the new() function which returns a 64 byte blake2b digest of a file.
/// 
/// ## Example
/// 
/// ```
/// use b2sum::Blake2bSum;
/// 
/// // Passes in the digest length in bytes and returns the struct for reading the file.
/// let b2 = Blake2bSum::new(64);
/// 
/// // Reads the given file with the provided digest size
/// let hex = b2.read("test.txt");
/// 
/// // Prints Hexadecimal Output
/// println!("{}",hex_output);
/// 
/// ```
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
    pub fn to_bytes(s: String) -> Vec<u8> {
        return hex::decode(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn b2sum() {
        assert_eq!(2 + 2, 4);
    }
}
