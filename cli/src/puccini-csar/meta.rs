use super::{cli::*, errors::*, utils::*};

use {
    kutil::{
        cli::{depict::*, run::*},
        std::error::*,
    },
    puccini_csar::Meta as CsarMeta,
    read_url::*,
    std::io,
};

impl Meta {
    /// Run meta subcommand.
    pub fn run(&self, cli: &CLI) -> Result<(), MainError> {
        let url_context = self.url_context()?;

        if let Some(csar_path_or_url) = &self.csar_path_or_url {
            let url = url_context.url(csar_path_or_url)?;
            let read = url.open()?;

            let mut csar_errors = Errors::default();
            let meta = CsarMeta::read(&mut io::BufReader::new(read), &mut csar_errors)?;

            if let Err(csar_errors) = csar_errors.check() {
                if !cli.quiet {
                    for error in csar_errors {
                        error.print_default_depiction();
                    }
                }

                return Err(ExitError::new(1, None).into());
            }

            output!(self, cli, meta);
        }

        // TODO: stdin

        Ok(())
    }

    /// URL context.
    pub fn url_context(&self) -> Result<UrlContextRef, MainError> {
        let url_context = UrlContext::new();

        #[cfg(feature = "filesystem")]
        let base_urls = url_context.working_dir_url_vec()?;

        #[cfg(not(feature = "filesystem"))]
        let base_urls = Vec::default();

        Ok(url_context.with_base_urls(base_urls))
    }

    /// Compris format.
    pub fn get_output_format(&self) -> Option<compris::Format> {
        self.output_format.as_ref().and_then(|format| format.to_compris())
    }
}
