//! Hash functions (RustCrypto `sha2`).

use sha2::{Digest, Sha256};

/// SHA-256 digest of `data`.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// SHA-256 digest of `data` as 64 lowercase hex characters.
pub fn sha256_hex(data: &[u8]) -> String {
    crate::to_hex(&sha256(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test vectors from NIST FIPS 180-2 / the NIST CAVP SHA-256 examples.
    #[test]
    fn sha256_empty_string() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sha256_abc() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn sha256_two_block_message() {
        assert_eq!(
            sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }
}
