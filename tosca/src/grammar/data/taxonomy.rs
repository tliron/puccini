use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::std::error::*,
    std::{slice, vec},
};

//
// Taxonomy
//

/// A sequence of name-value pairs.
///
/// A name-value pair is a bit like a key-value pair in a map, except that in a map the keys are
/// unique. Here names do not have that requirement, i.e. multiple values may have the same name.
/// Thus the only requirement for names is that they implement [PartialEq].
///
/// A taxonomy can be resolved from a [List] in which each item is a [Map] with a single key, which
/// is the name for the value. In TOSCA this notation has sometimes been called a "sequenced list".
#[derive(Clone, Debug)]
pub struct Taxonomy<NameT, ValueT>(pub Vec<(NameT, ValueT)>);

impl<NameT, ValueT> Taxonomy<NameT, ValueT> {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add a name-value pair.
    pub fn add(&mut self, name: NameT, value: ValueT) {
        self.0.push((name, value))
    }

    /// Iterate.
    pub fn iter(&self) -> slice::Iter<'_, (NameT, ValueT)> {
        self.0.iter()
    }
}

impl<NameT, ValueT> Taxonomy<NameT, ValueT>
where
    NameT: PartialEq,
{
    /// True if contains the name at least once.
    pub fn contains_name(&self, name: &NameT) -> bool {
        for (our_name, _) in &self.0 {
            if our_name == name {
                return true;
            }
        }
        false
    }

    /// Gets the value of the first occurrence of the name.
    pub fn first(&self, name: &NameT) -> Option<&ValueT> {
        for (our_name, value) in &self.0 {
            if our_name == name {
                return Some(value);
            }
        }
        None
    }

    /// Gets the value of the first occurrence of the name.
    pub fn first_mut(&mut self, name: &NameT) -> Option<&mut ValueT> {
        for (our_name, value) in &mut self.0 {
            if our_name == name {
                return Some(value);
            }
        }
        None
    }
}

impl<NameT, ValueT, AnnotatedT> Resolve<Taxonomy<NameT, ValueT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
    Variant<AnnotatedT>: Resolve<NameT, AnnotatedT> + Resolve<ValueT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorReceiverT>(
        self,
        errors: &mut ErrorReceiverT,
    ) -> ResolveResult<Taxonomy<NameT, ValueT>, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
    {
        let mut resolved = Vec::default();

        // List of single-key maps
        match self {
            Self::List(list) => {
                for item in list {
                    match item {
                        Self::Map(map) => {
                            if map.inner.len() == 1 {
                                let pair = map.into_key_value_pair().expect("single-key map");
                                if let Some((key, value)) = pair.resolve_with_errors(errors)? {
                                    resolved.push((key, value));
                                }
                            } else {
                                errors.give(
                                    MalformedError::new(
                                        "name-value pair".into(),
                                        "is not a map with a single key".into(),
                                    )
                                    .with_annotations_from(&map),
                                )?;
                            }
                        }

                        _ => errors.give(IncompatibleVariantTypeError::new_from(&item, &["map"]))?,
                    }
                }
            }

            _ => errors.give(IncompatibleVariantTypeError::new_from(&self, &["list"]))?,
        }

        Ok(Some(Taxonomy(resolved)))
    }
}

impl<NameT, ValueT> Default for Taxonomy<NameT, ValueT> {
    fn default() -> Self {
        Self(Default::default())
    }
}

// Delegated

impl<NameT, ValueT> IntoIterator for Taxonomy<NameT, ValueT> {
    type Item = (NameT, ValueT);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'own, NameT, ValueT> IntoIterator for &'own Taxonomy<NameT, ValueT> {
    type Item = &'own (NameT, ValueT);
    type IntoIter = slice::Iter<'own, (NameT, ValueT)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'own, NameT, ValueT> IntoIterator for &'own mut Taxonomy<NameT, ValueT> {
    type Item = &'own mut (NameT, ValueT);
    type IntoIter = slice::IterMut<'own, (NameT, ValueT)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<NameT, ValueT> FromIterator<(NameT, ValueT)> for Taxonomy<NameT, ValueT> {
    fn from_iter<IteratorT>(iterator: IteratorT) -> Self
    where
        IteratorT: IntoIterator<Item = (NameT, ValueT)>,
    {
        Self(iterator.into_iter().collect())
    }
}

// Conversions

impl<NameT, ValueT> From<Taxonomy<NameT, ValueT>> for Vec<(NameT, ValueT)> {
    fn from(value: Taxonomy<NameT, ValueT>) -> Self {
        value.0
    }
}
