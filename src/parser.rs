use crate::{
    MbrId, MbrPartId,
    error::{InvalidMbrId, InvalidMbrPartId, MbrIdError, MbrPartIdError},
};

#[inline]
pub const fn decode_hex<const HEX_SZ: usize, const BUF_SZ: usize>(
    hex: &[u8; HEX_SZ],
) -> Option<[u8; BUF_SZ]> {
    let mut nibbles = [0u8; HEX_SZ];
    let mut bad = 0u8;

    let mut i = 0;
    while i < HEX_SZ {
        let c = hex[i];

        // '0'..='9' map to 0..=9; 'a'..='f' and 'A'..='F' (via `| 0x20`) map to
        // 0..=5, offset by 10 to land in 10..=15.
        let digit = c.wrapping_sub(b'0');
        let alpha = (c | 0x20).wrapping_sub(b'a');

        let is_digit = digit < 10;
        let is_alpha = alpha < 6;

        nibbles[i] = if is_digit {
            digit
        } else {
            alpha.wrapping_add(10)
        };

        bad |= if is_digit | is_alpha { 0 } else { 1 };

        i += 1;
    }

    if bad != 0 {
        return None;
    }

    let mut buf = [0u8; BUF_SZ];
    let mut j = 0;
    while j < BUF_SZ {
        buf[j] = (nibbles[j * 2] << 4) | nibbles[j * 2 + 1];
        j += 1;
    }

    Some(buf)
}

impl MbrId {
    pub fn parse(input: &str) -> Result<Self, MbrIdError> {
        Self::try_parse_ascii(input.as_bytes()).map_err(InvalidMbrId::into_err)
    }

    pub const fn try_parse<'a>(input: &'a str) -> Result<Self, InvalidMbrId<'a>> {
        Self::try_parse_ascii(input.as_bytes())
    }

    pub const fn try_parse_ascii<'a>(s: &'a [u8]) -> Result<Self, InvalidMbrId<'a>> {
        match parse_mbrid(s) {
            Ok(bytes) => Ok(MbrId::from_bytes(bytes)),
            Err(e) => Err(e),
        }
    }
}

#[inline]
pub(crate) const fn parse_mbrid(s: &'_ [u8]) -> Result<[u8; MbrId::BYTE_LENGTH], InvalidMbrId<'_>> {
    let hex = if s.len() == MbrId::LENGTH {
        s.first_chunk::<{ MbrId::LENGTH }>().unwrap()
    } else {
        return Err(InvalidMbrId(s));
    };

    match decode_hex::<{ MbrId::LENGTH }, { MbrId::BYTE_LENGTH }>(hex) {
        Some(buf) => Ok(buf),
        None => Err(InvalidMbrId(s)),
    }
}

impl MbrPartId {
    pub fn parse(input: &str) -> Result<Self, MbrPartIdError> {
        Self::try_parse_ascii(input.as_bytes()).map_err(InvalidMbrPartId::into_err)
    }

    pub const fn try_parse<'a>(input: &'a str) -> Result<Self, InvalidMbrPartId<'a>> {
        Self::try_parse_ascii(input.as_bytes())
    }

    pub const fn try_parse_ascii<'a>(s: &'a [u8]) -> Result<Self, InvalidMbrPartId<'a>> {
        match parse_mbrpartid(s) {
            Ok(bytes) => Ok(MbrPartId::from_bytes(bytes)),
            Err(e) => Err(e),
        }
    }
}

#[inline]
pub(crate) const fn parse_mbrpartid(s: &'_ [u8]) -> Result<[u8; 5], InvalidMbrPartId<'_>> {
    if s.len() != MbrPartId::LENGTH {
        return Err(InvalidMbrPartId(s));
    }

    match [s[8]] {
        [b'-'] => {}
        _ => return Err(InvalidMbrPartId(s)),
    }

    let mut hex = [0u8; (MbrPartId::LENGTH - 1)]; // Size of MbrPartId WITHOUT hyphen.
    let mut i = 0;
    while i < 8 {
        hex[i] = s[i];
        i += 1;
    }
    while i < 10 {
        hex[i] = s[i + 1];
        i += 1;
    }

    match decode_hex::<{ MbrPartId::LENGTH - 1 }, { MbrPartId::BYTE_LENGTH }>(&hex) {
        Some(buf) => Ok(buf),
        None => Err(InvalidMbrPartId(s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{MbrIdError, MbrPartIdError};

    #[test]
    fn test_parse_mbrid_valid() {
        let known = MbrId::from_bytes([0x6d, 0xdc, 0xf6, 0xda]);
        let parsed = MbrId::try_parse("6ddcf6da").unwrap();

        assert_eq!(known, parsed);

        assert!(MbrId::try_parse("00000000").is_ok());
        assert!(MbrId::try_parse("6ddcf6da").is_ok());
        assert!(MbrId::try_parse("6DDCF6DA").is_ok());
    }

    #[test]
    fn test_parse_mbrid_invalid() {
        assert_eq!(MbrId::parse(""), Err(MbrIdError::InvalidLength));
        assert_eq!(MbrId::parse("!"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("F916-4fa"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("QABC-1234"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("F9168C5X"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("{F9168C5"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("67e5"), Err(MbrIdError::InvalidLength));
        assert_eq!(MbrId::parse("123456ABC"), Err(MbrIdError::InvalidLength));
        assert_eq!(MbrId::parse("67e55abg"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("67e5%2fb"), Err(MbrIdError::InvalidChar));
        assert_eq!(
            MbrId::parse("231231212212423424324323477343246663"),
            Err(MbrIdError::InvalidLength)
        );
        assert_eq!(
            MbrId::parse("67e5504410b1426f9247bb680e5fe0c"),
            Err(MbrIdError::InvalidLength)
        );
        assert_eq!(MbrId::parse("67e550Xb"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("F9-16BACE"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("F916-BACEE"), Err(MbrIdError::InvalidChar));
        assert_eq!(MbrId::parse("\u{bcf3c}"), Err(MbrIdError::InvalidChar));
    }

    #[test]
    fn test_parse_mbrpartid_valid() {
        let known = MbrPartId::from_bytes([0x6d, 0xdc, 0xf6, 0xda, 0x01]);
        let parsed = match MbrPartId::try_parse("6ddcf6da-01") {
            Ok(t) => t,
            Err(e) => panic!("{}", e.into_err()),
        };

        assert_eq!(known, parsed);

        assert!(MbrPartId::try_parse("00000000-00").is_ok());
        assert!(MbrPartId::try_parse("6ddaf6da-02").is_ok());
        assert!(MbrPartId::try_parse("6DDCF6DB-04").is_ok());
    }

    #[test]
    fn test_parse_mbrpartid_invalid() {
        assert_eq!(MbrPartId::parse(""), Err(MbrPartIdError::InvaildGroupCount));
        assert_eq!(MbrPartId::parse("!"), Err(MbrPartIdError::InvalidChar));
        assert_eq!(
            MbrPartId::parse("F916-4fa"),
            Err(MbrPartIdError::InvalidGroupLength)
        );
        assert_eq!(
            MbrPartId::parse("QABC-1234"),
            Err(MbrPartIdError::InvalidChar)
        );
        assert_eq!(
            MbrPartId::parse("F9168C5X"),
            Err(MbrPartIdError::InvalidChar)
        );
        assert_eq!(
            MbrPartId::parse("{F9168C5"),
            Err(MbrPartIdError::InvalidChar)
        );
        assert_eq!(
            MbrPartId::parse("67e5"),
            Err(MbrPartIdError::InvaildGroupCount)
        );
        assert_eq!(
            MbrPartId::parse("123456ABC"),
            Err(MbrPartIdError::InvaildGroupCount)
        );
        assert_eq!(
            MbrPartId::parse("67e55abg"),
            Err(MbrPartIdError::InvalidChar)
        );
        assert_eq!(
            MbrPartId::parse("67e5%2fb"),
            Err(MbrPartIdError::InvalidChar)
        );
        assert_eq!(
            MbrPartId::parse("231231212212423424324323477343246663"),
            Err(MbrPartIdError::InvaildGroupCount)
        );
        assert_eq!(
            MbrPartId::parse("67e5504410b1426f9247bb680e5fe0c"),
            Err(MbrPartIdError::InvaildGroupCount)
        );
        assert_eq!(
            MbrPartId::parse("67e550Xb"),
            Err(MbrPartIdError::InvalidChar)
        );
        assert_eq!(
            MbrPartId::parse("F9-16BACE"),
            Err(MbrPartIdError::InvalidGroupLength)
        );
        assert_eq!(
            MbrPartId::parse("F916-BACEE"),
            Err(MbrPartIdError::InvalidGroupLength)
        );
        assert_eq!(
            MbrPartId::parse("\u{bcf3c}"),
            Err(MbrPartIdError::InvalidChar)
        );
    }
}
