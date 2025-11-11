use super::command::*;

use {
    compris::annotate::*,
    puccini_tosca::{dialect::tosca_2_0, grammar::*},
    read_url::*,
    std::fmt,
};

#[cfg(feature = "plugins")]
use floria::{plugins::*, *};

impl Compile {
    /// TOSCA [Catalog] with supported dialects.
    pub fn catalog<AnnotatedT>() -> Result<Catalog, ToscaError<AnnotatedT>>
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut catalog = Catalog::default();
        catalog.add_dialect_ref(tosca_2_0::Dialect::default().into());
        catalog.add_sources(tosca_2_0::Dialect::built_in_sources::<AnnotatedT>()?);
        Ok(catalog)
    }

    /// Floria [PluginContext] with the plugins for supported dialects.
    #[cfg(feature = "plugins")]
    pub fn plugin_context<'environment, StoreT>(
        &self,
        environment: PluginEnvironment,
        store: StoreT,
        url_context: UrlContextRef,
    ) -> Result<PluginContext<StoreT>, FloriaError>
    where
        StoreT: Clone + Send + Store,
    {
        let mut context = PluginContext::new(environment, store, url_context);

        if let Some(plugin) = context.store.get_plugin_by_url(&tosca_2_0::PLUGIN_URL)? {
            match &self.plugin {
                Some(plugin_url) => {
                    context.load_dispatch_plugin(plugin.id, plugin_url, self.plugin_precompiled)?;
                }

                #[cfg(feature = "_blanket")]
                None => {}

                // Bundle the plugin
                #[cfg(not(feature = "_blanket"))]
                None => {
                    #[cfg(feature = "wasm-precompiled")]
                    context.add_dispatch_plugin(
                        plugin.id,
                        include_bytes!(concat!(env!("OUT_DIR"), "/puccini_plugin_tosca_2_0.cwasm")),
                        true,
                    )?;

                    #[cfg(not(feature = "wasm-precompiled"))]
                    context.add_dispatch_plugin(
                        plugin.id,
                        include_bytes!(concat!(env!("OUT_DIR"), "/puccini_plugin_tosca_2_0.wasm")),
                        false,
                    )?;
                }
            }
        }

        Ok(context)
    }
}
