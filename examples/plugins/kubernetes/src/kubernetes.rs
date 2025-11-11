use {
    duplicate::*,
    floria_plugin_sdk::{data::*, utils::*, *},
    k8s_openapi::{List, api::apps::v1::*},
    serde::de::*,
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
        serde_json::from_slice(&bytes).map_err(|error| escape_depiction_markup(error))
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

// Utils

fn namespace_in_url<NamespaceT>(namespace: Option<NamespaceT>) -> String
where
    NamespaceT: AsRef<str>,
{
    match namespace {
        Some(namespace) => format!("namespaces/{}/", namespace.as_ref()),
        None => "/".into(),
    }
}

fn http_client(configuration: &Map) -> Result<HttpClient, DispatchError> {
    let mut client = HttpClient::default();
    add_header(&mut client, "xx-root-certificates", configuration, "root-certificates")?;
    add_header(&mut client, "xx-certificates", configuration, "user-certificates")?;
    add_header(&mut client, "xx-private-key", configuration, "user-private-key")?;
    Ok(client)
}

fn add_header(client: &mut HttpClient, name: &str, configuration: &Map, key: &str) -> Result<(), DispatchError> {
    if let Some(value) = configuration.into_get(key)
        && let Expression::Text(value) = value
    {
        client.add_header(name, value)?;
    }
    Ok(())
}
