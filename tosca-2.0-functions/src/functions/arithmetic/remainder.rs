use super::super::super::data::*;

use floria_plugin_sdk::{data::*, errors, utils::*};

/// The $remainder function takes two arguments where the first argument is of an integer, or
/// scalar type and the second argument is of an integer. The result is of the same type as the
/// first argument and its value is the remainder of the division to the second argument.
pub fn remainder(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?.cast_i64("second argument")?;

    match &left {
        Expression::Custom(custom_resource) => {
            let scalar: Scalar = custom_resource.custom().try_into()?;
            match scalar.canonical()? {
                Number::Integer(left) => {
                    let remainder = rem_i64(left, right)?;
                    let remainder = Scalar::new(remainder.into(), scalar.schema.canonical_unit.clone(), scalar.schema);
                    Ok(Some(remainder.into()))
                }

                _ => Err(errors::not_of_types_for("first argument", &left, &["integer", "integer scalar"])),
            }
        }

        Expression::Integer(left) => Ok(Some(rem_i64(*left, right)?.into())),

        _ => Err(errors::not_of_types_for("first argument", &left, &["integer", "integer scalar"])),
    }
}
