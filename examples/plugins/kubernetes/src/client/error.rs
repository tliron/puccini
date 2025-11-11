use super::utils::*;

use {
    floria_plugin_sdk::{utils::*, *},
    k8s_openapi::apimachinery::pkg::apis::meta::v1::*,
    std::fmt,
};

//
// KubernetesError
//

/// Kubernetes error.
pub struct KubernetesError {
    /// Message.
    pub message: String,

    /// Status code.
    pub status_code: u16,

    /// Status.
    pub status: Option<Status>,
}

impl fmt::Display for KubernetesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, formatter)
    }
}

impl From<HttpError> for KubernetesError {
    fn from(http: HttpError) -> Self {
        let mut message = http.message;

        let status = if let Some(body) = http.body { from_json::<_, Status>(body.as_slice()).ok() } else { None };

        if let Some(status) = &status
            && let Some(status) = &status.message
        {
            message += ": ";
            message += status;
        };

        KubernetesError { message, status_code: http.status_code, status }
    }
}

impl From<DispatchError> for KubernetesError {
    fn from(message: DispatchError) -> Self {
        KubernetesError { message, status_code: 0, status: None }
    }
}

impl From<KubernetesError> for DispatchError {
    fn from(error: KubernetesError) -> Self {
        escape_depiction_markup(error.message)
    }
}
