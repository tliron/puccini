use super::super::super::data::*;

use floria_plugin_sdk::{data::*, utils::*, *};

/// The $quotient function takes two arguments where the first argument is of an integer, float, or
/// scalar type and the second argument is of an integer or float type. The result is of
///
/// * A scalar type if the first argument is a scalar, and its value is the arithmetic division of
///   the first argument value by the second argument value. If necessary, the result might be
///   truncated, as decided by the implementation.
///
/// * A float if the first argument is an integer or a float. Note that to transform the float to
///   an integer a round or ceil or floor function must be used.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn quotient(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?;

    Ok(Some(match left {
        Expression::Custom(custom_resource) => {
            let scalar: Scalar = custom_resource.custom().try_into()?;
            let left = scalar.canonical()?;

            let factor: Number = right.try_into()?;
            let quotient = left.div(factor, false)?;

            // TODO to int or float

            let quotient = Scalar::new(quotient, scalar.schema.canonical_unit.clone(), scalar.schema);
            quotient.into()
        }

        _ => {
            let left: Number = left.try_into()?;
            let right: Number = right.try_into()?;
            left.div(right, false)?.into()
        }
    }))
}
