use blake2_rfc::blake2b::Blake2b;
use filebuffer::FileBuffer;
use std::path::Path;

// For Reading Files without use FileBuffer
use std::fs;

// For Developer:
// * All outputs are in upper hexadecimal
// * You can use `as_bytes()` to convert from hexadecimal string to bytes
// * Blake2b digest size is between 1 and 64 bytes and will always be returned in hexadecimal format as a `String`
// * One function `read_using_fs()` uses the standard library as opposed to filebuffer to read files.

/// ## Blake2b File Hash Constructor
/// 
/// This is the official constructor used to call the new() function with the parameter of the intended digest size.
/// 
/// ## Example
/// 
/// ```no_run
/// use b2sum_rust::Blake2bSum;
/// 
/// fn main() {
///     // Creates a new File Instance
///     let context = Blake2bSum::new(64);
///     
///     // Outputs a Hexadecimal String
///     let hash = context.read("example_file.txt");
/// 
///     // Converts the hexadecimal string to a vector of bytes
///     let bytes = Blake2bSum::as_bytes(&hash);
/// 
///     // Prints The Hexadecimal Representation
///     println!("Hash: {}",hash);
/// 
///     // Asserts That These Are Equal
///     assert_eq!(hash,"33B20D15383F97EB46D4FA69442596170CCA01008963A7D0E47210C33AEEF991C78323850C012550C227954A40B3D7AD612568ABC73DB9233FAB9EA4F002B0CB");
/// }
/// 
/// ```
/// 
/// All outputs are in **UPPER Hexadecimal** and between 1 and 64 bytes.
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
            panic!("Digest Size is either too large or too small. It should be 1-64.")
        }
    }
    /// ## Hash File
    /// This is a function that hashes a file using **Blake2b** and returns the **Hexadecimal Representation** of it as a **String**. It takes as input any reference to Path.
    /// 
    /// It should be noted that changes to the file during hashing, such as truncating the file may cause problems.
    /// 
    /// ### About Filebuffer
    /// 
    /// > Filebuffer can map files into memory. This is often faster than using the primitives in std::io, and also more convenient. Furthermore this crate offers prefetching and checking whether file data is resident in physical memory (so access will not incur a page fault). This enables non-blocking file reading.
    pub fn read<T: AsRef<Path>>(&self, path: T) -> String {

        // Opens File Using File Buffer
        let fbuffer = FileBuffer::open(path).expect("failed to open file");
        
        // Sets Blake2b Context at the given digest size
        let mut context = Blake2b::new(self.digest_size);
        context.update(&fbuffer);
        let hash = context.finalize();
        
        // Return as Upper Hexadecimal Encoded String
        return hex::encode_upper(hash.as_bytes());
    }
    /// ## Hash File (Using Key)
    /// This is a function that hashes a file (using a key) with **Blake2b** and then returns the **Hexadecimal Representation** of it as a **String**. It takes as input any reference to Path.
    pub fn read_with_key<T: AsRef<Path>>(&self, path: T, key: &[u8]) -> String {

        // Opens File Using File Buffer
        let fbuffer = FileBuffer::open(path).expect("failed to open file");
        
        // Sets Blake2b Context at the given digest size and hashes with the provided key
        let mut context = Blake2b::with_key(self.digest_size, key);
        context.update(&fbuffer);
        let hash = context.finalize();
        
        // Return as Upper Hexadecimal Encoded String
        return hex::encode_upper(hash.as_bytes());
    }
    /// ## Hash File (using standard library)
    /// **Note: `read()` or `read_with_key()` should be used as opposed to this function.**
    /// 
    /// This is a function that hashes a file using **Blake2b** and returns the **Hexadecimal Representation** of it as a **String**. It takes as input any reference to Path.
    /// 
    /// This does not use `filebuffer` and instead uses the standard library. Filebuffer is much faster.
    #[deprecated(
        note = "Please use the `read()` function instead which uses Filebuffer. This function uses the standard library."
    )]
    pub fn read_using_fs<T: AsRef<Path>>(&self, path: T) -> String {

        // Opens File Using Standard Library (fs) and read file to string
        let fbuffer = fs::read_to_string(path).expect("failed to open file");

        
        // Sets Blake2b Context at the given digest size
        let mut context = Blake2b::new(self.digest_size);
        // Convert str to bytes and updated context
        context.update(&fbuffer.as_bytes());
        let hash = context.finalize();
        
        // Return as Upper Hexadecimal Encoded String
        return hex::encode_upper(hash.as_bytes());
    }
    /// # Read String
    /// This function will allow you to take a `String` or `str`, convert it to bytes, then hash it.
    pub fn read_str<T: AsRef<str>>(&self, string: T) -> String {
        
        // Sets Blake2b Context at the given digest size
        let mut context = Blake2b::new(self.digest_size);
        // Convert str to bytes
        context.update(string.as_ref().as_bytes());
        let hash = context.finalize();

        return hex::encode_upper(hash.as_bytes())
    }
    /// # Read Bytes
    /// This function will allow you to **read bytes** and then **hash the bytes** given the digest size.
    pub fn read_bytes(&self, bytes: &[u8]) -> String {
        
        // Sets Blake2b Context at the given digest size
        let mut context = Blake2b::new(self.digest_size);
        context.update(bytes);
        let hash = context.finalize();

        // Return encoded in upper hexadecimal
        return hex::encode_upper(hash.as_bytes())
    }
    /// ## as_bytes()
    /// `as_bytes()` converts from a **Hexadecimal String** to a **Vector of Bytes**
    pub fn as_bytes(s: &str) -> Vec<u8> {
        return hex::decode(s).unwrap()
    }
    /// ## Return Digest Size
    /// This method will return the provided digest size that the struct contains. It should be between 1 and 64 of type `usize`.
    pub fn return_digest_size(&self) -> usize {
        return self.digest_size
    }
}