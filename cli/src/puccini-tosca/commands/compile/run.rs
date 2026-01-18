use super::{super::root::*, command::*};

use {
    compris::annotate::*,
    floria::*,
    problemo::{common::*, *},
    puccini_tosca::grammar::*,
    read_url::*,
};

impl Compile {
    /// Run compile subcommand.
    pub fn run(&self, root: &Root) -> Result<(), Problem> {
        #[cfg(not(feature = "plugins"))]
        if self.instantiate {
            return Err(ExitError::failure("to use `--instantiate` you must enable \"plugins\" feature in the build"));
        }

        if !self.instantiate {
            if !self.events.is_empty() || self.update {
                return Err(ExitError::failure_message("cannot use `--event` without `--instantiate`"));
            }

            if !self.inputs.is_empty() {
                return Err(ExitError::failure_message("cannot use `--inputs` without `--instantiate`"));
            }

            if !self.inputs_from.is_empty() {
                return Err(ExitError::failure_message("cannot use `--inputs-from` without `--instantiate`"));
            }

            if !self.outputs.is_empty() {
                return Err(ExitError::failure_message("cannot use `--output` without `--instantiate`"));
            }
        }

        #[allow(unreachable_code)]
        if self.annotations {
            #[cfg(feature = "with-annotations")]
            return self.run_annotated::<WithAnnotations>(root);

            #[cfg(feature = "without-annotations")]
            return self.run_annotated::<WithoutAnnotations>(root);

            #[cfg(not(all(feature = "with-annotations", feature = "without-annotations")))]
            Err(ExitError::failure(
                "you must enable \"with-annotations\" and/or \"without-annotations\" features in the build",
            ))
        } else {
            #[cfg(feature = "without-annotations")]
            return self.run_annotated::<WithoutAnnotations>(root);

            #[cfg(not(feature = "without-annotations"))]
            Err(ExitError::failure(
                "to use `--annotations=false` you must enable \"without-annotations\" feature in the build",
            ))
        }
    }

    fn run_annotated<AnnotatedT>(&self, root: &Root) -> Result<(), Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let mut url_context = Self::url_context()?;
        let mut csar_problems = Problems::default();

        let source_id = self.source_id(&url_context, &mut csar_problems)?;
        let mut catalog = Self::catalog::<AnnotatedT>()?;

        let mut tosca_problems = Problems::default();

        // Inputs

        let inputs = self.inputs::<AnnotatedT>(&url_context)?;

        // Load

        if let Some(new_url_context) = if self.annotations {
            catalog.load_source_with_annotations(&source_id, &url_context, &mut tosca_problems)
        } else {
            catalog.load_source_without_annotations(&source_id, &url_context, &mut tosca_problems)
        }? {
            url_context = new_url_context;
        }

        // Complete

        if self.should_complete() {
            catalog.complete_entities(&mut tosca_problems)?;
        }

        // Compile

        let store = InMemoryStore::default();
        let mut floria_service_template_id = None;

        let directory = self.floria_directory()?;

        if self.should_compile() {
            let mut context = CompilationContext::new(
                &source_id,
                &catalog,
                &directory,
                store.clone().as_ref(),
                tosca_problems.as_ref(),
            );
            floria_service_template_id = if self.annotations {
                catalog.compile_service_template_with_annotations(&mut context)
            } else {
                catalog.compile_service_template_without_annotations(&mut context)
            }?;
        }

        // Instantiate

        #[cfg(feature = "plugins")]
        let mut floria_problems = Problems::default();

        #[cfg(feature = "plugins")]
        let floria_instance = if self.instantiate
            && let Some(floria_service_template_id) = &floria_service_template_id
        {
            self.instantiate(
                floria_service_template_id,
                inputs,
                &directory,
                store.clone(),
                &url_context,
                &mut floria_problems,
            )?
        } else {
            None
        };

        // Output

        let mut print_first = true;
        let mut output_floria = true;

        let has_problems = Self::depict_problems(
            csar_problems,
            tosca_problems,
            floria_problems,
            &mut print_first,
            &mut output_floria,
            root,
        );

        self.depict_debug(&catalog, &mut print_first, &mut output_floria, root);

        #[cfg(feature = "plugins")]
        self.output_floria_instance(floria_instance, store.clone(), &mut print_first, &mut output_floria, root)?;

        self.output_floria_template(floria_service_template_id, store, &mut print_first, &mut output_floria, root)?;

        return if has_problems { Err(ExitError::failure()) } else { Ok(()) };
    }

    fn source_id(&self, url_context: &UrlContextRef, problems: &mut Problems) -> Result<SourceID, Problem> {
        match self.input_file_or_url.clone() {
            Some(url) => url_to_source_id(url, url_context, problems),
            None => Ok(Default::default()),
        }
    }

    fn url_context() -> Result<UrlContextRef, Problem> {
        let url_context = UrlContext::new();

        #[cfg(feature = "filesystem")]
        let base_urls = url_context.working_dir_url_vec()?;

        #[cfg(not(feature = "filesystem"))]
        let base_urls = Vec::default();

        Ok(url_context.with_base_urls(base_urls))
    }
}
