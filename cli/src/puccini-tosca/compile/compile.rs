use super::super::{cli::*, errors::*};

use {
    compris::annotate::*,
    floria::*,
    kutil::{cli::run::*, std::error::*},
    puccini_tosca::grammar::*,
    read_url::*,
    std::fmt,
};

impl Compile {
    /// Run compile subcommand.
    pub fn run(&self, cli: &CLI) -> Result<(), MainError> {
        #[cfg(not(feature = "plugins"))]
        if self.instantiate {
            return Err(ExitError::from("to use `--instantiate` you must enable \"plugins\" feature in build").into());
        }

        if self.update && !self.instantiate {
            return Err(ExitError::from("cannot use `--update` without `--instantiate`").into());
        }

        if self.no_annotations {
            self.run_annotated::<WithoutAnnotations>(cli)
        } else {
            self.run_annotated::<WithAnnotations>(cli)
        }
    }

    /// Run compile subcommand.
    pub fn run_annotated<AnnotatedT>(&self, cli: &CLI) -> Result<(), MainError>
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default + Send + Sync,
    {
        let url_context = self.url_context()?;
        let source_id = self.source_id();
        let mut catalog = Self::catalog::<AnnotatedT>();

        let mut tosca_errors = Errors::<ToscaError<AnnotatedT>>::default();

        #[cfg(feature = "plugins")]
        let mut floria_errors = Errors::<FloriaError>::default();

        // Load

        if self.no_annotations {
            catalog.load_source_without_annotations(&source_id, &url_context, &mut tosca_errors)?;
        } else {
            catalog.load_source_with_annotations(&source_id, &url_context, &mut tosca_errors)?;
        }

        // Complete

        if self.should_complete() {
            catalog.complete_entities(&mut tosca_errors)?;
        }

        // Compile

        let store = InMemoryStore::default();
        let mut floria_service_template_id = None;

        let directory = self.floria_directory();

        if self.should_compile() {
            floria_service_template_id =
                catalog.compile_service_template(&directory, store.to_ref(), &source_id, &mut tosca_errors)?;
        }

        // Instantiate

        #[cfg(feature = "plugins")]
        let mut floria_instance = None;

        #[cfg(feature = "plugins")]
        if self.instantiate
            && let Some(floria_service_template_id) = &floria_service_template_id
        {
            floria_instance = self.instantiate(floria_service_template_id, &directory, &store, &mut floria_errors)?;
        }

        // Output

        let mut print_first = true;
        let mut output_floria = true;

        Self::depict_errors(&tosca_errors, &floria_errors, &mut print_first, &mut output_floria, cli);

        self.depict_debug(&catalog, &mut print_first, &mut output_floria, cli);

        #[cfg(feature = "plugins")]
        self.output_floria_instance(floria_instance, &store, &mut print_first, &mut output_floria, cli)?;

        self.output_floria_template(floria_service_template_id, &store, &mut print_first, &mut output_floria, cli)?;

        #[cfg(not(feature = "plugins"))]
        let has_errors = tosca_errors.is_empty();

        #[cfg(feature = "plugins")]
        let has_errors = tosca_errors.is_empty() && floria_errors.is_empty();

        return if has_errors { Ok(()) } else { Err(ExitError::new(1, None).into()) };
    }

    fn url_context(&self) -> Result<UrlContextRef, MainError> {
        let url_context = UrlContext::new();

        #[cfg(feature = "filesystem")]
        let base_urls = url_context.working_dir_url_vec()?;

        #[cfg(not(feature = "filesystem"))]
        let base_urls = Vec::default();

        Ok(url_context.with_base_urls(base_urls))
    }

    fn source_id(&self) -> SourceID {
        SourceID::url_or_default(self.input_file_or_url.clone().map(|input_path_or_url| input_path_or_url.into()))
    }

    fn floria_directory(&self) -> floria::Directory {
        self.directory
            .as_ref()
            .map(|directory| {
                let Ok(directory) = directory.parse();
                directory
            })
            .unwrap_or_else(|| Default::default())
    }
}
