use crate::{
    MbrId, MbrPartId,
    error::{MbrIdError, MbrPartIdError},
    parser::{parse_mbrid, parse_mbrpartid},
    std::{fmt, str::FromStr},
};

#[cfg(feature = "alloc")]
use crate::alloc::string::{String, ToString};

#[inline]
const fn nibble_to_hex(nibble: u8, alpha_offset: u8) -> u8 {
    nibble + b'0' + if nibble > 9 { alpha_offset } else { 0 }
}

impl fmt::Display for MbrId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self, f)
    }
}

impl fmt::LowerHex for MbrId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.encode_lower(&mut [0; Self::LENGTH]))
    }
}

impl fmt::UpperHex for MbrId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.encode_upper(&mut [0; Self::LENGTH]))
    }
}

impl fmt::Debug for MbrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self, f)
    }
}

#[cfg(feature = "alloc")]
impl From<MbrId> for String {
    fn from(v: MbrId) -> Self {
        // Provided by fmt::Display
        v.to_string()
    }
}

impl FromStr for MbrId {
    type Err = MbrIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_mbrid(s.as_bytes())
            .map(MbrId)
            .map_err(|invalid| invalid.into_err())
    }
}

#[inline]
const fn format_mbr_id(src: &[u8; MbrId::BYTE_LENGTH], upper: bool) -> [u8; MbrId::LENGTH] {
    let alpha_offset = if upper { 0x07 } else { 0x27 };
    let mut dst = [0; MbrId::LENGTH];
    let mut i = 0;
    while i < MbrId::BYTE_LENGTH {
        let x = src[i];
        dst[i * 2] = nibble_to_hex(x >> 4, alpha_offset);
        dst[i * 2 + 1] = nibble_to_hex(x & 0x0f, alpha_offset);
        i += 1;
    }
    dst
}

impl MbrId {
    #[inline]
    pub const fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode(&self.0, buffer, false)
    }

    #[inline]
    pub const fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode(&self.0, buffer, true)
    }

    #[inline]
    const fn _encode<'b>(src: &[u8; 4], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
        let buf = match buffer.first_chunk_mut::<{ Self::LENGTH }>() {
            Some(b) => b,
            None => panic!("Buffer too small to encode a MbrId"),
        };
        *buf = format_mbr_id(src, upper);

        // SAFETY: The encoded buffer is ASCII encoded
        unsafe { str::from_utf8_unchecked_mut(buf) }
    }
}

impl fmt::Display for MbrPartId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self, f)
    }
}

impl fmt::LowerHex for MbrPartId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.encode_lower(&mut [0; Self::LENGTH]))
    }
}

impl fmt::UpperHex for MbrPartId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.encode_upper(&mut [0; Self::LENGTH]))
    }
}

impl fmt::Debug for MbrPartId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self, f)
    }
}

#[cfg(feature = "alloc")]
impl From<MbrPartId> for String {
    fn from(v: MbrPartId) -> Self {
        // Provided by fmt::Display
        v.to_string()
    }
}

impl FromStr for MbrPartId {
    type Err = MbrPartIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_mbrpartid(s.as_bytes())
            .map(MbrPartId)
            .map_err(|invalid| invalid.into_err())
    }
}

#[inline]
const fn format_mbrpartid(src: &[u8; 5], upper: bool) -> [u8; MbrPartId::LENGTH] {
    let hex = {
        let alpha_offset = if upper { 0x07 } else { 0x27 };
        let mut dst = [0; (MbrPartId::LENGTH - 1)]; // Size of MbrPartId WITHOUT hyphen.
        let mut i = 0;
        while i < MbrPartId::BYTE_LENGTH {
            let x = src[i];
            dst[i * 2] = nibble_to_hex(x >> 4, alpha_offset);
            dst[i * 2 + 1] = nibble_to_hex(x & 0x0f, alpha_offset);
            i += 1;
        }
        dst
    };

    let mut dst = [0; MbrPartId::LENGTH];

    let mut i = 0;
    while i < 8 {
        dst[i] = hex[i];
        i += 1;
    }
    dst[8] = b'-';
    while i < 10 {
        dst[i + 1] = hex[i];
        i += 1;
    }
    dst
}

impl MbrPartId {
    #[inline]
    pub const fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode(&self.0, buffer, false)
    }

    #[inline]
    pub const fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        Self::_encode(&self.0, buffer, true)
    }

    #[inline]
    const fn _encode<'b>(src: &[u8; 5], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
        let buf = match buffer.first_chunk_mut::<{ Self::LENGTH }>() {
            Some(b) => b,
            None => panic!("Buffer too small to encode a MbrPartId"),
        };
        *buf = format_mbrpartid(src, upper);

        // SAFETY: The encoded buffer is ASCII encoded
        unsafe { str::from_utf8_unchecked_mut(buf) }
    }
}
