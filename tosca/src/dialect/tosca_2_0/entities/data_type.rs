use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
    schema_definition::*,
};

use {
    compris::{annotate::*, depict::*, errors::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    std::collections::*,
};

//
// DataType
//

/// A data type defines the schema for user-defined data types in TOSCA. User-defined data types
/// comprise derived types that derive from from the TOSCA built-in types and complex types that
/// define collections of properties that each have their own data types.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[depict(option, as(depict))]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[depict(option, as(depict))]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The optional validation clause that must evaluate to True for values of this data type to
    /// be valid.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// The optional map property definitions that comprise the schema for a complex data type in
    /// TOSCA.
    #[resolve]
    #[depict(option, iter(kv), as(depict), key_style(string))]
    pub properties: Option<PropertyDefinitions<AnnotatedT>>,

    /// For data types that derive from the TOSCA map data type, the optional schema definition for
    /// the keys used to identify entries in properties of this data type. If not specified, the
    /// key_schema defaults to string. If present, the key_schema must derive from string. For data
    /// types that do not derive from the TOSCA map data type, the key_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<SchemaDefinition<AnnotatedT>>,

    /// For data types that derive from the TOSCA list or map data types, the mandatory schema
    /// definition for the entries in properties of this data type. For data types that do not
    /// derive from the TOSCA list or map data type, the entry_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<SchemaDefinition<AnnotatedT>>,

    /// The data type of the number element of the scalar. Default value if not present is float.
    #[resolve(key = "data_type")]
    #[depict(option, as(depict))]
    pub scalar_data_type: Option<FullName>,

    /// Defines at least one unit string and its associated multiplier. At least one entry MUST
    /// have a multiplier value of one. The multiplier MUST be an integer or a float. If the
    /// data_type is integer then the multiplier MUST be an integer. If prefixes is used then the
    /// map MUST only contain one entry which MUST have a multiplier value of one.
    #[resolve(key = "units")]
    #[depict(option, iter(kv), as(depict), key_style(string))]
    pub scalar_units: Option<BTreeMap<ByteString, Variant<AnnotatedT>>>,

    /// Informs the TOSCA processor which of the possible units to use when storing, computing and
    /// presenting scalars of this type. MUST be present if 'units has more than one multiplier of
    /// one. If not present the unit with multipler of one is the default canonical_unit.
    #[resolve(key = "canonical_unit")]
    #[depict(option, style(string))]
    pub scalar_canonical_unit: Option<ByteString>,

    /// Defines at least one prefix and its associated multiplier. Where prefixes are defined they
    /// are prepended to the unit to obtain the unit string. This keyname is provided as a
    /// convenience so that metric units can use YAML anchor and alias to avoid repeating the table
    /// of SI prefixes.
    #[resolve(key = "prefixes")]
    #[depict(option, iter(kv), as(depict), key_style(string))]
    pub scalar_prefixes: Option<BTreeMap<ByteString, Variant<AnnotatedT>>>,

    /// Data kind.
    #[depict(option, style(symbol))]
    pub data_kind: Option<DataKind>,

    /// Scalar data kind.
    #[depict(option, style(symbol))]
    pub scalar_data_kind: Option<DataKind>,

    /// True if internal.
    #[depict(style(symbol))]
    pub internal: bool,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(DataType);

impl<AnnotatedT> DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_internal(data_kind: DataKind) -> Self {
        Self { data_kind: Some(data_kind), internal: true, ..Default::default() }
    }
}

impl<AnnotatedT> Entity for DataType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let (parent, parent_namespace) =
            completed_entity_checked_from_full_name_field!(DATA_TYPE, self, derived_from, derivation_path, context);

        complete_optional_parent_type_name_field!(scalar_data_type, self, parent, parent_namespace, false, context);

        if let Some(parent) = &parent {
            complete_optional_field!(scalar_canonical_unit, self, parent);
            complete_optional_field!(scalar_units, self, parent);
            complete_optional_field!(scalar_prefixes, self, parent);

            if self.data_kind.is_none() && parent.data_kind.is_some() {
                self.data_kind = parent.data_kind;
            }
        }

        if self.data_kind.is_none() {
            self.data_kind = Some(DataKind::Struct);
        }

        if self.scalar_data_kind.is_none()
            && let Some(data_type) = &self.scalar_data_type
            && let Some((data_type, _source)) = context.catalog.completed_entity::<Self, _>(
                DATA_TYPE,
                data_type,
                context.source_id,
                &mut context.problems.with_fallback_annotations_from_field(self, "data_type"),
            )?
            && data_type.data_kind.is_some()
        {
            self.scalar_data_kind = data_type.data_kind;
        }

        // TODO: units has to map text -> numbers
        // TODO: prefixes has to map text -> numbers
        // TODO: units*prefixes combinations have to be unique (unambiguous)

        if !self.internal
            && let Some(kind) = self.data_kind
        {
            // Mandatory keys

            if matches!(kind, DataKind::List | DataKind::Map) && self.entry_schema.is_none() {
                context.problems.give(
                    MissingRequiredKeyError::as_problem(Variant::<WithoutAnnotations>::from("entry_schema"))
                        .with_annotations_from(self),
                )?;
            }

            if matches!(kind, DataKind::Scalar) {
                if self.scalar_units.is_none() {
                    context.problems.give(
                        MissingRequiredKeyError::as_problem(Variant::<WithoutAnnotations>::from("units"))
                            .with_annotations_from(self),
                    )?;
                }

                if let Some(scalar_data_kind) = self.scalar_data_kind
                    && !matches!(scalar_data_kind, DataKind::Float | DataKind::Integer)
                {
                    context.problems.give(
                        MalformedError::as_problem("data_type", format!("not float or integer: {}", scalar_data_kind))
                            .with_annotations_from_field(self, "data_type"),
                    )?;
                }
            }

            // Invalid keys

            if self.properties.is_some() && !matches!(kind, DataKind::Struct) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("properties"))
                        .with_annotations_from_field(self, "properties"),
                )?;
            }

            if self.key_schema.is_some() && !matches!(kind, DataKind::Map) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("key_schema"))
                        .with_annotations_from_field(self, "key_schema"),
                )?;
            }

            if self.entry_schema.is_some() && !matches!(kind, DataKind::Map | DataKind::List) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("entry_schema"))
                        .with_annotations_from_field(self, "entry_schema"),
                )?;
            }

            if self.scalar_data_type.is_some() && !matches!(kind, DataKind::Scalar) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("data_type"))
                        .with_annotations_from_field(self, "data_type"),
                )?;
            }

            if self.scalar_units.is_some() && !matches!(kind, DataKind::Scalar) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("units"))
                        .with_annotations_from_field(self, "units"),
                )?;
            }

            if self.scalar_canonical_unit.is_some() && !matches!(kind, DataKind::Scalar) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("canonical_unit"))
                        .with_annotations_from_field(self, "canonical_unit"),
                )?;
            }

            if self.scalar_prefixes.is_some() && !matches!(kind, DataKind::Scalar) {
                context.problems.give(
                    InvalidKeyError::as_problem(Variant::<WithoutAnnotations>::from("prefixes"))
                        .with_annotations_from_field(self, "prefixes"),
                )?;
            }
        }

        if let Some(parent) = &parent {
            complete_validation!(self, parent);
        }

        // We complete these last because they may recurse to *this* data type
        // (they will be using the fallback, but in any case we want the errors above to occur
        // first in case of fail-fast)
        complete_subentity_field!(key_schema, self, parent, parent_namespace, context);
        complete_subentity_field!(entry_schema, self, parent, parent_namespace, context);
        complete_optional_subentity_map_field!(property, properties, self, parent, parent_namespace, false, context);

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            derived_from: self.derived_from.to_namespace(namespace),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            validation: self.validation.to_namespace(namespace),
            properties: self.properties.to_namespace(namespace),
            key_schema: self.key_schema.to_namespace(namespace),
            entry_schema: self.entry_schema.to_namespace(namespace),
            scalar_data_type: self.scalar_data_type.to_namespace(namespace),
            scalar_units: self.scalar_units.clone(),
            scalar_canonical_unit: self.scalar_canonical_unit.clone(),
            scalar_prefixes: self.scalar_prefixes.clone(),
            data_kind: self.data_kind,
            scalar_data_kind: self.scalar_data_kind,
            internal: self.internal,
            annotations: self.annotations.clone(),
            completion_state: self.completion_state,
        }
    }
}

//
// DataTypes
//

/// Map of [DataType].
pub type DataTypes<AnnotatedT> = BTreeMap<Name, DataType<AnnotatedT>>;
