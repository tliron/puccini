use super::super::{errors::*, meta::*};

use {kutil::std::error::*, read_url::*, std::io};

///
pub fn open_entry_definitions(_url: UrlRef, _url_context: UrlContextRef) {
    // is it .yaml or .yml?
    // if not, try as CsarUrl
    //let url = CsarUrl::new(url, url_context);
}

//
// CsarUrl
//

/// CSAR URL.
pub struct CsarUrl {
    /// URL context.
    pub url_context: UrlContextRef,

    /// URL.
    pub url: String,

    /// Format.
    pub format: Option<Format>,
}

impl CsarUrl {
    /// Constructor.
    pub fn new(url_context: UrlContextRef, url: String, format: Option<Format>) -> CsarUrl {
        Self { url_context, url, format }
    }

    /// Format.
    pub fn format(&self) -> Format {
        self.format.unwrap_or_else(|| Format::from_url(&self.url))
    }

    /// Get meta.
    pub fn meta<ErrorRecipientT>(
        &self,
        validate_location: bool,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<Meta>, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        let mut meta_url = None;

        let scheme = self.format().scheme();
        for location in meta_locations() {
            let url = format!("{}:{}!{}", scheme, self.url, location.display());

            match self.url_context.url(&url) {
                Ok(url) => {
                    if validate_location {
                        if meta_url.is_some() {
                            errors.give(CsarError::Invalid("multiple \"TOSCA.meta\" files in CSAR".into()))?;
                        }

                        meta_url = Some(url);
                    } else {
                        meta_url = Some(url);
                        break;
                    }
                }

                Err(UrlError::IO(error)) => {
                    if error.kind() != io::ErrorKind::NotFound {
                        errors.give(UrlError::IO(error))?;
                    }
                }

                Err(error) => {
                    errors.give(error)?;
                }
            }
        }

        Ok(match meta_url {
            Some(meta_url) => {
                let reader = unwrap_or_give_and_return!(meta_url.open(), errors, Ok(None));
                let mut reader = io::BufReader::new(reader);
                Some(Meta::read(&mut reader, errors)?)
            }

            None => {
                errors.give(CsarError::Invalid(format!("CSAR URL: {}", self.url)))?;
                None
            }
        })
    }

    // /// Get entry definitions URL.
    // pub fn entry_definitions_url(&self) -> UrlRef {
    //     let meta = self.meta();
    //     if let Some(_entry_definitions) = &meta.entry_definitions {
    //         // URL relative to archive URL
    //         todo!();
    //     } else {
    //         // Error
    //         todo!();
    //     }
    // }
}

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
    pub fn from_url(url: &str) -> Self {
        if url.ends_with(".zip") || url.ends_with(".csar") { Self::ZIP } else { Self::default() }
    }

    /// Scheme.
    pub fn scheme(&self) -> &'static str {
        match self {
            Format::Tarball => "tar",
            Format::ZIP => "zip",
        }
    }
}
