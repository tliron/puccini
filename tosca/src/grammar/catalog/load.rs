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
    pub fn load_source<AnnotatedT, ErrorReceiverT>(
        &mut self,
        source_id: &SourceID,
        url_context: &UrlContextRef,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorReceiverT: ErrorReceiver<ToscaError<AnnotatedT>>,
    {
        if self.sources.contains_key(source_id) {
            return Ok(());
        }

        // Read and parse
        let (variant, url_context) = match source_id {
            SourceID::ID(id) => {
                tracing::info!(source = source_id.to_string(), "reading from stdin");
                let mut stdin = io::stdin();
                if !stdin.is_terminal() {
                    let parser = Parser::new(Format::YAML).with_source(source_id.into());
                    (unwrap_or_give_and_return!(parser.parse_reader(&mut stdin), errors, Ok(())), url_context.clone())
                } else {
                    tracing::error!("cannot load source from stdin: {}", id);
                    errors.give(SourceNotLoadedError::new(source_id.clone()))?;
                    return Ok(());
                }
            }

            SourceID::URL(url) => {
                tracing::info!(source = source_id.to_string(), "reading");
                let url = unwrap_or_give_and_return!(url_context.url_or_file_path(&url), errors, Ok(()));
                let mut reader = io::BufReader::new(unwrap_or_give_and_return!(url.open(), errors, Ok(())));
                let parser = Parser::new(Format::YAML).with_source(source_id.into());
                (
                    unwrap_or_give_and_return!(parser.parse_reader(&mut reader), errors, Ok(())),
                    url.base()
                        .and_then(|base| {
                            let mut base_urls = url_context.clone_base_urls();
                            base_urls.insert(0, base.into());
                            Some(url_context.with_base_urls(base_urls))
                        })
                        .unwrap_or_else(|| url_context.clone()),
                )
            }

            SourceID::Internal(internal) => {
                tracing::error!("cannot load internal source: {}", internal);
                errors.give(SourceNotLoadedError::new(source_id.clone()))?;
                return Ok(());
            }
        };

        Ok(match get_dialect_id(&variant) {
            Some(dialect_id) => {
                let dialect = self.get_dialect_ref(dialect_id)?;

                let mut source = Source::new(source_id.clone(), dialect_id.clone());

                // Merge implicit sources
                for (internal_source_id, internal_source) in &self.sources {
                    if matches!(internal_source_id, SourceID::Internal(_)) {
                        tracing::debug!(
                            source = source_id.to_string(),
                            from = internal_source_id.to_string(),
                            "merging namespace"
                        );

                        for (entity_kind, full_name, source_id) in internal_source.namespace() {
                            unwrap_or_give!(source.map_name(entity_kind, full_name.clone(), source_id.clone()), errors);
                        }
                    }
                }

                // Initialize
                tracing::debug!(source = source_id.to_string(), "initializing");
                dialect
                    .initialize_source(&mut source, variant, errors.into_annotated().to_ref())
                    .map_err(|error| error.into_annotated())?;

                let dependencies = source.dependencies.clone();

                // Load dependencies (recurse)
                for source_id in dependencies.keys() {
                    self.load_source(source_id, &url_context, errors).map_err(|error| error.into_annotated())?;
                }

                // Merge namespaces
                for (dependency_source_id, namespace) in dependencies {
                    tracing::debug!(
                        source = source_id.to_string(),
                        from = dependency_source_id.to_string(),
                        namespace = namespace.to_string(),
                        "merging namespace"
                    );

                    let dependency = self.get_source(&dependency_source_id)?;
                    for (entity_kind, full_name, source_id) in dependency.namespace() {
                        unwrap_or_give!(
                            source.map_name(
                                entity_kind,
                                full_name.clone().into_namespace(namespace.clone()),
                                source_id.clone()
                            ),
                            errors,
                        );
                    }
                }

                self.add_source(source);
            }

            None => {
                errors.give(UnsupportedSourceError::new(source_id.clone()))?;
            }
        })
    }
}
