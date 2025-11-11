use super::super::super::super::grammar::*;

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

/// Get or create a plugin by URL.
pub fn get_or_create_plugin_by_url(
    url: ByteString,
    directory: floria::Directory,
    name: Option<ByteString>,
    context: &mut CompilationContext<'_>,
) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>> {
    Ok(Some(match context.store.get_plugin_by_url(&url)? {
        Some(plugin) => plugin.id,

        None => {
            let precompiled = url.ends_with(".cwasm");
            let plugin = match name {
                Some(name) => floria::Plugin::new_with_name(directory, name, url, precompiled)?,
                None => floria::Plugin::new_create_id(directory, url, precompiled, context.store.clone())?,
            };

            let plugin_id = plugin.id.clone();
            unwrap_or_give_and_return!(context.store.add_plugin(plugin), context.errors, Ok(None));
            plugin_id
        }
    }))
}
