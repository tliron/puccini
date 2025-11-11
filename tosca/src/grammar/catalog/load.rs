use super::{
    super::{dialect::*, errors::*, source::*},
    catalog::*,
};

use {
    compris::{annotate::*, parse::*, *},
    duplicate::*,
    kutil::std::error::*,
    read_url::*,
    std::io::{self, IsTerminal},
};

impl Catalog {
    #[duplicate_item(
      load_source                       initialize_source;
      [load_source_with_annotations]    [initialize_source_with_annotations];
      [load_source_without_annotations] [initialize_source_without_annotations];
    )]
    /// Loads a [Source] and its imports (recursively) if not already loaded.
    ///
    /// A [UrlContext] is returned if the provided one was modified with an additional base  URL
    /// (that of the provided source).
    pub fn load_source<AnnotatedT, ErrorReceiverT>(
        &mut self,
        source_id: &SourceID,
        url_context: &UrlContextRef,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<UrlContextRef>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorReceiverT: ErrorReceiver<ToscaError<AnnotatedT>>,
    {
        let mut new_url_context = None;

        if self.sources.contains_key(source_id) {
            return Ok(new_url_context);
        }

        // Read and parse
        let (variant, url_context) = match source_id {
            SourceID::URL(url) => {
                tracing::info!(source = source_id.to_string(), "reading");
                let url = must_unwrap_give!(url_context.url_or_file_path(&url), errors, new_url_context);
                let mut reader = io::BufReader::new(must_unwrap_give!(url.open(), errors, new_url_context));
                let parser = Parser::new(Format::YAML).with_source(source_id.into());
                (
                    must_unwrap_give!(parser.parse_reader(&mut reader), errors, new_url_context),
                    url.base()
                        .and_then(|base| {
                            let mut base_urls = url_context.clone_base_urls();
                            base_urls.insert(0, base.into());
                            new_url_context = Some(url_context.with_base_urls(base_urls));
                            new_url_context.clone()
                        })
                        .unwrap_or_else(|| url_context.clone()),
                )
            }

            SourceID::Profile(_profile) => {
                // TODO
                return Ok(new_url_context);
            }

            SourceID::ID(id) => {
                tracing::info!(source = source_id.to_string(), "reading from stdin");
                let mut stdin = io::stdin();
                if !stdin.is_terminal() {
                    let parser = Parser::new(Format::YAML).with_source(source_id.into());
                    (must_unwrap_give!(parser.parse_reader(&mut stdin), errors, new_url_context), url_context.clone())
                } else {
                    tracing::error!("cannot load source from stdin: {}", id);
                    errors.give(SourceNotLoadedError::new(source_id.clone()))?;
                    return Ok(new_url_context);
                }
            }

            SourceID::Internal(internal) => {
                tracing::error!("cannot load internal source: {}", internal);
                errors.give(SourceNotLoadedError::new(source_id.clone()))?;
                return Ok(new_url_context);
            }
        };

        Ok(match get_dialect_id(&variant) {
            Some(dialect_id) => {
                let dialect = must_unwrap_give!(self.get_dialect_ref(dialect_id), errors);
                let mut source = Source::new(source_id.clone(), dialect_id.clone());

                // Merge internal sources
                for (internal_source_id, internal_source) in &self.sources {
                    if matches!(internal_source_id, SourceID::Internal(_)) {
                        source.merge_namespace(internal_source, &Default::default(), errors)?;
                    }
                }

                // Initialize
                tracing::debug!(source = source_id.to_string(), "initializing");
                dialect
                    .initialize_source(&mut source, variant, errors.into_annotated().as_ref())
                    .map_err(|error| error.into_annotated())?;

                let dependencies = source.dependencies.clone();

                // Load dependencies (recurse)
                for source_id in dependencies.keys() {
                    self.load_source(source_id, &url_context, errors)?;
                }

                // Merge namespaces
                for (dependency_source_id, namespace) in dependencies {
                    if let Some(dependency) = ok_give!(self.source(&dependency_source_id), errors) {
                        source.merge_namespace(dependency, &namespace, errors)?;
                    }
                }

                self.add_source(source);
                new_url_context
            }

            None => {
                errors.give(UnsupportedSourceError::new(source_id.clone()))?;
                new_url_context
            }
        })
    }
}
