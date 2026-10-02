// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
// Copyright 2026 rysndavjd.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! TODO!

#![no_std]
//#![deny(missing_debug_implementations, missing_docs)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

mod error;
mod fmt;
mod parser;
mod serde;

pub use crate::error::{MbrIdError, MbrPartIdError};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MbrId([u8; 4]);

impl MbrId {
    pub const LENGTH: usize = 8;
    pub const BYTE_LENGTH: usize = 4;

    pub const fn nil() -> Self {
        MbrId([u8::MIN; 4])
    }

    pub const fn max() -> Self {
        MbrId([u8::MAX; 4])
    }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 4]) -> MbrId {
        MbrId(bytes)
    }

    #[inline]
    pub const fn from_bytes_be(b: [u8; 4]) -> MbrId {
        MbrId([b[3], b[2], b[1], b[0]])
    }

    pub const fn from_slice(b: &[u8]) -> Result<Self, MbrIdError> {
        if b.len() != 4 {
            return Err(MbrIdError::InvalidLength);
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        Ok(MbrId::from_bytes(bytes))
    }

    pub const fn from_slice_be(b: &[u8]) -> Result<Self, MbrIdError> {
        if b.len() != 4 {
            return Err(MbrIdError::InvalidLength);
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(b);

        Ok(MbrId::from_bytes_be(bytes))
    }

    pub const fn from_u32(v: u32) -> Self {
        MbrId::from_bytes(v.to_le_bytes())
    }

    pub const fn from_u32_be(v: u32) -> Self {
        MbrId::from_bytes(v.to_be_bytes())
    }

    pub const fn as_u32(&self) -> u32 {
        u32::from_le_bytes(*self.as_bytes())
    }

    pub const fn as_u32_be(&self) -> u32 {
        u32::from_be_bytes(*self.as_bytes())
    }

    pub const fn from_u16_pair(hi: u16, lo: u16) -> Self {
        MbrId::from_u32(((hi as u32) << 16) | lo as u32)
    }

    pub const fn from_u16_pair_be(hi: u16, lo: u16) -> Self {
        MbrId::from_u32_be(((hi as u32) << 16) | lo as u32)
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    #[inline]
    pub const fn into_bytes(self) -> [u8; 4] {
        self.0
    }

    #[inline]
    pub const fn to_bytes_be(self) -> [u8; 4] {
        [self.0[3], self.0[2], self.0[1], self.0[0]]
    }

    pub const fn is_nil(&self) -> bool {
        self.as_u32() == u32::MIN
    }

    pub const fn is_max(&self) -> bool {
        self.as_u32() == u32::MAX
    }
}

impl crate::std::hash::Hash for MbrId {
    fn hash<H: crate::std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

impl AsRef<MbrId> for MbrId {
    #[inline]
    fn as_ref(&self) -> &MbrId {
        self
    }
}

impl AsRef<[u8]> for MbrId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for MbrId {
    type Error = MbrIdError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        MbrId::from_slice(value)
    }
}

#[cfg(feature = "alloc")]
impl From<MbrId> for crate::alloc::vec::Vec<u8> {
    fn from(value: MbrId) -> Self {
        value.0.to_vec()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::alloc::vec::Vec<u8>> for MbrId {
    type Error = MbrIdError;

    fn try_from(value: crate::alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        MbrId::from_slice(&value)
    }
}

impl Default for MbrId {
    fn default() -> Self {
        Self::nil()
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct MbrPartId([u8; 5]);

impl MbrPartId {
    pub const LENGTH: usize = 11;
    pub const BYTE_LENGTH: usize = 5;

    pub const fn nil() -> Self {
        Self([u8::MIN; 5])
    }

    pub const fn max() -> Self {
        Self([u8::MAX; 5])
    }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 5]) -> MbrPartId {
        Self(bytes)
    }

    #[inline]
    pub const fn from_bytes_be(bytes: [u8; 5]) -> MbrPartId {
        Self([bytes[4], bytes[3], bytes[2], bytes[1], bytes[0]])
    }

    pub const fn from_slice(b: &[u8]) -> Result<Self, MbrPartIdError> {
        if b.len() != 5 {
            return Err(MbrPartIdError::InvalidLength);
        }

        let mut bytes = [0u8; 5];
        bytes.copy_from_slice(b);

        Ok(Self::from_bytes(bytes))
    }

    pub const fn from_slice_be(b: &[u8]) -> Result<Self, MbrPartIdError> {
        if b.len() != 5 {
            return Err(MbrPartIdError::InvalidLength);
        }

        let mut bytes = [0u8; 5];
        bytes.copy_from_slice(b);

        Ok(Self::from_bytes_be(bytes))
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8; 5] {
        &self.0
    }

    #[inline]
    pub const fn into_bytes(self) -> [u8; 5] {
        self.0
    }

    #[inline]
    pub const fn to_bytes_be(self) -> [u8; 5] {
        [self.0[4], self.0[3], self.0[2], self.0[1], self.0[0]]
    }

    pub const fn from_parts(id: &MbrId, part_no: u8) -> MbrPartId {
        let i = id.as_bytes();
        MbrPartId::from_bytes([i[0], i[1], i[2], i[3], part_no])
    }

    pub const fn to_mbrid(&self) -> MbrId {
        MbrId::from_bytes([self.0[0], self.0[1], self.0[2], self.0[3]])
    }

    pub const fn is_nil(&self) -> bool {
        u32::from_le_bytes([self.0[0], self.0[1], self.0[2], self.0[3]]) == u32::MIN
            && self.0[0] == u8::MIN
    }

    pub const fn is_max(&self) -> bool {
        u32::from_le_bytes([self.0[1], self.0[2], self.0[3], self.0[4]]) == u32::MAX
            && self.0[0] == u8::MAX
    }
}

impl crate::std::hash::Hash for MbrPartId {
    fn hash<H: crate::std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0);
    }
}

impl AsRef<MbrPartId> for MbrPartId {
    #[inline]
    fn as_ref(&self) -> &MbrPartId {
        self
    }
}

impl AsRef<[u8]> for MbrPartId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for MbrPartId {
    type Error = MbrPartIdError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        MbrPartId::from_slice(value)
    }
}

#[cfg(feature = "alloc")]
impl From<MbrPartId> for crate::alloc::vec::Vec<u8> {
    fn from(value: MbrPartId) -> Self {
        value.0.to_vec()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::alloc::vec::Vec<u8>> for MbrPartId {
    type Error = MbrPartIdError;

    fn try_from(value: crate::alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        MbrPartId::from_slice(&value)
    }
}

impl Default for MbrPartId {
    fn default() -> Self {
        Self::nil()
    }
}
