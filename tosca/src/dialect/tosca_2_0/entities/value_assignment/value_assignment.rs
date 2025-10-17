use super::super::{
    super::{super::super::grammar::*, data::*, dialect::*, schema::*},
    data_type::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::{error::*, immutable::*},
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
        _name: Option<ByteString>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some(parent) = parent else {
            return Ok(());
        };

        complete_optional_parent_name_field!(type_name, parent_namespace, self, Some(parent), context);
        complete_none_field!(expression, self, parent);

        if let Some(type_name) = &self.type_name
            && let Some(data_type) = context
                .catalog
                .completed_entity::<DataType<AnnotatedT>, _, _>(
                    DATA_TYPE,
                    type_name,
                    context.source_id,
                    &mut context.errors.with_fallback_annotations_from_field(self, "type_name"),
                )?
                .cloned()
        {
            if let Some(parent_data_type) = &parent.type_name {
                validate_type(&data_type, parent_data_type, context)?;
            }

            if let Some(validation) = &parent.validation {
                // TODO: what does inheritance even mean here?
                self.validation = Some(validation.clone());
            } else if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(
                    &self.to_schema_key(parent.type_name.clone()),
                    parent,
                    context.source_id,
                    context.catalog
                ),
                context.errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_none_field!(description, self, parent);

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
            expression: self.expression.clone(),
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

impl<AnnotatedT> Resolve<ValueAssignment<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorReceiverT>(
        self,
        errors: &mut ErrorReceiverT,
    ) -> ResolveResult<ValueAssignment<AnnotatedT>, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
    {
        let expression: Option<Expression<_>> = self.resolve_with_errors(errors)?;
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
pub type ValueAssignments<AnnotatedT> = BTreeMap<ByteString, ValueAssignment<AnnotatedT>>;
