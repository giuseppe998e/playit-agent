use hmac::digest::generic_array::GenericArray;
use sha2::Digest;

pub(crate) mod signer;

// pub(crate) const SHA224_BYTES: usize = 28;
pub(crate) const SHA256_BYTES: usize = 32;
// pub(crate) const SHA384_BYTES: usize = 48;
// pub(crate) const SHA512_BYTES: usize = 64;

#[derive(Clone, Default)]
pub struct HmacSign<D: Digest>(pub(super) GenericArray<u8, D::OutputSize>);

impl<D: Digest> HmacSign<D> {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

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
