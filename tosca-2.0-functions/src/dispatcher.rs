use super::functions::*;

use floria_plugin_sdk::{data::*, dispatch_bindings::*, export_dispatcher};

//
// Dispatcher
//

/// Dispatcher.
pub struct Dispatcher;

export_dispatcher!(Dispatcher);

impl Guest for Dispatcher {
    type AnyList = List;
    type AnyMap = Map;
    type AnyCall = Call;

    fn dispatch(name: String, arguments: Vec<Any>, site: Site) -> Result<Any, String> {
        match &*name {
            // Graph
            "get_input" => get_input(arguments, site),
            "get_property" => get_property(arguments, site),
            "get_attribute" => get_attribute(arguments, site),
            "get_artifact" => get_artifact(arguments, site),
            "value" => value(arguments, site),
            "node_index" => node_index(arguments, site),
            "relationship_index" => relationship_index(arguments, site),
            "available_allocation" => available_allocation(arguments, site),
            "select_capability" => select_capability(arguments, site),

            // Boolean logic
            "and" => and(arguments, site),
            "or" => or(arguments, site),
            "not" => not(arguments, site),
            "xor" => xor(arguments, site),

            // Boolean comparison
            "equal" => equal(arguments, site),
            "greater_than" => greater_than(arguments, site),
            "greater_or_equal" => greater_or_equal(arguments, site),
            "less_than" => less_than(arguments, site),
            "less_or_equal" => less_or_equal(arguments, site),
            "valid_values" => valid_values(arguments, site),
            "matches" => matches(arguments, site),

            // Boolean collection
            "has_suffix" => has_suffix(arguments, site),
            "has_prefix" => has_prefix(arguments, site),
            "contains" => contains(arguments, site),
            "has_entry" => has_entry(arguments, site),
            "has_key" => has_key(arguments, site),
            "has_all_entries" => has_all_entries(arguments, site),
            "has_all_keys" => has_all_keys(arguments, site),
            "has_any_entry" => has_any_entry(arguments, site),
            "has_any_key" => has_any_key(arguments, site),

            // Collection
            "length" => length(arguments, site),
            "concat" => concat(arguments, site),
            "join" => join(arguments, site),
            "token" => token(arguments, site),

            // Set
            "union" => union(arguments, site),
            "intersection" => intersection(arguments, site),

            // Arithmetic
            "sum" => sum(arguments, site),
            "difference" => difference(arguments, site),
            "product" => product(arguments, site),
            "quotient" => quotient(arguments, site),
            "remainder" => remainder(arguments, site),
            "round" => round(arguments, site),
            "floor" => floor(arguments, site),
            "ceil" => ceil(arguments, site),

            // Data
            "_is_a" => is_a(arguments, site),
            "_literal" => literal(arguments, site),
            "_list" => list(arguments, site),
            "_map" => map(arguments, site),

            _ => Err("unsupported function".into()),
        }
    }
}
