use super::super::super::super::grammar::{Dialect as DialectTrait, *};

use {
    compris::{annotate::*, normal::*},
    kutil::std::immutable::*,
    problemo::*,
};

/// Dialect ID.
pub const DIALECT_ID: DialectID = DialectID::from_static("tosca_2_0");

/// Plugin name.
pub const PLUGIN_NAME: ByteString = ByteString::from_static("tosca:2.0");

/// Plugin URL.
pub const PLUGIN_URL: ByteString = ByteString::from_static("implicit:tosca:2.0");

//
// Dialect
//

/// TOSCA 2.0 dialect.
#[derive(Clone, Debug)]
pub struct Dialect {
    /// Implementation.
    pub implementation: DialectImplementation,
}

impl Default for Dialect {
    fn default() -> Self {
        Self { implementation: DialectImplementation::new(DIALECT_ID, Self::entity_kinds()) }
    }
}

impl DialectTrait for Dialect {
    fn dialect_id(&self) -> DialectID {
        self.implementation.dialect_id()
    }

    fn entity_kinds(&self) -> &EntityKinds {
        &self.implementation.entity_kinds
    }

    fn initialize_source_with_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithAnnotations>,
        mut problems: ProblemReceiverRef,
    ) -> Result<(), Problem> {
        self.initialize_source(source, variant, &mut problems)
    }

    fn initialize_source_without_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithoutAnnotations>,
        mut problems: ProblemReceiverRef,
    ) -> Result<(), Problem> {
        self.initialize_source(source, variant, &mut problems)
    }

    fn compile_source_with_annotations(&self, context: &mut CompilationContext) -> Result<Option<floria::ID>, Problem> {
        self.compile_service_template::<WithAnnotations>(context)
    }

    fn compile_source_without_annotations(
        &self,
        context: &mut CompilationContext,
    ) -> Result<Option<floria::ID>, Problem> {
        self.compile_service_template::<WithoutAnnotations>(context)
    }
}
