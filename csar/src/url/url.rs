use super::{
    super::{errors::*, tosca_meta::*},
    format::*,
};

use {
    problemo::{common::*, *},
    read_url::*,
    std::io,
};

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
    pub fn new(url_context: UrlContextRef, url: String, format: Option<Format>) -> Self {
        Self { url_context, url, format }
    }

    /// Get entry definitions URL if CSAR.
    pub fn to_entry_definitions_url<ProblemReceiverT>(
        url: String,
        url_context: UrlContextRef,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<UrlRef>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match Format::from_url(&url) {
            Some(format) => {
                let csar_url = CsarUrl::new(url_context.clone(), url, Some(format));
                Some(csar_url.entry_definitions_url(problems)?)
            }

            None => None,
        })
    }

    /// Format.
    pub fn format(&self) -> Format {
        self.format.unwrap_or_else(|| Format::from_url(&self.url).unwrap_or_default())
    }

    /// Get TOSCA meta.
    pub fn tosca_meta<ProblemReceiverT>(
        &self,
        validate_location: bool,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<ToscaMeta>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match self.tosca_meta_url(validate_location, problems)? {
            Some(tosca_meta_url) => {
                let reader = give_unwrap!(tosca_meta_url.open(), problems);
                let mut reader = io::BufReader::new(reader);
                Some(ToscaMeta::read(&mut reader, problems)?)
            }

            None => {
                problems.give(
                    InvalidError::as_problem(format!("archive does not have \"TOSCA.meta\": {}", self.url))
                        .via(CsarError),
                )?;
                None
            }
        })
    }

    /// Get TOSCA meta URL.
    pub fn tosca_meta_url<ProblemReceiverT>(
        &self,
        validate_location: bool,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<UrlRef>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut tosca_meta_url = None;

        let format = self.format();
        for location in tosca_meta_locations() {
            let url = format.with_scheme(&self.url, &location.display().to_string());

            match self.url_context.url_or_file_path(&url) {
                Ok(url) => {
                    if validate_location {
                        if tosca_meta_url.is_some() {
                            problems.give(
                                InvalidError::as_problem("multiple \"TOSCA.meta\" files in CSAR").via(CsarError),
                            )?;
                        }

                        tosca_meta_url = Some(url);
                    } else {
                        tosca_meta_url = Some(url);
                        break;
                    }
                }

                Err(problem) => {
                    if !problem.has_error_type::<UnreachableError>() {
                        problems.give(problem.via(CsarError))?;
                    }
                }
            }
        }

        Ok(tosca_meta_url)
    }

    /// Get entry definitions URL.
    pub fn entry_definitions_url<ProblemReceiverT>(&self, problems: &mut ProblemReceiverT) -> Result<UrlRef, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let Some(meta) = self.tosca_meta(false, problems)? else {
            return Err(MissingError::as_problem("TOSCA.meta").via(CsarError));
        };

        match &meta.entry_definitions {
            Some(entry_definitions) => {
                let url = self.format().with_scheme(&self.url, entry_definitions);
                Ok(self.url_context.url_or_file_path(&url)?)
            }

            None => Err(MissingError::as_problem("entry_definitions").via(CsarError)),
        }
    }
}
