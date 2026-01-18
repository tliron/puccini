use super::super::errors::*;

use {
    problemo::{common::*, *},
    std::fmt,
};

/// Default compression level.
pub const DEFAULT_COMPRESSION_LEVEL: usize = 7;

//
// CompressionLevel
//

/// Compression level, from 1 to 10.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CompressionLevel(usize);

impl CompressionLevel {
    /// Constructor.
    pub const fn new_unchecked(compression_level: usize) -> Self {
        CompressionLevel(compression_level)
    }

    /// Gzip compression level is 0 to 9 (default 6).
    pub fn to_gzip(&self) -> u32 {
        let gzip = (self.0 - 1) as u32;
        assert!(gzip <= 9);
        gzip
    }

    /// Zstandard compression level is 1 to 22 (default 3).
    pub fn to_zstandard(&self) -> i32 {
        let zstandard = ((self.0 - 1) * 21 / 9 + 1) as i32;
        assert!((zstandard >= 1) && (zstandard <= 22));
        zstandard
    }

    /// ZIP DEFLATE (with flate only) compression level is 1 to 9 (default 6).
    pub fn to_zip_deflate(&self) -> i64 {
        let zip = ((self.0 - 1) * 8 / 9 + 1) as i64;
        assert!((zip >= 1) && (zip <= 9));
        zip
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel(DEFAULT_COMPRESSION_LEVEL)
    }
}

impl fmt::Display for CompressionLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

// Conversions

impl TryFrom<usize> for CompressionLevel {
    type Error = Problem;

    fn try_from(compression_level: usize) -> Result<Self, Self::Error> {
        if (compression_level >= 1) && (compression_level <= 10) {
            Ok(CompressionLevel(compression_level))
        } else {
            Err(InvalidError::as_problem(format!("compression level must be 1 to 10: {}", compression_level))
                .via(CsarError))
        }
    }
}

impl Into<usize> for CompressionLevel {
    fn into(self) -> usize {
        self.0
    }
}
