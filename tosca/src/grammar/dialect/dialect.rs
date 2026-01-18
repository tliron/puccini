use super::{
    super::{compile::*, source::*},
    implementation::*,
};

use {
    compris::{annotate::*, normal::*},
    problemo::*,
    std::any::*,
};

//
// Dialect
//

/// Dialect.
pub trait Dialect
where
    Self: Any,
{
    /// Implementation.
    fn implementation(&self) -> &DialectImplementation;

    /// Initialize a source with annotations.
    fn initialize_source_with_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithAnnotations>,
        problems: ProblemReceiverRef,
    ) -> Result<(), Problem>;

    /// Initialize a source without annotations.
    fn initialize_source_without_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithoutAnnotations>,
        problems: ProblemReceiverRef,
    ) -> Result<(), Problem>;

    /// Compile a source representing a TOSCA service template to a Floria
    /// [VertexTemplate](floria::VertexTemplate).
    ///
    /// Though only one Floria ID is returned, the implementation may create other Floria entities.
    fn compile_source_with_annotations(&self, context: &mut CompilationContext) -> Result<Option<floria::ID>, Problem>;

    /// Compile a source representing a TOSCA service template to a Floria
    /// [VertexTemplate](floria::VertexTemplate).
    ///
    /// Though only one Floria ID is returned, the implementation may create other Floria entities.
    fn compile_source_without_annotations(
        &self,
        context: &mut CompilationContext,
    ) -> Result<Option<floria::ID>, Problem>;
}
