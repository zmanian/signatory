//! Ed25519: Schnorr signatures using the twisted Edwards form of Curve25519
//!
//! Described in RFC 8032: <https://tools.ietf.org/html/rfc8032>

use error::Error;
use core::fmt;

/// curve25519-dalek software provider
#[cfg(feature = "dalek")]
pub mod dalek;

/// YubiHSM2 hardware provider
#[cfg(feature = "yubihsm")]
pub mod yubihsm;

/// RFC 8032 Ed25519 test vectors
#[cfg(test)]
pub mod test_vectors;

#[cfg(feature = "dalek")]
pub use self::dalek::DalekSigner;

#[cfg(feature = "yubihsm")]
pub use self::yubihsm::YubihsmSigner;

/// Size of an Ed25519 signature (512-bits)
pub const SIGNATURE_SIZE: usize = 64;

/// Ed25519 signatures
pub struct Signature([u8; SIGNATURE_SIZE]);

impl Signature {
    /// Obtain signature as a byte slice
    pub fn to_bytes(&self) -> &[u8; SIGNATURE_SIZE] {
        &self.0
    }

    /// Return signature as a raw byte slice
    pub fn into_bytes(self) -> [u8; SIGNATURE_SIZE] {
        self.0
    }
}

impl<'a> From<&'a [u8]> for Signature {
    fn from(signature: &[u8]) -> Signature {
        if signature.len() != SIGNATURE_SIZE {
            panic!("signature is incorrect size: {}", signature.len())
        }

        let mut s = [0u8; SIGNATURE_SIZE];
        s.copy_from_slice(signature);
        Signature(s)
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "signatory::ed25519::Signature(")?;

        for byte in self.0.iter() {
            write!(f, "{:x} ", byte)?;
        }

        write!(f, ")")
    }
}

/// Parent trait for Ed25519 signers
/// Signer is an object-safe trait for producing a particular type of signature
pub trait Signer {
    /// Compute an Ed25519 signature for the given message
    fn sign(&self, msg: &[u8]) -> Result<Signature, Error>;
}
