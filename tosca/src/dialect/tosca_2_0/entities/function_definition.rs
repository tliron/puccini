use super::{super::super::super::grammar::*, function_signature::*};

use {
    compris::{annotate::*, depict::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    std::collections::*,
};

//
// FunctionDefinition
//

/// TOSCA allows for the use of custom functions that extend the set of built-in functions
/// documented in the previous section. TOSCA Processors use standard function parsing rules to
/// detect the presence of a custom function.
///
/// In addition, TOSCA also includes grammar for defining function signatures and associated
/// implementation artifacts in TOSCA profiles or in TOSCA service templates. This allows for
/// validation of function return values and function arguments at design time, and the possibility
/// to provide function implementation artifacts within CSARs. Note that the use of custom function
/// definitions is entirely optional. Service designers can use custom functions without defining
/// associated function signatures and instead rely on support for those functions directly in the
/// TOSCA orchestrator that will be used to process the TOSCA files. Of course, TOSCA processors
/// may support custom functions that are not user-defined.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct FunctionDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The map of signature definitions.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub signatures: FunctionSignatures<AnnotatedT>,

    /// The description of the function.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// True if internal.
    #[depict(style(symbol))]
    pub internal: bool,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl<AnnotatedT> FunctionDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_internal(namespace: Namespace, artifact_file: ByteString) -> Self {
        Self {
            signatures: vec![FunctionSignature::new_internal(namespace, artifact_file)],
            internal: true,
            ..Default::default()
        }
    }
}

impl<AnnotatedT> Entity for FunctionDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        _derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        for signature in &mut self.signatures {
            signature.complete(None, None, None, context)?;
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// FunctionDefinitions
//

/// Map of [FunctionDefinition].
pub type FunctionDefinitions<AnnotatedT> = BTreeMap<Name, FunctionDefinition<AnnotatedT>>;
