use super::{
    super::{
        super::{super::super::grammar::*, data::*},
        data_type::*,
        schema::*,
    },
    macros::*,
};

use compris::annotate::*;

impl<AnnotatedT> DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Initialize scalar schema.
    pub fn initialize_scalar_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        definition: &SchemaDetailsT,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut scalar_schema: ScalarSchema<_> = self.into();
        complete_schema_default_and_validation!(scalar_schema, self, definition);
        Ok(schema.add_unique(reference, scalar_schema.into()))
    }
}

impl<AnnotatedT> Into<ScalarSchema<AnnotatedT>> for &DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into(self) -> ScalarSchema<AnnotatedT> {
        ScalarSchema::new(
            self.scalar_data_kind,
            self.scalar_units.clone().unwrap_or_default(),
            self.scalar_canonical_unit.clone(),
            self.scalar_prefixes.clone().unwrap_or_default(),
        )
    }
}
