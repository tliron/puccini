use super::super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
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
impl<AnnotatedT> Subentity<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        scope: Option<&Scope>,
        parent: Option<&Self>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some(parent) = parent else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        complete_name_field_both_option!(type_name, scope, self, Some(parent), catalog, errors);
        //complete_field_none_to!(type_name, self, parent, parent.type_name.into_scoped(scope));
        complete_field_none!(expression, self, parent);

        if let Some(type_name) = &self.type_name
            && let Some(data_type) = catalog
                .completed_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, type_name, source_id, errors)?
                .cloned()
        {
            if let Some(parent_data_type) = &parent.type_name {
                validate_type(&data_type, parent_data_type, catalog, errors)?;
            }

            if let Some(validation) = &parent.validation {
                // TODO: what does inheritance even mean here?
                self.validation = Some(validation.clone());
            } else if let Some(validation) =
                unwrap_or_give!(data_type.schema_validation(type_name, parent, source_id, catalog), errors, None)
            {
                self.validation.join_apply(validation);
            }
        }

        complete_field_none!(description, self, parent);

        Ok(())
    }
}

// Used by ArtifactAssignment and ArtifactDefinition
impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> Self {
        Self {
            expression: self.expression.clone(),
            validation: None,
            type_name: self.type_name.into_scoped(scope),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> AnnotatedStruct for ValueAssignment<AnnotatedT> {
    fn field_annotations(&self, name: &str) -> Option<&Annotations> {
        self.annotations.get(name)
    }
}

impl<AnnotatedT> Resolve<ValueAssignment<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<ValueAssignment<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
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

//
// ValueAssignments
//

/// Map of [ValueAssignment].
pub type ValueAssignments<AnnotatedT> = BTreeMap<ByteString, ValueAssignment<AnnotatedT>>;
