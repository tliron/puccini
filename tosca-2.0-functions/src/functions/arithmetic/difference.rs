use super::super::super::data::*;

use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $difference function takes two arguments of either integer, float, or scalar type. The
/// result is of the same type as the arguments and its value is the arithmetic subtraction of the
/// second argument value from the first argument value.
pub fn difference(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?;

    Ok(Some(match (left, right) {
        (Expression::Custom(left), Expression::Custom(right)) => {
            let scalar: Scalar = left.custom().try_into()?;
            let left = scalar.canonical()?;

            let right: Scalar = right.custom().try_into()?;
            let right = right.canonical()?;

            let difference = left.sub(right, true)?;
            let difference = Scalar::new(difference, scalar.schema.canonical_unit.clone(), scalar.schema);
            difference.into()
        }

        (left, right) => {
            let left: Number = left.try_into()?;
            let right: Number = right.try_into()?;
            left.sub(right, true)?.into()
        }
    }))
}
