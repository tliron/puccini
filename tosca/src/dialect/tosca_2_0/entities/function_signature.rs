use super::{super::super::super::grammar::*, implementation_definition::*, schema_definition::*};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
};

//
// FunctionSignature
//

/// Function signatures can be defined in TOSCA profiles or TOSCA service templates using a YAML
/// map under the functions keyname using the grammar specified below. Note that this grammar
/// allows the definition of functions that have arguments expressed within a YAML seq, however
/// intrinsic functions may accept other argument definition syntaxes.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct FunctionSignature<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// All defined arguments must be used in the function invocation (and in the order defined
    /// here). If no arguments are defined, the signature either accepts no arguments or any
    /// arguments of any form (depending on if the variadic keyname is false or true).
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub arguments: Vec<SchemaDefinition<AnnotatedT>>,

    /// Optional arguments may be used in the function invocation after the regular arguments.
    /// Still the order defined here must be respected.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub optional_arguments: Vec<SchemaDefinition<AnnotatedT>>,

    /// Specifies if the last defined argument (or optional_arguments if defined) may be repeated
    /// any number of times in the function invocation. If this value is not specified, a default
    /// of False is assumed.
    #[resolve]
    #[depict(style(symbol))]
    pub variadic: bool,

    /// Defines the type of the function result. If no result keyname is defined, then the function
    /// may return any result.
    #[resolve]
    #[depict(option, as(depict))]
    pub result: Option<SchemaDefinition<AnnotatedT>>,

    /// Defines the implementation (e.g., artifact) for the function. The same definition as for
    /// operation/notification implementation is used.
    #[resolve]
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> FunctionSignature<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_internal(namespace: Namespace, artifact_file: ByteString) -> Self {
        Self {
            implementation: Some(ImplementationDefinition::new_internal(namespace, artifact_file)),
            variadic: true,
            ..Default::default()
        }
    }
}

impl<AnnotatedT> Subentity<Self> for FunctionSignature<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        for argument in &mut self.arguments {
            argument.complete(None, None, parent_namespace, context)?;
        }

        for argument in &mut self.optional_arguments {
            argument.complete(None, None, parent_namespace, context)?;
        }

        complete_subentity_field!(result, self, parent, parent_namespace, context);

        // TODO: complete by not allowing names, only definitions
        complete_subentity_field!(implementation, self, parent, parent_namespace, context);

        Ok(())
    }
}

//
// FunctionSignatures
//

/// Map of [FunctionSignature].
pub type FunctionSignatures<AnnotatedT> = Vec<FunctionSignature<AnnotatedT>>;
