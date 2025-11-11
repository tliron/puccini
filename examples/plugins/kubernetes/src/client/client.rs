use super::{error::*, utils::*};

use {
    duplicate::*,
    floria_plugin_sdk::{data::*, utils::*, *},
    k8s_openapi::{List, api::apps::v1::*},
    serde::{de::*, ser::*},
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
    pub fn get<DeserializeT>(&self, url: &str) -> Result<DeserializeT, KubernetesError>
    where
        DeserializeT: DeserializeOwned,
    {
        let bytes = self.client.get_bytes(&format!("{}/apis/{}", self.url, url), Headers::default())?;
        Ok(from_json(bytes.as_slice())?)
    }

    /// Put a resource.
    pub fn put<BodyT>(&self, url: &str, body: BodyT) -> Result<BodyT, KubernetesError>
    where
        BodyT: Serialize + DeserializeOwned,
    {
        println!("{}", url);
        let bytes = to_json(body)?;
        let bytes = self.client.put_bytes(&format!("{}/apis/{}", self.url, url), Headers::default(), &bytes)?;
        Ok(from_json(bytes.as_slice())?)
    }

    /// Post a resource.
    pub fn post<BodyT>(&self, url: &str, body: BodyT) -> Result<BodyT, KubernetesError>
    where
        BodyT: Serialize + DeserializeOwned,
    {
        let bytes = to_json(body)?;
        let bytes = self.client.post_bytes(&format!("{}/apis/{}", self.url, url), Headers::default(), &bytes)?;
        Ok(from_json(bytes.as_slice())?)
    }

    #[duplicate_item(
      get_resources     Resource           url;
      [get_deployments] [List<Deployment>] ["apps/v1/{}deployments"];
    )]
    /// Get a resource.
    pub fn get_resources<NamespaceT>(&self, namespace: Option<NamespaceT>) -> Result<Resource, KubernetesError>
    where
        NamespaceT: AsRef<str>,
    {
        self.get(&format!(url, namespace_in_url(namespace)))
    }

    #[duplicate_item(
      get_resource     Resource     url;
      [get_deployment] [Deployment] ["apps/v1/{}deployments/{}"];
    )]
    /// Get a resource.
    pub fn get_resource<NamespaceT, NameT>(
        &self,
        namespace: Option<NamespaceT>,
        name: NameT,
    ) -> Result<Resource, KubernetesError>
    where
        NamespaceT: AsRef<str>,
        NameT: AsRef<str>,
    {
        self.get(&format!(url, namespace_in_url(namespace), name.as_ref()))
    }

    #[duplicate_item(
      create_resource     Resource     url;
      [create_deployment] [Deployment] ["apps/v1/{}deployments"];
    )]
    /// Create a resource.
    pub fn create_resource<NamespaceT>(
        &self,
        namespace: NamespaceT,
        resource: Resource,
    ) -> Result<Resource, KubernetesError>
    where
        NamespaceT: AsRef<str>,
    {
        self.post(&format!(url, namespace_in_url(Some(namespace))), resource)
    }

    #[duplicate_item(
      replace_resource     Resource     url;
      [replace_deployment] [Deployment] ["apps/v1/{}deployments/{}"];
    )]
    /// Replace a resource.
    pub fn replace_resource<NamespaceT, NameT>(
        &self,
        namespace: NamespaceT,
        name: NameT,
        resource: Resource,
    ) -> Result<Resource, KubernetesError>
    where
        NamespaceT: AsRef<str>,
        NameT: AsRef<str>,
    {
        self.put(&format!(url, namespace_in_url(Some(namespace)), name.as_ref()), resource)
    }
}
