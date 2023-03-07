use hmac::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};

pub(crate) mod signer;

#[derive(Clone, Default)]
pub struct HmacSign<D: Digest>(pub(crate) GenericArray<u8, D::OutputSize>);

impl<D: Digest> HmacSign<D> {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

// impl HmacSign<Sha224> {
//     pub const SIZE: usize = 28;
// }

impl HmacSign<Sha256> {
    pub const SIZE: usize = 32;
}

// impl HmacSign<Sha384> {
//     pub const SIZE: usize = 48;
// }

// impl HmacSign<Sha512> {
//     pub const SIZE: usize = 48;
// }

impl<D: Digest> PartialEq for HmacSign<D> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<D: Digest> Eq for HmacSign<D> {}

impl<D: Digest> std::fmt::Debug for HmacSign<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HmacSign").field(&self.0).finish()
    }
}
