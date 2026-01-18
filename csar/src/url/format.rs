use read_url::*;

//
// Format
//

/// Format.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Format {
    /// Tarball.
    #[default]
    Tarball,

    /// ZIP.
    ZIP,
}

impl Format {
    /// From URL.
    pub fn from_url(url: &str) -> Option<Self> {
        if url.ends_with(".tar") || url.ends_with(".tar.gz") || url.ends_with(".tgz") || url.ends_with(".tar.zst") {
            return Some(Self::Tarball);
        }

        if url.ends_with(".zip") || url.ends_with(".csar") {
            return Some(Self::ZIP);
        }

        None
    }

    /// Scheme.
    pub fn scheme(&self) -> &'static str {
        match self {
            Format::Tarball => "tar",
            Format::ZIP => "zip",
        }
    }

    /// With scheme.
    pub fn with_scheme(&self, url: &str, path: &str) -> String {
        format_archive_url(self.scheme(), url, path)
    }
}
