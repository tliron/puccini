use super::super::{data::*, dialect::*, errors::*, name::*};

use {
    compris::{annotate::*, normal::*},
    kutil::std::{error::*, immutable::*},
};

//
// FloriaToscaMetadata
//

/// Set Floria metadata for TOSCA.
pub trait FloriaToscaMetadata {
    /// Set TOSCA metadata.
    fn set_tosca_metadata<ValueT>(&mut self, key: &'static str, value: ValueT)
    where
        ValueT: Into<Variant<WithoutAnnotations>>;

    /// Set TOSCA `dialect` and `kind` metadata.
    fn set_tosca_entity(&mut self, dialect: DialectID, entity_kind: ByteString);

    /// Set TOSCA `dialect` and `kind` metadata.
    fn set_tosca_entity_static(&mut self, dialect: DialectID, entity_kind: &'static str);

    /// Set TOSCA `name` metadata.
    fn set_tosca_name(&mut self, name: ByteString);

    /// Set TOSCA `description` metadata.
    fn set_tosca_description(&mut self, description: Option<&ByteString>);

    /// Merge in the TOSCA [Metadata].
    fn set_tosca_custom_metadata<AnnotatedT>(&mut self, from_metadata: &Metadata<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default;

    /// Set TOSCA `internal` metadata.
    fn set_tosca_internal(&mut self, internal: bool);

    /// Set TOSCA `version` metadata.
    fn set_tosca_version(&mut self, version: Option<&ByteString>);

    /// Set TOSCA `directives` metadata.
    fn set_tosca_directives(&mut self, directives: &Vec<ByteString>);
}

impl FloriaToscaMetadata for floria::Metadata {
    fn set_tosca_metadata<ValueT>(&mut self, key: &'static str, value: ValueT)
    where
        ValueT: Into<Variant<WithoutAnnotations>>,
    {
        if let Some(metadata) = self.into_get_mut("tosca")
            && let Variant::Map(metadata) = metadata
        {
            metadata.into_insert(key, value);
        } else {
            let mut metadata = Map::default();
            metadata.into_insert(key, value);
            self.into_insert("tosca", metadata);
        }
    }

    fn set_tosca_entity(&mut self, dialect: DialectID, entity_kind: ByteString) {
        self.set_tosca_metadata("dialect", dialect);
        self.set_tosca_metadata("kind", entity_kind);
    }

    fn set_tosca_entity_static(&mut self, dialect: DialectID, entity_kind: &'static str) {
        self.set_tosca_entity(dialect, ByteString::from_static(entity_kind));
    }

    fn set_tosca_name(&mut self, name: ByteString) {
        self.set_tosca_metadata("name", name);
    }

    fn set_tosca_description(&mut self, description: Option<&ByteString>) {
        if let Some(description) = description
            && !description.is_empty()
        {
            self.set_tosca_metadata("description", description.clone());
        }
    }

    fn set_tosca_custom_metadata<AnnotatedT>(&mut self, from_metadata: &Metadata<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        if !from_metadata.is_empty() {
            let metadata: Variant<WithoutAnnotations> =
                from_metadata.iter().map(|(key, value)| (key.clone().into(), value.clone().into_annotated())).collect();
            self.set_tosca_metadata("custom", metadata);
        }
    }

    fn set_tosca_internal(&mut self, internal: bool) {
        self.set_tosca_metadata("internal", internal);
    }

    fn set_tosca_version(&mut self, version: Option<&ByteString>) {
        if let Some(version) = version {
            self.set_tosca_metadata("version", version.clone());
        }
    }

    fn set_tosca_directives(&mut self, directives: &Vec<ByteString>) {
        if !directives.is_empty() {
            let directives: Vec<Variant<_>> =
                directives.into_iter().map(|directive| directive.clone().into()).collect();
            self.set_tosca_metadata("directives", directives);
        }
    }
}

//
// FloriaToscaType
//

/// Add TOSCA type and its ancestors as Floria classes.
pub trait FloriaToscaType {
    /// Add TOSCA type and its ancestors as Floria classes.
    fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
        &mut self,
        type_name: &FullName,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl FloriaToscaType for Vec<floria::ID> {
    fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
        &mut self,
        type_name: &FullName,
        floria_directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut id =
            floria::ID::new_for(floria::EntityKind::Class, floria_directory.clone(), type_name.to_string().into());

        loop {
            match unwrap_or_give_and_return!(store.get_class(&id), errors, Ok(())) {
                Some(class) => {
                    self.push(class.id.clone());
                    match class.metadata.inner.get(&"tosca:parent".into()) {
                        Some(parent) => id.name = parent.to_string().into(),
                        None => break,
                    }
                }

                None => {
                    // TODO
                    break;
                }
            }
        }

        Ok(())
    }
}
