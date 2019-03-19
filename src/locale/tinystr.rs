//! A small ASCII-only bounded length string representation.

use std::fmt;
use std::num::{NonZeroU32, NonZeroU64};
use std::ops::Deref;
use std::ptr::copy_nonoverlapping;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidSize,
    InvalidNull,
    NonAscii,
}

/// A tiny string that is from 1 to 8 non-NUL ASCII characters.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TinyStr8(NonZeroU64);

/// A tiny string that is from 1 to 4 non-NUL ASCII characters.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TinyStr4(NonZeroU32);

impl TinyStr8 {
    /// Create a new tiny string.
    ///
    /// Returns an error result if the string is not 1 to 8 characters in length,
    /// contains non-ASCII, or contains an embedded NUL byte.
    pub fn new(text: &str) -> Result<TinyStr8, Error> {
        let len = text.len();
        if len < 1 || len > 8 {
            return Err(Error::InvalidSize);
        }
        unsafe {
            let mut word: u64 = 0;
            copy_nonoverlapping(text.as_ptr(), &mut word as *mut u64 as *mut u8, len);
            let mask = 0x80808080_80808080u64 >> (8 * (8 - len));
            // TODO: could do this with #cfg(target_endian), but this is clearer and
            // more confidence-inspiring.
            let mask = mask.to_le();
            if (word & mask) != 0 {
                return Err(Error::NonAscii);
            }
            if ((mask - word) & mask) != 0 {
                return Err(Error::InvalidNull);
            }
            Ok(TinyStr8(NonZeroU64::new_unchecked(word)))
        }
    }

    /// Dereference to string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    pub fn to_ascii_uppercase(self) -> TinyStr8 {
        let word = self.0.get();
        let result = word &
        !(
            (
                (word + 0x1f1f1f1f_1f1f1f1f) &
                !(word + 0x05050505_05050505) &
                0x80808080_80808080
            ) >> 2
        );
        unsafe { TinyStr8(NonZeroU64::new_unchecked(result)) }
    }

    pub fn to_ascii_lowercase(self) -> TinyStr8 {
        let word = self.0.get();
        let result = word |
        (
            (
                (word + 0x3f3f3f3f_3f3f3f3f) &
                !(word + 0x25252525_25252525) &
                0x80808080_80808080
            ) >> 2
        );
        unsafe { TinyStr8(NonZeroU64::new_unchecked(result)) }
    }

    /// Determine whether string is all ASCII alphabetical characters.
    pub fn is_all_ascii_alpha(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f7f7f_7f7f7f7f) & 0x80808080_80808080;
        let lower = word | 0x20202020_20202020;
        (
            (
                !(lower + 0x1f1f1f1f_1f1f1f1f) |
                (lower + 0x05050505_05050505)
            ) & mask
        ) == 0
    }
}

impl Deref for TinyStr8 {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        // Again, could use #cfg to hand-roll a big-endian implementation.
        let word = self.0.get().to_le();
        let len = (8 - word.leading_zeros() / 8) as usize;
        unsafe {
            let slice = core::slice::from_raw_parts(&self.0 as *const _ as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl fmt::Display for TinyStr8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl fmt::Debug for TinyStr8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

unsafe fn make_4byte_str(text: &str, len: usize, mask: u32) -> Result<NonZeroU32, Error> {
    // Mask is always supplied as little-endian.
    let mask = mask.to_le();
    let mut word: u32 = 0;
    copy_nonoverlapping(text.as_ptr(), &mut word as *mut u32 as *mut u8, len);
    if (word & mask) != 0 {
        return Err(Error::NonAscii);
    }
    if ((mask - word) & mask) != 0 {
        return Err(Error::InvalidNull);
    }
    Ok(NonZeroU32::new_unchecked(word))
}

impl TinyStr4 {
    /// Create a new tiny string.
    ///
    /// Returns an error result if the string is not 1 to 4 characters in length,
    /// contains non-ASCII, or contains an embedded NUL byte.
    pub fn new(text: &str) -> Result<TinyStr4, Error> {
        unsafe {
            match text.len() {
                1 => make_4byte_str(text, 1, 0x80).map(TinyStr4),
                2 => make_4byte_str(text, 2, 0x8080).map(TinyStr4),
                3 => make_4byte_str(text, 3, 0x808080).map(TinyStr4),
                4 => make_4byte_str(text, 4, 0x80808080).map(TinyStr4),
                _ => Err(Error::InvalidSize),
            }
        }
    }

    /// Dereference to string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    pub fn to_ascii_uppercase(self) -> TinyStr4 {
        let word = self.0.get();
        let result = word &
        !(
            (
                (word + 0x1f1f1f1f) &
                !(word + 0x05050505) &
                0x80808080
            ) >> 2
        );
        unsafe { TinyStr4(NonZeroU32::new_unchecked(result)) }
    }

    pub fn to_ascii_lowercase(self) -> TinyStr4 {
        let word = self.0.get();
        let result = word |
        (
            (
                (word + 0x3f3f3f3f) &
                !(word + 0x25252525) &
                0x80808080
            ) >> 2
        );
        unsafe { TinyStr4(NonZeroU32::new_unchecked(result)) }
    }

    /// Makes the string all lowercase except for the first character,
    /// which is made uppercase.
    pub fn to_ascii_titlecase(self) -> TinyStr4 {
        let word = self.0.get().to_le();
        let mask = (
            (word + 0x3f3f3f1f) &
            !(word + 0x25252505) &
            0x80808080
        ) >> 2;
        let result = (word | mask) & !(0x20 & mask);
        unsafe { TinyStr4(NonZeroU32::new_unchecked(result.to_le())) }
    }

    /// Determine whether string is all ASCII alphabetical characters.
    pub fn is_all_ascii_alpha(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f7f7f) & 0x80808080;
        let lower = word | 0x20202020;
        (
            (
                !(lower + 0x1f1f1f1f) |
                (lower + 0x05050505)
            ) & mask
        ) == 0
    }
}

impl Deref for TinyStr4 {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        // Again, could use #cfg to hand-roll a big-endian implementation.
        let word = self.0.get().to_le();
        let len = (4 - word.leading_zeros() / 8) as usize;
        unsafe {
            let slice = core::slice::from_raw_parts(&self.0 as *const _ as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl fmt::Display for TinyStr4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl fmt::Debug for TinyStr4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, TinyStr4, TinyStr8};
    use std::ops::Deref;

    #[test]
    fn tiny4_basic() {
        let s = TinyStr4::new("abc").unwrap();
        assert_eq!(s.deref(), "abc");
    }

    #[test]
    fn tiny4_size() {
        assert_eq!(TinyStr4::new(""), Err(Error::InvalidSize));
        assert!(TinyStr4::new("1").is_ok());
        assert!(TinyStr4::new("12").is_ok());
        assert!(TinyStr4::new("123").is_ok());
        assert!(TinyStr4::new("1234").is_ok());
        assert_eq!(TinyStr4::new("12345"), Err(Error::InvalidSize));
        assert_eq!(TinyStr4::new("123456789"), Err(Error::InvalidSize));
    }

    #[test]
    fn tiny4_null() {
        assert_eq!(TinyStr4::new("a\u{0}b"), Err(Error::InvalidNull));
    }

    #[test]
    fn tiny4_nonascii() {
        assert_eq!(TinyStr4::new("\u{4000}"), Err(Error::NonAscii));
    }

    #[test]
    fn tiny4_alpha() {
        let s = TinyStr4::new("@aZ[").unwrap();
        assert!(!s.is_all_ascii_alpha());
        assert_eq!(s.to_ascii_uppercase().as_str(), "@AZ[");
        assert_eq!(s.to_ascii_lowercase().as_str(), "@az[");

        assert!(TinyStr4::new("abYZ").unwrap().is_all_ascii_alpha());
    }

    #[test]
    fn tiny4_titlecase() {
        assert_eq!(TinyStr4::new("abcd").unwrap().to_ascii_titlecase().as_str(), "Abcd");
        assert_eq!(TinyStr4::new("ABCD").unwrap().to_ascii_titlecase().as_str(), "Abcd");
        assert_eq!(TinyStr4::new("aBCD").unwrap().to_ascii_titlecase().as_str(), "Abcd");
        assert_eq!(TinyStr4::new("A123").unwrap().to_ascii_titlecase().as_str(), "A123");
        assert_eq!(TinyStr4::new("123a").unwrap().to_ascii_titlecase().as_str(), "123a");
    }

    #[test]
    fn tiny8_basic() {
        let s = TinyStr8::new("abcde").unwrap();
        assert_eq!(s.deref(), "abcde");
    }

    #[test]
    fn tiny8_size() {
        assert_eq!(TinyStr8::new(""), Err(Error::InvalidSize));
        assert!(TinyStr8::new("1").is_ok());
        assert!(TinyStr8::new("12").is_ok());
        assert!(TinyStr8::new("123").is_ok());
        assert!(TinyStr8::new("1234").is_ok());
        assert!(TinyStr8::new("12345").is_ok());
        assert!(TinyStr8::new("123456").is_ok());
        assert!(TinyStr8::new("1234567").is_ok());
        assert!(TinyStr8::new("12345678").is_ok());
        assert_eq!(TinyStr8::new("123456789"), Err(Error::InvalidSize));
    }

    #[test]
    fn tiny8_null() {
        assert_eq!(TinyStr8::new("a\u{0}b"), Err(Error::InvalidNull));
    }

    #[test]
    fn tiny8_nonascii() {
        assert_eq!(TinyStr8::new("\u{4000}"), Err(Error::NonAscii));
    }

    #[test]
    fn tiny8_alpha() {
        let s = TinyStr8::new("@abcXYZ[").unwrap();
        assert!(!s.is_all_ascii_alpha());
        assert_eq!(s.to_ascii_uppercase().as_str(), "@ABCXYZ[");
        assert_eq!(s.to_ascii_lowercase().as_str(), "@abcxyz[");

        assert!(TinyStr8::new("abcXYZ").unwrap().is_all_ascii_alpha());
    }
}
