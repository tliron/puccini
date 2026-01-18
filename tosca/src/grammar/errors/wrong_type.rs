use {
    compris::annotate::*,
    depiction::*,
    derive_more::*,
    kutil::std::string::*,
    problemo::*,
    std::{fmt, io},
};

//
// WrongTypeError
//

/// Wrong type error.
#[derive(Debug, Error, PartialEq)]
pub struct WrongTypeError {
    /// Entity.
    pub entity: String,

    /// Type name.
    pub type_name: String,

    /// Expected type names.
    pub expected_type_names: Vec<String>,
}

impl WrongTypeError {
    /// Constructor.
    pub fn new<EntityT, TypeNameT>(entity: EntityT, type_name: TypeNameT, expected_type_names: Vec<String>) -> Self
    where
        EntityT: ToString,
        TypeNameT: ToString,
    {
        Self { entity: entity.to_string(), type_name: type_name.to_string(), expected_type_names }
    }

    /// Constructor.
    #[track_caller]
    pub fn as_problem<EntityT, TypeNameT>(
        entity: EntityT,
        type_name: TypeNameT,
        expected_type_names: Vec<String>,
    ) -> Problem
    where
        EntityT: ToString,
        TypeNameT: ToString,
    {
        Self::new(entity, type_name, expected_type_names)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for WrongTypeError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let type_name = format!("{:?}", self.type_name);
        if self.expected_type_names.is_empty() {
            write!(writer, "{} has wrong type: {}", self.entity, context.theme.error(type_name))
        } else {
            write!(
                writer,
                "{} has wrong type: is {}, expected {}",
                self.entity,
                context.theme.error(&self.type_name),
                self.expected_type_names.join_conjunction("or")
            )
        }
    }
}

impl fmt::Display for WrongTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.expected_type_names.is_empty() {
            write!(formatter, "{} is {}", self.entity, self.type_name,)
        } else {
            write!(
                formatter,
                "{} is {}, expected {}",
                self.entity,
                self.type_name,
                self.expected_type_names.join_conjunction("or")
            )
        }
    }
}
