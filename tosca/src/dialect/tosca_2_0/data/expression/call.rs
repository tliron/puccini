use super::{super::super::dialect::*, expression::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::{cmp::*, fmt, hash::*, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug)]
pub struct Call<AnnotatedT> {
    /// Plugin name.
    pub plugin: ByteString,

    /// Function name.
    pub function: Text<AnnotatedT>,

    /// Arguments.
    pub arguments: Vec<Expression<AnnotatedT>>,

    /// Kind.
    pub kind: floria::CallKind,
}

impl<AnnotatedT> Call<AnnotatedT> {
    /// Constructor.
    pub fn new(
        plugin: ByteString,
        function: Text<AnnotatedT>,
        arguments: Vec<Expression<AnnotatedT>>,
        kind: floria::CallKind,
    ) -> Self {
        Self { plugin, function, arguments, kind }
    }

    /// Constructor.
    ///
    /// Note that TOSCA 2.0 can only represent normal calls.
    pub fn new_native(
        function: Text<AnnotatedT>,
        arguments: Vec<Expression<AnnotatedT>>,
        kind: floria::CallKind,
    ) -> Self {
        Self::new(DIALECT_ID, function, arguments, kind)
    }

    /// Constructor.
    pub fn new_native_static(function: &'static str, kind: floria::CallKind) -> Self
    where
        AnnotatedT: Default,
    {
        Self::new(DIALECT_ID, ByteString::from_static(function).into(), Default::default(), kind)
    }

    /// True if in dialect's plugin.
    pub fn is_native(&self) -> bool {
        self.plugin == DIALECT_ID
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

impl<AnnotatedT> Annotated for Call<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.function.annotated.annotations()
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.function.annotated.annotations_mut()
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

        context.theme.write_name(writer, &self.plugin)?;
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

        write!(formatter, "{}:{}(", self.plugin, self.function)?;

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
        (self.plugin == other.plugin) && (self.function == other.function) && (self.arguments == other.arguments)
    }
}

impl<AnnotatedT> Eq for Call<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Call<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.plugin.partial_cmp(&other.plugin) {
            Some(Ordering::Equal) => match self.function.partial_cmp(&other.function) {
                Some(Ordering::Equal) => self.arguments.partial_cmp(&other.arguments),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl<AnnotatedT> Ord for Call<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.plugin.cmp(&other.plugin) {
            Ordering::Equal => match self.function.cmp(&other.function) {
                Ordering::Equal => self.arguments.cmp(&other.arguments),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl<AnnotatedT> Hash for Call<AnnotatedT> {
    fn hash<HasherT: Hasher>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.plugin.hash(state);
        self.function.hash(state);
        self.arguments.hash(state);
    }
}

impl<AnnotatedT> Into<floria::Call> for Call<AnnotatedT> {
    fn into(self) -> floria::Call {
        let arguments: Vec<_> = self.arguments.into_iter().map(|argument| argument.into()).collect();
        floria::Call::new(self.plugin, self.function.inner, arguments, self.kind)
    }
}

impl<AnnotatedT> Into<floria::Expression> for Call<AnnotatedT> {
    fn into(self) -> floria::Expression {
        floria::Expression::Call(self.into())
    }
}
