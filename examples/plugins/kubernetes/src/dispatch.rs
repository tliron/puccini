use super::functions::*;

use floria_plugin_sdk::*;

impl_dispatch!(arguments, call_site, {
    "apply_kubernetes" => apply_kubernetes(arguments, call_site),
    "get_kubernetes" => get_kubernetes(arguments, call_site),
});
