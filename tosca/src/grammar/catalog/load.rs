use super::{
    super::{dialect::*, errors::*, source::*},
    catalog::*,
};

use {
    compris::{parse::*, *},
    duplicate::*,
    problemo::*,
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
    pub fn load_source<ProblemReceiverT>(
        &mut self,
        source_id: &SourceID,
        url_context: &UrlContextRef,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<UrlContextRef>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut new_url_context = None;

        if self.sources.contains_key(source_id) {
            return Ok(new_url_context);
        }

        // Read and parse
        let (variant, url_context) = match source_id {
            SourceID::URL(url) => {
                tracing::info!(source = source_id.to_string(), "reading");
                let url = give_unwrap!(url_context.url_or_file_path(&url), problems, new_url_context);
                let mut reader = io::BufReader::new(give_unwrap!(url.open(), problems, new_url_context));
                let parser = Parser::new(Format::YAML).with_source(source_id.into());
                (
                    give_unwrap!(parser.parse_reader(&mut reader), problems, new_url_context),
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
                    (give_unwrap!(parser.parse_reader(&mut stdin), problems, new_url_context), url_context.clone())
                } else {
                    tracing::error!("cannot load source from stdin: {}", id);
                    problems.give(SourceNotLoadedError::as_problem(source_id.clone()))?;
                    return Ok(new_url_context);
                }
            }

            SourceID::Internal(internal) => {
                tracing::error!("cannot load internal source: {}", internal);
                problems.give(SourceNotLoadedError::as_problem(source_id.clone()))?;
                return Ok(new_url_context);
            }
        };

        Ok(match get_dialect_id(&variant) {
            Some(dialect_id) => {
                let dialect = give_unwrap!(self.get_dialect_ref(dialect_id), problems);
                let mut source = Source::new(source_id.clone(), dialect_id.clone());

                // Merge internal sources
                for (internal_source_id, internal_source) in &self.sources {
                    if internal_source_id.is_internal() {
                        source.merge_namespace(internal_source, &Default::default(), problems)?;
                    }
                }

                // Initialize
                tracing::debug!(source = source_id.to_string(), "initializing");
                dialect.initialize_source(&mut source, variant, problems.as_ref())?;

                // Load dependencies (recurse)
                for source_id in source.dependencies.keys() {
                    self.load_source(source_id, &url_context, problems)?;
                }

                // Merge namespaces
                for (dependency_source_id, namespace) in source.dependencies.clone() {
                    if let Some(dependency) = self.source(&dependency_source_id).give_ok(problems)? {
                        source.merge_namespace(dependency, &namespace, problems)?;
                    }
                }

                self.add_source(source);
                new_url_context
            }

            None => {
                problems.give(UnsupportedSourceError::as_problem(source_id.clone()))?;
                new_url_context
            }
        })
    }
}
