//! Finite Labs cryptography wrappers.
//!
//! Every function here is a thin, well-named wrapper around an audited crate
//! (RustCrypto `sha2`, `ed25519-dalek`). We never implement primitives ourselves
//! (PRD §6.4 rule 1). This crate is shared by the server (`crates/web`) and the
//! browser demos (`crates/playground`, compiled to WebAssembly).

#![forbid(unsafe_code)]

pub mod hash;
pub mod sign;

/// Lowercase hex with no `0x` prefix. Encoding, not cryptography.
pub fn to_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(DIGITS[usize::from(b >> 4)] as char);
        out.push(DIGITS[usize::from(b & 0x0f)] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::to_hex;

    #[test]
    fn hex_encodes_lowercase_without_prefix() {
        assert_eq!(to_hex(&[]), "");
        assert_eq!(to_hex(&[0x00, 0x0f, 0xa5, 0xff]), "000fa5ff");
    }
}
