use super::command::*;

use {
    compris::annotate::*,
    problemo::*,
    puccini_tosca::{dialect::tosca_2_0, grammar::*},
    read_url::*,
};

#[cfg(feature = "plugins")]
use floria::{plugins::*, *};

impl Compile {
    /// TOSCA [Catalog] with supported dialects.
    pub fn catalog<AnnotatedT>() -> Result<Catalog, Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let mut catalog = Catalog::default();
        tosca_2_0::Dialect::add_to_catalog::<AnnotatedT>(&mut catalog)?;
        Ok(catalog)
    }

    /// Floria [PluginContext] with the plugins for supported dialects.
    #[cfg(feature = "plugins")]
    pub fn plugin_context<'environment, StoreT>(
        &self,
        environment: PluginEnvironment,
        store: StoreT,
        url_context: UrlContextRef,
    ) -> Result<PluginContext<StoreT>, Problem>
    where
        StoreT: Clone + Send + Store,
    {
        let mut context = PluginContext::new(environment, store, url_context);

        if let Some(plugin) = context.store.get_plugin_by_url(&tosca_2_0::PLUGIN_URL)? {
            match &self.tosca_plugin {
                Some(plugin_url) => {
                    let precompiled = self.tosca_plugin_precompiled.unwrap_or_else(|| plugin_url.ends_with(".cwasm"));
                    context.load_dispatch_plugin(plugin.id, plugin_url, precompiled)?;
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
