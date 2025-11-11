use super::utils::*;

use {
    duplicate::*,
    floria_plugin_sdk::{data::*, utils::*, *},
    k8s_openapi::{List, api::apps::v1::*},
    serde::de::*,
    struson::reader::*,
};

//
// KubernetesClient
//

/// Kubernetes client.
pub struct KubernetesClient {
    client: HttpClient,
    url: String,
}

impl KubernetesClient {
    /// Constructor.
    pub fn new(configuration: &Map) -> Result<Self, DispatchError> {
        let url = if let Some(url) = configuration.into_get("url")
            && let Expression::Text(url) = url
        {
            url.clone()
        } else {
            return Err("missing required key: |error|url|".into());
        };

        let client = http_client(&configuration)?;
        Ok(KubernetesClient { client, url })
    }

    /// Get a resource.
    pub fn get<DeserializeT>(&self, url: &str) -> Result<DeserializeT, DispatchError>
    where
        DeserializeT: DeserializeOwned,
    {
        let bytes = self.client.get_bytes(&(format!("{}/apis/{}", self.url, url)), Headers::default())?;
        //serde_json::from_slice(&bytes).map_err(|error| escape_depiction_markup(error))
        let mut reader = JsonStreamReader::new(bytes.as_slice());
        reader.deserialize_next().map_err(|error| escape_depiction_markup(error))
    }

    #[duplicate_item(
      get_resource       Resource            url;
      [get_deployments]  [List<Deployment>]  ["apps/v1/{}deployments"];
    )]
    /// Get a resource.
    pub fn get_resource<NamespaceT>(&self, namespace: Option<NamespaceT>) -> Result<Resource, DispatchError>
    where
        NamespaceT: AsRef<str>,
    {
        self.get(&format!(url, namespace_in_url(namespace)))
    }
}
