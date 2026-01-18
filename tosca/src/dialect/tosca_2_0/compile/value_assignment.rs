use super::super::{super::super::grammar::*, dialect::*, entities::*};

use {compris::annotate::*, kutil::std::immutable::*, problemo::*, std::collections::*};

impl<AnnotatedT> ValueAssignment<AnnotatedT> {
    /// Compile to Floria.
    pub fn compile(
        &self,
        tosca_entity: &'static str,
        read_only: bool,
        context: &mut CompilationContext,
    ) -> Result<floria::Property, Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let (preparer, updater, value) = self.floria_property_fields(context)?;
        let mut floria_property = floria::Property::new(read_only, preparer, updater, value);

        floria_property.metadata.set_tosca_entity_static(DIALECT_ID, tosca_entity);
        floria_property.metadata.set_tosca_description(self.description.as_ref());
        floria_property.metadata.set_tosca_custom_metadata(&self.metadata);

        if let Some(data_type) = &self.type_name {
            floria_property.class_ids.add_tosca_type(DATA_TYPE, DATA_TYPE_NAME, data_type, context)?;
        }

        Ok(floria_property)
    }

    /// Floria property preparer, updater, and value.
    pub fn floria_property_fields(
        &self,
        context: &mut CompilationContext,
    ) -> Result<(Option<floria::Expression>, Option<floria::Expression>, Option<floria::Expression>), Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let preparer = match &self.validation {
            Some(validation) => validation.clone().into_eager().compile(context).give_ok(&mut context.problems)?,
            None => None,
        };

        let (updater, value) = match &self.expression {
            Some(expression) => {
                match expression.clone().into_eager().compile(context).give_ok(&mut context.problems)? {
                    Some(expression) => {
                        if expression.is_literal() {
                            (None, Some(expression))
                        } else {
                            (Some(expression), None)
                        }
                    }

                    None => (None, None),
                }
            }

            None => (None, None),
        };

        Ok((preparer, updater, value))
    }
}

/// Compile value assignments.
pub fn compile_value_assignments<AnnotatedT>(
    property_templates: &mut BTreeMap<ByteString, floria::Property>,
    value_assignments: &ValueAssignments<AnnotatedT>,
    prefix: &'static str,
    tosca_entity: &'static str,
    read_only: bool,
    context: &mut CompilationContext,
) -> Result<(), Problem>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    for (name, value_assignment) in value_assignments {
        let name = if prefix.is_empty() { name.clone().into() } else { format!("{}:{}", prefix, name).into() };
        property_templates.insert(name, value_assignment.compile(tosca_entity, read_only, context)?);
    }
    Ok(())
}
