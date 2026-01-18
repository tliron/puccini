use {
    compris::{annotate::*, errors::*, normal::*, resolve::*},
    depiction::*,
    problemo::*,
    std::{fmt, io},
};

//
// Range
//

/// Unsigned integer range with optional upper bound.
#[derive(Clone, Debug, Default)]
pub struct Range {
    /// Lower bound.
    pub lower: u64,

    /// Upper bound.
    pub upper: RangeUpperBound,
}

impl Range {
    /// Constructor.
    pub fn new(lower: u64, upper: RangeUpperBound) -> Self {
        Self { lower, upper }
    }

    /// True if the number is in the range.
    pub fn contains(&self, number: u64) -> bool {
        match self.upper {
            RangeUpperBound::Unbounded => number >= self.lower,
            RangeUpperBound::Bounded(upper) => (number >= self.lower) && (number <= upper),
        }
    }
}

impl<AnnotatedT> Resolve<Range> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(self, problems: &mut ProblemReceiverT) -> ResolveResult<Range>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let maybe_annotations = self.maybe_annotations();

        let give = |problems: &mut ProblemReceiverT, message: &str| {
            problems.give(MalformedError::as_problem("range", message).with_annotations_from(&maybe_annotations))
        };

        Ok(match self {
            Self::List(list) => match list.into_pair() {
                Some((lower_value, upper_value)) => match lower_value.resolve_with_problems(problems)? {
                    Some(lower) => match upper_value {
                        Self::Text(text) => {
                            if text.inner == "UNBOUNDED" {
                                return Ok(Some(Range::new(lower, RangeUpperBound::Unbounded)));
                            } else {
                                give(problems, "upper bound is not an unsigned integer or \"UNBOUNDED\"")?;
                                None
                            }
                        }

                        _ => upper_value
                            .resolve_with_problems(problems)?
                            .and_then(|upper| Some(Range::new(lower, RangeUpperBound::Bounded(upper)))),
                    },

                    None => None,
                },

                None => {
                    give(problems, "is not a list of length 2")?;
                    None
                }
            },

            _ => {
                problems.give(IncompatibleVariantTypeError::as_problem_from(&self, &["list"]))?;
                None
            }
        })
    }
}

impl Depict for Range {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, '[')?;
        context.theme.write_number(writer, self.lower)?;
        context.theme.write_delimiter(writer, ',')?;
        match &self.upper {
            RangeUpperBound::Unbounded => context.theme.write_symbol(writer, "Unbounded")?,
            RangeUpperBound::Bounded(upper) => context.theme.write_number(writer, upper)?,
        }
        context.theme.write_delimiter(writer, ']')
    }
}

impl fmt::Display for Range {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[{}, {}]", self.lower, self.upper)
    }
}

//
// RangeUpperBound
//

/// Range upper bound.
#[derive(Clone, Debug, Default)]
pub enum RangeUpperBound {
    /// Unbounded.
    #[default]
    Unbounded,

    /// Bounded.
    Bounded(u64),
}

impl fmt::Display for RangeUpperBound {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unbounded => fmt::Display::fmt("UNBOUNDED", formatter),
            Self::Bounded(bounded) => fmt::Display::fmt(bounded, formatter),
        }
    }
}
