use super::{
    super::{super::errors::*, root::*},
    command::*,
};

use {
    clap::*,
    compris::{annotate::*, normal::*, ser::*},
    depiction::*,
    kutil::{cli::run::*, std::error::*},
    puccini_csar::{tosca_meta::*, url::*, *},
    read_url::*,
    std::{
        fs::*,
        io::{self, IsTerminal, Write},
    },
};

impl Inspect {
    /// Run inspect subcommand.
    pub fn run(&self, root: &Root) -> Result<(), MainError> {
        match &self.file_or_url {
            Some(file_or_url) => self.from_url(file_or_url.clone(), root),
            None => self.from_stdin(root),
        }
    }

    fn from_url(&self, file_or_url: String, root: &Root) -> Result<(), MainError> {
        let url_context = Self::url_context()?;
        let mut csar_errors = Errors::default();
        let url = CsarUrl::new(url_context, file_or_url, None);
        let tosca_meta = url.tosca_meta(true, &mut csar_errors)?;
        self.output(tosca_meta, csar_errors, root)
    }

    fn from_stdin(&self, root: &Root) -> Result<(), MainError> {
        let stdin = io::stdin();
        if stdin.is_terminal() {
            Root::command().print_help()?;
            return Err(ExitError::success().into());
        }

        let mut csar_errors = Errors::default();
        let tosca_meta = ToscaMeta::read(&mut io::BufReader::new(stdin), &mut csar_errors)?;
        self.output(Some(tosca_meta), csar_errors, root)
    }

    fn output(
        &self,
        tosca_meta: Option<ToscaMeta>,
        csar_errors: Errors<CsarError>,
        root: &Root,
    ) -> Result<(), MainError> {
        if let Err(csar_errors) = csar_errors.check() {
            if !root.quiet {
                for error in csar_errors {
                    error.eprint_default_depiction();
                }
            }

            return Err(ExitError::new(1, None).into());
        }

        if let Some(tosca_meta) = tosca_meta {
            if let Some(output_format) = &self.output_format
                && matches!(output_format, OutputFormat::Text)
            {
                let tosca_meta = tosca_meta.stringify(Some(self.max_columns))?;
                if let Some(output_file) = &self.output_file {
                    let mut file = io::BufWriter::new(File::create(output_file)?);
                    file.write_all(tosca_meta.as_bytes())?;
                } else if !root.quiet {
                    print!("{}", tosca_meta);
                }
            } else {
                let format = self.output_format.as_ref().and_then(|format| format.to_compris());
                let variant: Variant<WithoutAnnotations> = tosca_meta.into();

                RepresentationWriter::new(format, !self.output_plain, self.output_base64).write_to_file_or_stdout(
                    &variant,
                    root.quiet,
                    false,
                    true,
                    self.output_file.as_ref(),
                )?;
            }
        }

        Ok(())
    }

    fn url_context() -> Result<UrlContextRef, MainError> {
        let url_context = UrlContext::new();

        #[cfg(feature = "filesystem")]
        let base_urls = url_context.working_dir_url_vec()?;

        #[cfg(not(feature = "filesystem"))]
        let base_urls = Vec::default();

        Ok(url_context.with_base_urls(base_urls))
    }
}
