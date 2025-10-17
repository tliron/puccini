use super::{cli::*, errors::*};

use {
    compris::{annotate::*, normal::*, ser::*},
    kutil::{
        cli::{depict::*, run::*},
        std::error::*,
    },
    puccini_csar::url::*,
    read_url::*,
    std::{fs::*, io::Write},
};

impl Meta {
    /// Run meta subcommand.
    pub fn run(&self, cli: &CLI) -> Result<(), MainError> {
        let url_context = self.url_context()?;

        if let Some(file_or_url) = &self.file_or_url {
            let mut csar_errors = Errors::default();

            let url = CsarUrl::new(url_context, file_or_url.clone(), None);
            let meta = url.meta(true, &mut csar_errors)?;

            if let Err(csar_errors) = csar_errors.check() {
                if !cli.quiet {
                    for error in csar_errors {
                        error.eprint_default_depiction();
                    }
                }

                return Err(ExitError::new(1, None).into());
            }

            if let Some(meta) = meta {
                match self.compris_format() {
                    Some(format) => {
                        let variant: Variant<WithoutAnnotations> = meta.into();

                        let serializer =
                            Serializer::new(format).with_pretty(!self.output_plain).with_base64(self.output_base64);

                        if let Some(output_file) = &self.output_file {
                            let mut file = File::create(output_file)?;
                            serializer.write(&variant, &mut file)?;
                        } else if !cli.quiet {
                            serializer.print(&variant).expect("print");
                        }
                    }

                    None => {
                        if let Some(output_format) = &self.output_format
                            && matches!(output_format, OutputFormat::Text)
                        {
                            let meta = meta.stringify(Some(self.max_columns))?;
                            if let Some(output_file) = &self.output_file {
                                let mut file = File::create(output_file)?;
                                file.write_all(meta.as_bytes())?;
                            } else if !cli.quiet {
                                print!("{}", meta);
                            }
                        } else if !cli.quiet {
                            if self.output_file.is_some() {
                                return Err(ExitError::from("`--output` requires a compatible `--format`").into());
                            }

                            meta.print_default_depiction();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn url_context(&self) -> Result<UrlContextRef, MainError> {
        let url_context = UrlContext::new();

        #[cfg(feature = "filesystem")]
        let base_urls = url_context.working_dir_url_vec()?;

        #[cfg(not(feature = "filesystem"))]
        let base_urls = Vec::default();

        Ok(url_context.with_base_urls(base_urls))
    }

    fn compris_format(&self) -> Option<compris::Format> {
        match &self.output_format {
            Some(output_format) => output_format.to_compris(),
            None => self.output_file.as_ref().and_then(|output_file| compris::Format::from_path(output_file)),
        }
    }
}
