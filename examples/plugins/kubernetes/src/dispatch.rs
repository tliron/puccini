use super::functions::*;

use floria_plugin_sdk::*;

impl_dispatch!(arguments, call_site, {
    "kubernetes" => kubernetes(arguments, call_site),
});
