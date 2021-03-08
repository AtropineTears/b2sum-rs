#[cfg(test)]
mod tests {
    use b2sum_rust::*;
    #[test]
    fn blake2b_digest_sizes(){
        // Creates a new File Instance
        let _context_64 = Blake2bSum::new(64);
        
        let _context_48 = Blake2bSum::new(48);

        let _context_32 = Blake2bSum::new(32);

        let _context_24 = Blake2bSum::new(24);

        let _context_16 = Blake2bSum::new(16);

        let _context_8 = Blake2bSum::new(8);

        let _context_1 = Blake2bSum::new(1);
    }
    #[test]
    #[should_panic]
    fn wrong_size_zero(){
        let _context = Blake2bSum::new(0);
    }
    #[test]
    #[should_panic]
    fn wrong_size_65(){
        let _context = Blake2bSum::new(65);
    }
    #[test]
    #[should_panic]
    fn wrong_size_128(){
        let _context = Blake2bSum::new(128);
    }
}