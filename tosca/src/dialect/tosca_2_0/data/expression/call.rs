use super::{super::super::super::super::grammar::*, expression::*};

use {
    compris::annotate::*,
    depiction::*,
    kutil::std::iter::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug)]
pub struct Call<AnnotatedT> {
    /// Function name.
    pub function: FullName,

    /// Arguments.
    pub arguments: Vec<Expression<AnnotatedT>>,

    /// Kind.
    pub kind: floria::CallKind,

    annotated: AnnotatedT,
}

impl_annotated!(Call);

impl<AnnotatedT> Call<AnnotatedT> {
    /// Constructor.
    pub fn new(function: FullName, arguments: Vec<Expression<AnnotatedT>>, kind: floria::CallKind) -> Self
    where
        AnnotatedT: Default,
    {
        Self { function, arguments, kind, annotated: Default::default() }
    }

    /// Constructor.
    pub fn new_native(function: &'static str, arguments: Vec<Expression<AnnotatedT>>, kind: floria::CallKind) -> Self
    where
        AnnotatedT: Default,
    {
        Self::new(Name::from_static(function).into(), arguments, kind)
    }

    /// True if native.
    pub fn is_native(&self) -> bool {
        // TODO
        self.function.namespace.is_empty()
    }

    /// True if we already have the argument.
    pub fn has_argument(&self, expression: &Expression<AnnotatedT>) -> bool {
        self.arguments.iter().any(|argument| argument == expression)
    }

    /// Prepend argument if we don't already have it
    pub fn prepend_unique_argument(&mut self, argument: Expression<AnnotatedT>) {
        if !self.has_argument(&argument) {
            self.arguments.insert(0, argument);
        }
    }

    /// Append argument if we don't already have it
    pub fn append_unique_argument(&mut self, argument: Expression<AnnotatedT>) {
        if !self.has_argument(&argument) {
            self.arguments.push(argument);
        }
    }

    /// Make it lazy.
    pub fn make_lazy(&mut self) {
        self.kind = floria::CallKind::Lazy;
    }
}

impl<AnnotatedT> RemoveAnnotations<Call<WithoutAnnotations>> for &Call<AnnotatedT>
where
    AnnotatedT: Clone,
{
    fn remove_annotations(self) -> Call<WithoutAnnotations> {
        Call::new(
            self.function.clone(),
            self.arguments.iter().map(|item| item.remove_annotations()).collect(),
            self.kind,
        )
    }
}

impl<AnnotatedT> Depict for Call<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;

        match self.kind {
            floria::CallKind::Eager => context.theme.write_delimiter(writer, '*')?,
            floria::CallKind::Lazy => context.theme.write_delimiter(writer, '&')?,
            _ => {}
        }

        self.function.depict(writer, context)?;
        context.theme.write_delimiter(writer, ':')?;
        context.theme.write_name(writer, &self.function)?;
        context.theme.write_delimiter(writer, '(')?;

        let child_context = &context.child().with_format(DepictionFormat::Compact).with_separator(false);
        for (argument, last) in IterateWithLast::new(&self.arguments) {
            argument.depict(writer, child_context)?;
            if !last {
                context.theme.write_delimiter(writer, ',')?;
            }
        }

        context.theme.write_delimiter(writer, ')')
    }
}

impl<AnnotatedT> fmt::Display for Call<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            floria::CallKind::Eager => write!(formatter, "*")?,
            floria::CallKind::Lazy => write!(formatter, "&")?,
            _ => {}
        }

        write!(formatter, "{}(", self.function)?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            fmt::Display::fmt(argument, formatter)?;
            if !last {
                write!(formatter, ",")?;
            }
        }

        write!(formatter, ")")
    }
}

impl<AnnotatedT> PartialEq for Call<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.function == other.function) && (self.arguments == other.arguments)
    }
}

impl<AnnotatedT> Eq for Call<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Call<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.function.partial_cmp(&other.function) {
            Some(Ordering::Equal) => self.arguments.partial_cmp(&other.arguments),
            ordering => ordering,
        }
    }
}

impl<AnnotatedT> Ord for Call<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.function.cmp(&other.function) {
            Ordering::Equal => self.arguments.cmp(&other.arguments),
            ordering => ordering,
        }
    }
}

impl<AnnotatedT> Hash for Call<AnnotatedT> {
    fn hash<HasherT: Hasher>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.function.hash(state);
        self.arguments.hash(state);
    }
}
