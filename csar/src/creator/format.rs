use std::{ffi::*, fmt, path::*};

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
        if let Some(extension) = path.extension() {
            if extension == OsStr::new("tar") {
                return Some(Self::Tarball);
            } else if (extension == OsStr::new("gz")) || (extension == OsStr::new("tgz")) {
                return Some(Self::GzipTarball);
            } else if extension == OsStr::new("zst") {
                return Some(Self::ZstandardTarball);
            } else if (extension == OsStr::new("zip")) || (extension == OsStr::new("csar")) {
                return Some(Self::ZIP);
            }
        }

        None
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}
