use super::{
    super::{super::errors::*, root::*},
    command::*,
};

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
    pub fn run(&self, root: &Root) -> Result<(), MainError> {
        #[cfg(not(feature = "plugins"))]
        if self.instantiate {
            return Err(
                ExitError::from("to use `--instantiate` you must enable \"plugins\" feature in the build").into()
            );
        }

        if self.update && !self.instantiate {
            return Err(ExitError::from("cannot use `--update` without `--instantiate`").into());
        }

        #[allow(unreachable_code)]
        if self.no_annotations {
            #[cfg(feature = "without-annotations")]
            return self.run_annotated::<WithoutAnnotations>(root);

            #[cfg(not(feature = "without-annotations"))]
            Err(ExitError::from(
                "to use `--no-annotations` you must enable \"without-annotations\" feature in the build",
            )
            .into())
        } else {
            #[cfg(feature = "with-annotations")]
            return self.run_annotated::<WithAnnotations>(root);

            #[cfg(feature = "without-annotations")]
            return self.run_annotated::<WithoutAnnotations>(root);

            #[cfg(not(all(feature = "with-annotations", feature = "without-annotations")))]
            Err(ExitError::from(
                "you must enable \"with-annotations\" and/or \"without-annotations\" features in the build",
            )
            .into())
        }
    }

    fn run_annotated<AnnotatedT>(&self, root: &Root) -> Result<(), MainError>
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default + Send + Sync,
    {
        let url_context = Self::url_context()?;
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
        let floria_instance = if self.instantiate
            && let Some(floria_service_template_id) = &floria_service_template_id
        {
            self.instantiate(floria_service_template_id, &directory, &store, &mut floria_errors)?
        } else {
            None
        };

        // Output

        let mut print_first = true;
        let mut output_floria = true;

        Self::depict_errors(&tosca_errors, &floria_errors, &mut print_first, &mut output_floria, root);

        self.depict_debug(&catalog, &mut print_first, &mut output_floria, root);

        #[cfg(feature = "plugins")]
        self.output_floria_instance(floria_instance, &store, &mut print_first, &mut output_floria, root)?;

        self.output_floria_template(floria_service_template_id, &store, &mut print_first, &mut output_floria, root)?;

        #[cfg(not(feature = "plugins"))]
        let has_errors = tosca_errors.is_empty();

        #[cfg(feature = "plugins")]
        let has_errors = tosca_errors.is_empty() && floria_errors.is_empty();

        return if has_errors { Ok(()) } else { Err(ExitError::new(1, None).into()) };
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
