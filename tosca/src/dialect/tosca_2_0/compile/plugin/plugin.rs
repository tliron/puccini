use super::super::super::{super::super::grammar::*, dialect::*};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

//
// Plugin
//

/// Plugin.
pub struct Plugin {
    /// Plugin URL.
    pub url: ByteString,

    /// Whether the plugin is global.
    pub global: bool,

    /// Function name.
    pub function: Option<ByteString>,

    /// Event name.
    pub event: Option<ByteString>,
}

impl Plugin {
    /// Constructor.
    pub fn new(url: ByteString, global: bool, function: Option<ByteString>, event: Option<ByteString>) -> Self {
        Self { url, global, function, event }
    }

    /// Get or create the implicit plugin.
    pub fn get_or_create_implicit(
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>> {
        Self::new(PLUGIN_URL, true, None, None).get_or_create(Some(PLUGIN_NAME), context)
    }

    /// Get or create.
    pub fn get_or_create(
        self,
        name: Option<ByteString>,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>> {
        Ok(Some(match context.store.get_plugin_by_url(&self.url)? {
            Some(plugin) => plugin.id,

            None => {
                let directory = if self.global { Default::default() } else { context.directory.clone() };
                let precompiled = self.url.ends_with(".cwasm");
                let plugin = match name {
                    Some(name) => floria::Plugin::new_with_name(directory, name, self.url, precompiled)?,
                    None => floria::Plugin::new_create_id(directory, self.url, precompiled, context.store.clone())?,
                };

                let plugin_id = plugin.id.clone();
                must_unwrap_give!(context.store.add_plugin(plugin), context.errors);
                plugin_id
            }
        }))
    }
}
