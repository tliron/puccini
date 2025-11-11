use super::super::client::*;

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

/// Get from Kubernetes.
pub fn get_kubernetes(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let configuration = arguments.next().unwrap().must_evaluate(&call_site)?;
    let configuration = configuration.cast_map("argument")?.map();

    let resource = arguments.next().unwrap().must_evaluate(&call_site)?;
    let (kind, name, namespace) = parse_resource(resource)?;

    let client = KubernetesClient::new(&configuration)?;

    match kind.as_str() {
        "deployments" => {
            let deployments = client.get_deployments(namespace)?;
            Ok(Some(to_expression(&deployments)?))
        }

        "deployment" => {
            let Some(name) = name else {
                return Err("missing |name|name|".into());
            };
            let deployment = client.get_deployment(namespace, name)?;
            Ok(Some(to_expression(&deployment)?))
        }

        _ => Err(format!("unsupported kind: |error|{}|", escape_depiction_markup(kind))),
    }
}

fn parse_resource(resource: Expression) -> Result<(String, Option<String>, Option<String>), DispatchError> {
    match resource {
        Expression::Map(resource) => {
            let resource = resource.map();

            let kind = match resource.into_get("kind") {
                Some(kind) => kind.cast_string_clone("kind")?,
                None => return Err("missing |name|kind|".into()),
            };

            let name = match resource.into_get("name") {
                Some(name) => Some(name.cast_string_clone("name")?),
                None => None,
            };

            let namespace = match resource.into_get("namespace") {
                Some(namespace) => Some(namespace.cast_string_clone("namespace")?),
                None => None,
            };

            Ok((kind, name, namespace))
        }

        Expression::Text(kind) => Ok((kind, None, None)),

        _ => Err(errors::not_of_types_for("resource", &resource, &["map", "text"])),
    }
}
