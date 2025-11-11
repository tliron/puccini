use super::kubernetes::*;

use floria_plugin_sdk::{data::*, utils::*, *};

// https://iximiuz.com/en/posts/kubernetes-api-call-simple-http-client/
//
// Show URL:
// kubectl -v=6
//
// KUBE_API=$(kubectl config view --output=jsonpath='{.clusters[0].cluster.server}')
// ARTIFACTS=examples/online-boutique/artifacts
//
// curl --cacert "$ARTIFACTS/cacert.pem" --cert "$ARTIFACTS/cert.pem" --key "$ARTIFACTS/key.pem" $KUBE_API/apis/apps/v1/deployments
//
// SSL_CERT_FILE=$ARTIFACTS/cacert.pem curl --cert "$ARTIFACTS/cert.pem" --key "$ARTIFACTS/key.pem" $KUBE_API/apis/apps/v1/deployments

/// Kubernetes.
pub fn kubernetes(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    let configuration = arguments.next().unwrap().must_evaluate(&call_site)?;
    let configuration = configuration.cast_map("argument")?.map();

    let client = KubernetesClient::new(&configuration)?;

    let deployments = client.get_deployments(Some("kube-system"))?;
    let deployments = serde_json::to_string(&deployments).map_err(|error| escape_depiction_markup(error))?;

    Ok(Some(deployments.into()))
}
