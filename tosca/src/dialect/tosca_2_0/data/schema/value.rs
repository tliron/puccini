use super::{
    super::{data_kind::*, expression::*},
    list::*,
    map::*,
    primitive::*,
    reference::*,
    scalar::*,
    r#struct::*,
};

use {compris::annotate::*, depiction::*};

//
// ValueSchema
//

/// Value schema.
#[derive(Clone, Debug, Depict, Eq)]
pub enum ValueSchema<AnnotatedT> {
    /// Reference.
    #[depict(style(number))]
    Reference(SchemaReference),

    /// Primitive.
    #[depict(as(depict))]
    Primitive(PrimitiveSchema<AnnotatedT>),

    /// Scalar.
    #[depict(as(depict))]
    Scalar(ScalarSchema<AnnotatedT>),

    /// List.
    #[depict(as(depict))]
    List(ListSchema<AnnotatedT>),

    /// Map.
    #[depict(as(depict))]
    Map(MapSchema<AnnotatedT>),

    /// Struct.
    #[depict(as(depict))]
    Struct(StructSchema<AnnotatedT>),
}

impl<AnnotatedT> ValueSchema<AnnotatedT> {
    /// Data kind.
    pub fn data_kind(&self) -> Option<DataKind> {
        match self {
            Self::Reference(_) => None,
            Self::Primitive(primitive) => Some(primitive.data_kind),
            Self::Scalar(_) => Some(DataKind::Scalar),
            Self::List(_) => Some(DataKind::List),
            Self::Map(_) => Some(DataKind::Map),
            Self::Struct(_) => Some(DataKind::Struct),
        }
    }

    /// Update reference.
    pub fn update_reference(&mut self, old: SchemaReference, new: SchemaReference) {
        match self {
            Self::Reference(reference) => {
                if *reference == old {
                    *reference = new;
                }
            }

            Self::List(list) => list.update_reference(old, new),
            Self::Map(map) => map.update_reference(old, new),
            Self::Struct(struct_) => struct_.update_reference(old, new),

            _ => {}
        }
    }

    /// Into expression.
    pub fn into_expression(self, positions: &SchemaReferencePositions) -> Expression<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match self {
            // The only schema that can be represented as an unsigned integer
            Self::Reference(reference) => positions.expression(reference),

            // The only schema that can be represented as text
            Self::Primitive(primitive) => primitive.into(),

            Self::Scalar(scalar) => scalar.into(),
            Self::List(list) => list.into_expression(positions),
            Self::Map(map) => map.into_expression(positions),
            Self::Struct(struct_) => struct_.into_expression(positions),
        }
    }
}

impl<AnnotatedT> PartialEq for ValueSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(primitive), Self::Primitive(other_primitive)) => primitive == other_primitive,
            (Self::Scalar(scalar), Self::Scalar(other_scalar)) => scalar == other_scalar,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            (Self::Struct(struct_), Self::Struct(other_struct)) => struct_ == other_struct,
            _ => false,
        }
    }
}

impl<AnnotatedT> From<PrimitiveSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: PrimitiveSchema<AnnotatedT>) -> Self {
        Self::Primitive(schema)
    }
}

impl<AnnotatedT> From<ScalarSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: ScalarSchema<AnnotatedT>) -> Self {
        Self::Scalar(schema)
    }
}

impl<AnnotatedT> From<ListSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: ListSchema<AnnotatedT>) -> Self {
        Self::List(schema)
    }
}

impl<AnnotatedT> From<MapSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: MapSchema<AnnotatedT>) -> Self {
        Self::Map(schema)
    }
}

impl<AnnotatedT> From<StructSchema<AnnotatedT>> for ValueSchema<AnnotatedT> {
    fn from(schema: StructSchema<AnnotatedT>) -> Self {
        Self::Struct(schema)
    }
}

impl<AnnotatedT> From<DataKind> for ValueSchema<AnnotatedT> {
    fn from(data_kind: DataKind) -> Self {
        Self::Primitive(data_kind.into())
    }
}

impl<AnnotatedT> From<SchemaReference> for ValueSchema<AnnotatedT> {
    fn from(reference: SchemaReference) -> Self {
        Self::Reference(reference)
    }
}
