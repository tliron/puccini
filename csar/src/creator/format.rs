use {
    kutil::std::string::*,
    std::{fmt, path::*, str::*},
};

//
// Format
//

/// Format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    /// Tarball.
    Tarball,

    /// Gzip tarball.
    GzipTarball,

    /// Zstandard tarball.
    ZstandardTarball,

    /// ZIP.
    ZIP,
}

impl Format {
    /// As string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tarball => "tarball",
            Self::GzipTarball => "Gzip tarball",
            Self::ZstandardTarball => "Zstandard tarball",
            Self::ZIP => "ZIP",
        }
    }

    /// From path extension.
    pub fn from_extension(path: &Path) -> Option<Self> {
        if let Some(extension) = path.extension()
            && let Some(extension) = extension.to_str()
        {
            extension.parse().ok()
        } else {
            None
        }
    }
}

impl FromStr for Format {
    type Err = ParseError;

    fn from_str(extension: &str) -> Result<Self, Self::Err> {
        match extension {
            "tar" => Ok(Self::Tarball),
            "gz" | "tgz" => Ok(Self::GzipTarball),
            "zst" => Ok(Self::ZstandardTarball),
            "zip" | "csar" => Ok(Self::ZIP),
            _ => Err(format!("unsupported format: {}", extension).into()),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}
