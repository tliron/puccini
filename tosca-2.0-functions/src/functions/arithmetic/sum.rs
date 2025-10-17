use super::super::super::data::*;

use floria_plugin_sdk::{data::*, errors, utils::*, *};

/// The $sum function takes one or more arguments of either integer, float, or scalar type. The
/// result is of the same type as the arguments and its value is the arithmetic sum of the
/// arguments' values.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn sum(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count_min(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    while let Some(argument) = arguments.next() {
        let argument = argument.must_evaluate(&call_site)?;
        match argument {
            Expression::Integer(mut sum) => {
                while let Some(argument) = arguments.next() {
                    let argument = argument.must_evaluate(&call_site)?.cast_i64("argument")?;
                    sum = add_i64(sum, argument)?;
                }

                return Ok(Some(sum.into()));
            }

            // Can't be created by TOSCA, but we still support it
            Expression::UnsignedInteger(mut sum) => {
                while let Some(argument) = arguments.next() {
                    let argument = argument.must_evaluate(&call_site)?.cast_u64("argument")?;
                    sum = add_u64(sum, argument)?;
                }

                return Ok(Some(sum.into()));
            }

            Expression::Float(mut sum) => {
                while let Some(argument) = arguments.next() {
                    let argument = argument.must_evaluate(&call_site)?.cast_f64("argument")?;
                    sum += argument;
                }

                return Ok(Some(sum.into()));
            }

            Expression::Custom(custom_resource) => {
                let scalar: Scalar = custom_resource.custom().try_into()?;
                let mut sum = scalar.canonical()?;

                while let Some(argument) = arguments.next() {
                    match argument {
                        Expression::Custom(custom_resource) => {
                            let scalar: Scalar = custom_resource.custom().try_into()?;
                            sum = sum.add(scalar.canonical()?, true)?;
                        }

                        _ => return Err(errors::not_of_types_for("argument", &argument, &["scalar"])),
                    }
                }

                let sum = Scalar::new(sum, scalar.schema.canonical_unit.clone(), scalar.schema);
                return Ok(Some(sum.into()));
            }

            _ => {
                return Err(errors::not_of_types_for("argument", &argument, &["integer", "float", "scalar"]));
            }
        }
    }

    panic!("we should never get here if we have an argument");
}
