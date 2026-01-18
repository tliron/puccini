use {
    compris::{annotate::*, errors::*, normal::*, resolve::*},
    problemo::*,
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

impl<NameT, ValueT, AnnotatedT> Resolve<Taxonomy<NameT, ValueT>> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
    Variant<AnnotatedT>: Resolve<NameT> + Resolve<ValueT>,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<Taxonomy<NameT, ValueT>>
    where
        ProblemReceiverT: ProblemReceiver,
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
                                if let Some((key, value)) = pair.resolve_with_problems(problems)? {
                                    resolved.push((key, value));
                                }
                            } else {
                                problems.give(
                                    MalformedError::as_problem("name-value pair", "is not a map with a single key")
                                        .with_annotations_from(&map),
                                )?;
                            }
                        }

                        _ => problems.give(
                            IncompatibleVariantTypeError::as_problem_from(&item, &["map"]).with_annotations_from(&item),
                        )?,
                    }
                }
            }

            _ => problems
                .give(IncompatibleVariantTypeError::as_problem_from(&self, &["list"]).with_annotations_from(&self))?,
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

impl<'this, NameT, ValueT> IntoIterator for &'this Taxonomy<NameT, ValueT> {
    type Item = &'this (NameT, ValueT);
    type IntoIter = slice::Iter<'this, (NameT, ValueT)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'this, NameT, ValueT> IntoIterator for &'this mut Taxonomy<NameT, ValueT> {
    type Item = &'this mut (NameT, ValueT);
    type IntoIter = slice::IterMut<'this, (NameT, ValueT)>;

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
