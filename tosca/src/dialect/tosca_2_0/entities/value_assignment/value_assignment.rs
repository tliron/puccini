use super::super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    std::collections::*,
};

//
// ValueAssignment
//

/// Value assignment.
///
/// For properties, attributes, and parameters.
#[derive(Clone, Debug, Default, Depict)]
pub struct ValueAssignment<AnnotatedT> {
    /// Expression.
    #[depict(option, as(depict))]
    pub expression: Option<Expression<AnnotatedT>>,

    /// Validation.
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// Data type name.
    #[depict(option, as(depict))]
    pub type_name: Option<FullName>,

    /// Metadata.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// Description.
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    // Note: we manage this manually; there's no derive(Resolve)
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

// Used by ArtifactDefinition
impl<AnnotatedT> Subentity<Self> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        complete_optional_parent_type_name_field!(type_name, self, parent, parent_namespace, false, context);

        let Some(parent) = parent else {
            return Ok(());
        };

        complete_namespaced_field!(expression, self, parent, parent_namespace, context);

        let (data_type, _data_type_namespace) =
            completed_entity_from_optional_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            if let Some(parent_data_type) = &parent.type_name {
                validate_type(&data_type, parent_data_type, context)?;
            }

            // if let Some(validation) = &parent.validation {
            //     self.validation.join_apply(validation.to_namespace(parent_namespace));
            //     // TODO: what does inheritance even mean here?
            //     // if self.validation.is_none() {
            //     //     todo!("can we even override this?");
            //     // }
            // } else

            if let Some(validation) =
                give_unwrap!(data_type.schema_validation(parent, parent_namespace, context), &mut context.problems)
            {
                self.validation.join_apply(validation);
            }
        }

        complete_optional_field!(description, self, parent);

        Ok(())
    }
}

// Used by ArtifactAssignment and ArtifactDefinition
impl<AnnotatedT> ToNamespace<Self> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            // expression: self.expression.clone(),
            expression: None,
            validation: None,
            type_name: self.type_name.to_namespace(namespace),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> Annotated for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.expression.as_ref().and_then(|expression| expression.annotations())
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.expression.as_mut().and_then(|expression| expression.annotations_mut())
    }
}

impl<AnnotatedT> AnnotatedStruct for ValueAssignment<AnnotatedT> {
    fn field_annotations(&self, name: &str) -> Option<&Annotations> {
        self.annotations.get(name)
    }

    fn field_annotations_mut(&mut self, name: &str) -> Option<&mut Annotations> {
        self.annotations.get_mut(name)
    }
}

impl<AnnotatedT> Resolve<ValueAssignment<AnnotatedT>> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<ValueAssignment<AnnotatedT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let expression: Option<Expression<_>> = self.resolve_with_problems(problems)?;
        Ok(expression.map(|expression| expression.into()))
    }
}

impl<AnnotatedT> From<Expression<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn from(expression: Expression<AnnotatedT>) -> Self {
        let mut annotations = StructAnnotations::default();

        if AnnotatedT::can_have_annotations()
            && let Some(expression_annotations) = expression.annotations()
        {
            annotations.insert("".into(), expression_annotations.clone());
        }

        Self { expression: Some(expression), annotations, ..Default::default() }
    }
}

//
// ValueAssignments
//

/// Map of [ValueAssignment].
pub type ValueAssignments<AnnotatedT> = BTreeMap<Name, ValueAssignment<AnnotatedT>>;

//
// GetValueAssignment
//

/// Get value assignment.
pub trait GetValueAssignment {
    /// Get value assignment as boolean.
    fn get_boolean_value_assignment(&self, name: &'static str) -> Option<bool>;

    /// Get value assignment as text.
    fn get_text_value_assignment(&self, name: &'static str) -> Option<ByteString>;
}

impl<AnnotatedT> GetValueAssignment for ValueAssignments<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_boolean_value_assignment(&self, name: &'static str) -> Option<bool> {
        if let Some(value_assignment) = self.get(&Name::new_static_unchecked(name))
            && let Some(expression) = &value_assignment.expression
            && let Expression::Simple(simple) = expression
            && let Variant::Boolean(boolean) = simple
        {
            Some(boolean.into())
        } else {
            None
        }
    }

    fn get_text_value_assignment(&self, name: &'static str) -> Option<ByteString> {
        if let Some(value_assignment) = self.get(&Name::new_static_unchecked(name))
            && let Some(expression) = &value_assignment.expression
            && let Expression::Simple(simple) = expression
            && let Variant::Text(text) = simple
        {
            Some(text.inner.clone())
        } else {
            None
        }
    }
}
