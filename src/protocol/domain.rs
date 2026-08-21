use std::ops::Deref;

use arrayvec::ArrayVec;

/// Maximum length
pub const MAX_NAME_LENGTH: usize = 255;
pub const MAX_LABEL_LENGTH: usize = 63;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainName {
    octets: ArrayVec<u8, MAX_NAME_LENGTH>,
}

impl DomainName {
    pub fn new(octets: ArrayVec<u8, MAX_NAME_LENGTH>) -> Self {
        Self { octets }
    }
}

impl Deref for DomainName {
    type Target = ArrayVec<u8, MAX_NAME_LENGTH>;

    fn deref(&self) -> &Self::Target {
        &self.octets
    }
}
