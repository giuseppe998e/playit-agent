use hmac::{digest::MacError, Hmac, Mac};
use sha2::{Digest, Sha256};

use super::HmacSign;

pub(crate) trait HmacSigner<D: Digest> {
    fn sign_data(&self, data: &[u8]) -> HmacSign<D>;

    fn verify_data(&self, sig: &HmacSign<D>, data: &[u8]) -> Result<(), MacError>;
}

impl HmacSigner<Sha256> for Hmac<Sha256> {
    fn sign_data(&self, data: &[u8]) -> HmacSign<Sha256> {
        let mut mac = self.clone();
        mac.update(data);

        let bytes = mac.finalize().into_bytes();
        HmacSign(bytes)
    }

    fn verify_data(&self, sig: &HmacSign<Sha256>, data: &[u8]) -> Result<(), MacError> {
        let mut mac = self.clone();
        mac.update(data);

        mac.verify(&sig.0)
    }
}
