use super::{
    super::{errors::*, tosca_meta::*},
    format::*,
};

use {kutil::std::error::*, read_url::*, std::io};

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
        self.format.unwrap_or_else(|| Format::from_url(&self.url).unwrap_or_default())
    }

    /// Get TOSCA meta.
    pub fn tosca_meta<ErrorReceiverT>(
        &self,
        validate_location: bool,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<ToscaMeta>, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Ok(match self.tosca_meta_url(validate_location, errors)? {
            Some(tosca_meta_url) => {
                let reader = must_unwrap_give!(tosca_meta_url.open(), errors);
                let mut reader = io::BufReader::new(reader);
                Some(ToscaMeta::read(&mut reader, errors)?)
            }

            None => {
                errors.give(CsarError::Invalid(format!("archive does not have \"TOSCA.meta\": {}", self.url)))?;
                None
            }
        })
    }

    /// Get TOSCA meta URL.
    pub fn tosca_meta_url<ErrorReceiverT>(
        &self,
        validate_location: bool,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<UrlRef>, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        let mut tosca_meta_url = None;

        let format = self.format();
        for location in tosca_meta_locations() {
            let url = format.with_scheme(&self.url, &location.display().to_string());

            match self.url_context.url(&url) {
                Ok(url) => {
                    if validate_location {
                        if tosca_meta_url.is_some() {
                            errors.give(CsarError::Invalid("multiple \"TOSCA.meta\" files in CSAR".into()))?;
                        }

                        tosca_meta_url = Some(url);
                    } else {
                        tosca_meta_url = Some(url);
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

        Ok(tosca_meta_url)
    }

    /// Get entry definitions URL.
    pub fn entry_definitions_url<ErrorReceiverT>(&self, errors: &mut ErrorReceiverT) -> Result<UrlRef, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        let Some(meta) = self.tosca_meta(false, errors)? else {
            return Err(CsarError::Missing("TOSCA.meta".into()));
        };

        match &meta.entry_definitions {
            Some(entry_definitions) => {
                let url = self.format().with_scheme(&self.url, entry_definitions);
                Ok(self.url_context.url(&url)?)
            }

            None => Err(CsarError::Missing("entry_definitions".into())),
        }
    }
}
