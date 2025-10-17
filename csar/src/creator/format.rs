use std::{ffi::*, path::*};

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

    /// ZIP.
    ZIP,
}

impl Format {
    /// From path extension.
    pub fn from_extension(path: &Path) -> Option<Self> {
        if let Some(extension) = path.extension() {
            if (extension == OsStr::new("gz")) || (extension == OsStr::new("tgz")) {
                return Some(Self::GzipTarball);
            } else if (extension == OsStr::new("zip")) || (extension == OsStr::new("csar")) {
                return Some(Self::ZIP);
            }
        }

        None
    }
}
