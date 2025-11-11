use floria_plugin_sdk::{data::*, utils::*, *};

// https://iximiuz.com/en/posts/kubernetes-api-call-simple-http-client/
//
// KUBE_API=$(kubectl config view --output=jsonpath='{.clusters[0].cluster.server}')
// ARTIFACTS=examples/online-boutique/artifacts
// curl --cacert "$ARTIFACTS/cacert.pem" --cert "$ARTIFACTS/cert.pem" --key "$ARTIFACTS/key.pem" $KUBE_API/apis/apps/v1/deployments

/// Kubernetes.
pub fn kubernetes(arguments: Vec<Expression>, _call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 1)?;

    let client = HttpClient::default();
    let body = client.get_string("https://google.com")?;

    Ok(Some(body.into()))
}
