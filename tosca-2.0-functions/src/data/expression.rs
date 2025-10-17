use super::{comparator::*, kind::*, scalar::*, timestamp::*, version::*};

use {
    base64::prelude::*,
    floria_plugin_sdk::{data::*, utils::*, *},
};

//
// ExpressionUtilities
//

/// Expression utilities.
pub trait ExpressionUtilities: Sized {
    /// Coerce into a data kind.
    ///
    /// Returns true if the expression was modified.
    fn maybe_coerce(self, data_kind: &str) -> Result<(Expression, bool), DispatchError>;

    /// Coerce into a data kind.
    fn must_coerce(self, data_kind: &str) -> Result<Expression, DispatchError> {
        self.maybe_coerce(data_kind).map(|(expression, _modified)| expression)
    }

    /// If the other expression is custom, coerce into its data kind.
    ///
    /// This can be called before [comparator](ExpressionUtilities::comparator) in order to ensure
    /// compatibility between two expressions.
    fn coerce_if_custom(self, other: &Expression) -> Result<Expression, DispatchError>;

    /// Comparator.
    fn comparator(self) -> Result<Expression, DispatchError>;
}

impl ExpressionUtilities for Expression {
    fn maybe_coerce(self, data_kind: &str) -> Result<(Expression, bool), DispatchError> {
        match data_kind {
            STRING_DATA_KIND => {
                if matches!(self, Expression::Text(_)) {
                    return Ok((self, false));
                }
            }

            INTEGER_DATA_KIND => {
                if matches!(self, Expression::Integer(_) | Expression::UnsignedInteger(_)) {
                    return Ok((self, false));
                }
            }

            FLOAT_DATA_KIND => {
                if matches!(self, Expression::Float(_)) {
                    return Ok((self, false));
                }
            }

            BOOLEAN_DATA_KIND => {
                if matches!(self, Expression::Boolean(_)) {
                    return Ok((self, false));
                }
            }

            BYTES_DATA_KIND => match self {
                // Note that blob expressions cannot be created directly from TOSCA
                Expression::Blob(_) => return Ok((self, false)),

                Expression::Text(text) => {
                    let blob = BASE64_STANDARD.decode(text.as_bytes()).map_err(|_| "|name|bytes| not valid Base64")?;
                    return Ok((blob.into(), true));
                }

                _ => return Err(format!("|name|bytes| not valid Base64: |error|{}|", self.type_name())),
            },

            NIL_DATA_KIND => {
                if matches!(self, Expression::Null) {
                    return Ok((self, false));
                }
            }

            LIST_DATA_KIND => {
                if matches!(self, Expression::List(_)) {
                    return Ok((self, false));
                }
            }

            MAP_DATA_KIND => {
                if matches!(self, Expression::Map(_)) {
                    return Ok((self, false));
                }
            }

            TIMESTAMP_DATA_KIND => {
                let timestamp: Timestamp = self.try_into()?;
                return Ok((timestamp.into(), true));
            }

            SCALAR_DATA_KIND => {
                // Note: The try_into here only supports Expression::Custom
                // You need to coerce_custom for Expression::Text!
                // (because text doesn't have the scalar schema)
                let scalar: Scalar = self.try_into()?;
                return Ok((scalar.into(), true));
            }

            VERSION_DATA_KIND => {
                let version: Version = self.try_into()?;
                return Ok((version.into(), true));
            }

            _ => return Err(format!("unsupported data kind: |error|{}|", escape_depiction_markup(data_kind))),
        }

        Err(format!("not |name|{}|: |error|{}|", data_kind, self.type_name()))
    }

    fn coerce_if_custom(self, other: &Expression) -> Result<Expression, DispatchError> {
        if let Expression::Custom(other) = other
            && !matches!(self, Expression::Custom(_))
        {
            let custom = other.custom();
            match custom.kind.as_str() {
                SCALAR_CUSTOM_KIND => {
                    let scalar: Scalar = custom.try_into()?;
                    let scalar = Scalar::new_from_expression(self, &scalar.schema)?;
                    Ok(scalar.into())
                }

                TIMESTAMP_CUSTOM_KIND => {
                    let timestamp: Timestamp = custom.try_into()?;
                    Ok(timestamp.into())
                }

                VERSION_CUSTOM_KIND => {
                    let version: Version = custom.try_into()?;
                    Ok(version.into())
                }

                kind => Err(format!("unsupported custom data type: |error|{}|", escape_depiction_markup(kind))),
            }
        } else {
            Ok(self)
        }
    }

    fn comparator(self) -> Result<Expression, DispatchError> {
        match self {
            Expression::Integer(_) | Expression::UnsignedInteger(_) | Expression::Float(_) | Expression::Text(_) => {
                Ok(self)
            }

            Expression::Custom(custom) => {
                let custom = custom.custom();
                match custom.kind.as_str() {
                    SCALAR_CUSTOM_KIND => {
                        let scalar: Scalar = custom.try_into()?;
                        scalar.comparator()
                    }

                    TIMESTAMP_CUSTOM_KIND => {
                        let timestamp: Timestamp = custom.try_into()?;
                        timestamp.comparator()
                    }

                    VERSION_CUSTOM_KIND => {
                        let version: Version = custom.try_into()?;
                        version.comparator()
                    }

                    kind => Err(format!("unsupported custom data type: |error|{}|", escape_depiction_markup(kind))),
                }
            }

            _ => Err(format!("not comparable: |error|{}|", self.type_name())),
        }
    }
}
