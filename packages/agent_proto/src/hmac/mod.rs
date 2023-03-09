use std::io;

use bytes::{Buf, BufMut};
use hmac::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};

use crate::codec::{Decode, Encode};

pub(crate) mod signer;

#[derive(Clone, Default)]
pub struct HmacSign<D: Digest>(pub(crate) GenericArray<u8, D::OutputSize>);

impl<D: Digest> HmacSign<D> {
    // Not stable
    //pub const fn size() -> usize {
    //    const SHA224: TypeId = TypeId::of::<Sha224>();
    //    const SHA265: TypeId = TypeId::of::<Sha256>();
    //    const SHA384: TypeId = TypeId::of::<Sha384>();
    //    const SHA512: TypeId = TypeId::of::<Sha512>();
    //
    //    match TypeId::of::<D>() {
    //        SHA224 => 28,
    //        SHA265 => 32,
    //        SHA384 => 48,
    //        SHA512 => 64,
    //        _ => unimplemented!(),
    //    }
    //}

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

impl Encode for HmacSign<Sha256> {
    fn encode<B: BufMut>(self, buf: &mut B) -> io::Result<()> {
        crate::codec::ensure!(buf.remaining_mut() >= Self::SIZE);
        let bytes = self.as_slice();
        buf.put_slice(bytes);
        Ok(())
    }
}

impl Decode for HmacSign<Sha256> {
    fn check<B: AsRef<[u8]>>(buf: &mut io::Cursor<&B>) -> io::Result<()> {
        crate::codec::checked_advance!(buf.remaining() >= Self::SIZE);
        Ok(())
    }

    fn decode<B: Buf>(buf: &mut B) -> Self {
        let bytes = buf.copy_to_bytes(Self::SIZE);
        let bytes = GenericArray::<u8, _>::clone_from_slice(&bytes);
        Self(bytes)
    }
}
