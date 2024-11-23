use super::super::super::data::*;

use floria_plugin_sdk::{data::*, errors, utils::*};

/// The $product function takes either:
///
/// * Two arguments where the first argument is of a scalar type and the second argument is of an
///   integer or float type. The result is of the same type as the first argument and its value is
///   the arithmetic product of the first argument value and the second argument value.
///
/// * Any number of arguments of type integer or float. If all inputs are of type integer, then the
///   result is of type integer, otherwise it is of type float. The result value is the arithmetic
///   product of all the arguments values.
pub fn product(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_min(&arguments, 1)?;
    let length = arguments.len();
    let mut arguments = arguments.into_iter();

    while let Some(argument) = arguments.next() {
        let argument = argument.must_evaluate(&call_site)?;
        match argument {
            Expression::Custom(custom_resource) => {
                let scalar: Scalar = custom_resource.custom().try_into()?;
                let mut product = scalar.canonical()?;

                match arguments.next() {
                    Some(right) => {
                        if arguments.next().is_some() {
                            return Err(errors::arguments_exact(2, length));
                        }

                        let right = right.must_evaluate(&call_site)?;
                        let right: Number = right.try_into()?;
                        product = product.mul(right, false)?;

                        // TODO to int or float

                        let product = Scalar::new(product, scalar.schema.canonical_unit.clone(), scalar.schema);
                        return Ok(Some(product.into()));
                    }

                    None => {
                        return Err(errors::arguments_exact(2, 1));
                    }
                }
            }

            _ => {
                let mut product: Number = argument.try_into()?;

                while let Some(argument) = arguments.next() {
                    let argument = argument.must_evaluate(&call_site)?;
                    let argument: Number = argument.try_into()?;
                    product = product.mul(argument, false)?;
                }

                return Ok(Some(product.into()));
            }
        }
    }

    panic!("we should never get here if we have an argument");
}
