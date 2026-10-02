use crate::std::{fmt, str::from_utf8};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MbrIdError {
    InvalidChar,
    InvalidLength,
    InvalidUTF8,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidMbrId<'a>(pub(crate) &'a [u8]);

impl<'a> InvalidMbrId<'a> {
    /// Converts the lightweight error type into detailed diagnostics.
    pub fn into_err(self) -> MbrIdError {
        // Check whether or not the input was ever actually a valid UTF8 string
        let input_str = match from_utf8(self.0) {
            Ok(s) => s,
            Err(_) => return MbrIdError::InvalidUTF8,
        };

        for character in input_str.chars() {
            if !character.is_ascii_hexdigit() {
                return MbrIdError::InvalidChar;
            }
        }

        // This means that we tried and failed to parse a MbrId.
        // Since we verified that all the characters are valid, this means
        // that it MUST have an invalid length.
        MbrIdError::InvalidLength
    }
}

impl fmt::Display for MbrIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MbrIdError::InvalidChar => {
                write!(f, "invalid character: expected [0-9a-fA-F]",)
            }
            MbrIdError::InvalidLength => {
                write!(f, "invalid length: expected length for MbrId")
            }
            MbrIdError::InvalidUTF8 => {
                write!(f, "non-UTF8 input")
            }
        }
    }
}

impl crate::std::error::Error for MbrIdError {}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MbrPartIdError {
    InvalidChar,
    InvalidLength,
    InvalidUTF8,
    InvaildGroupCount,
    InvalidGroupLength,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidMbrPartId<'a>(pub(crate) &'a [u8]);

impl<'a> InvalidMbrPartId<'a> {
    /// Converts the lightweight error type into detailed diagnostics.
    pub fn into_err(self) -> MbrPartIdError {
        // Check whether or not the input was ever actually a valid UTF8 string
        let input_str = match from_utf8(self.0) {
            Ok(s) => s,
            Err(_) => return MbrPartIdError::InvalidUTF8,
        };

        let mut hyphen_count = 0;

        for character in input_str.chars() {
            match character {
                character if character.is_ascii_hexdigit() => (),
                '-' => {
                    hyphen_count += 1;
                }
                _ => {
                    return MbrPartIdError::InvalidChar;
                }
            }
        }

        if hyphen_count != 1 {
            MbrPartIdError::InvaildGroupCount
        } else {
            MbrPartIdError::InvalidGroupLength
        }
    }
}

impl fmt::Display for MbrPartIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MbrPartIdError::InvalidChar => {
                write!(f, "invalid character: expected [0-9a-fA-F]",)
            }
            MbrPartIdError::InvalidLength => {
                write!(f, "invalid length: expected length for MbrPartId")
            }
            MbrPartIdError::InvalidUTF8 => {
                write!(f, "non-UTF8 input")
            }
            MbrPartIdError::InvaildGroupCount => {
                write!(f, "invalid group count: expected 2")
            }
            MbrPartIdError::InvalidGroupLength => {
                write!(f, "invalid group length")
            }
        }
    }
}

impl crate::std::error::Error for MbrPartIdError {}
