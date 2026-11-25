//! Ed25519 digital signatures (`ed25519-dalek`).
//!
//! Key generation takes a 32-byte seed from the caller. The caller must get it
//! from a cryptographically secure random source: the OS on the server, or
//! `crypto.getRandomValues` in the browser.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

pub const SEED_LEN: usize = 32;
pub const PUBLIC_KEY_LEN: usize = 32;
pub const SIGNATURE_LEN: usize = 64;

/// An Ed25519 signing key. Its secret bytes are wiped from memory on drop
/// (`ed25519-dalek`'s `zeroize` feature).
pub struct Keypair(SigningKey);

impl Keypair {
    /// Build a key pair from a 32-byte secret seed (RFC 8032 "private key").
    pub fn from_seed(seed: &[u8; SEED_LEN]) -> Self {
        Keypair(SigningKey::from_bytes(seed))
    }

    pub fn public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
        self.0.verifying_key().to_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> [u8; SIGNATURE_LEN] {
        self.0.sign(message).to_bytes()
    }
}

/// `true` only if `signature` is a valid signature of exactly `message` under
/// `public_key`. Malformed keys or signatures verify as `false`.
///
/// Uses `verify_strict`, which also rejects weak public keys and
/// non-canonical signatures.
pub fn verify(
    public_key: &[u8; PUBLIC_KEY_LEN],
    message: &[u8],
    signature: &[u8; SIGNATURE_LEN],
) -> bool {
    let Ok(key) = VerifyingKey::from_bytes(public_key) else {
        return false;
    };
    let signature = Signature::from_bytes(signature);
    key.verify_strict(message, &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_hex;

    fn from_hex<const N: usize>(s: &str) -> [u8; N] {
        let mut out = [0u8; N];
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("valid hex");
        }
        out
    }

    // RFC 8032 §7.1, TEST 1 (empty message).
    const SECRET: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
    const PUBLIC: &str = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
    const SIGNATURE: &str = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b";

    #[test]
    fn rfc8032_test1_public_key_and_signature() {
        let keypair = Keypair::from_seed(&from_hex(SECRET));
        assert_eq!(to_hex(&keypair.public_key()), PUBLIC);
        assert_eq!(to_hex(&keypair.sign(b"")), SIGNATURE);
    }

    #[test]
    fn rfc8032_test1_verifies() {
        assert!(verify(&from_hex(PUBLIC), b"", &from_hex(SIGNATURE)));
    }

    #[test]
    fn tampered_message_fails() {
        let keypair = Keypair::from_seed(&[7u8; SEED_LEN]);
        let signature = keypair.sign(b"Send 5,000 to Ada");
        assert!(verify(
            &keypair.public_key(),
            b"Send 5,000 to Ada",
            &signature
        ));
        assert!(!verify(
            &keypair.public_key(),
            b"Send 9,000 to Ada",
            &signature
        ));
    }

    #[test]
    fn tampered_signature_fails() {
        let keypair = Keypair::from_seed(&[7u8; SEED_LEN]);
        let mut signature = keypair.sign(b"hello");
        signature[0] ^= 1;
        assert!(!verify(&keypair.public_key(), b"hello", &signature));
    }

    #[test]
    fn wrong_key_fails() {
        let alice = Keypair::from_seed(&[1u8; SEED_LEN]);
        let bob = Keypair::from_seed(&[2u8; SEED_LEN]);
        let signature = alice.sign(b"hello");
        assert!(!verify(&bob.public_key(), b"hello", &signature));
    }
}
