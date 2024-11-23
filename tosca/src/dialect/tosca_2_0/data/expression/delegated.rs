use super::expression::*;

use {
    compris::annotate::*,
    kutil::{
        cli::depict::{utils::*, *},
        std::iter::*,
    },
    std::{cmp::*, fmt, hash::*, io},
};

impl<AnnotatedT> Annotated for Expression<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Simple(simple) => simple.annotations(),
            Self::List(list) => list.iter().next().and_then(|item| item.annotations()),
            Self::Map(map) => map.iter().next().and_then(|(_key, value)| value.annotations()),
            Self::Call(call) => call.annotations(),
        }
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Simple(simple) => simple.annotations_mut(),
            Self::List(list) => list.iter_mut().next().and_then(|item| item.annotations_mut()),
            Self::Map(map) => map.iter_mut().next().and_then(|(_key, value)| value.annotations_mut()),
            Self::Call(call) => call.annotations_mut(),
        }
    }
}

impl<AnnotatedT> Depict for Expression<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Simple(simple) => simple.depict(writer, context),
            Self::List(list) => depict_list(list.iter(), None, writer, context),
            Self::Map(map) => depict_map(map.iter(), None, writer, context),
            Self::Call(call) => call.depict(writer, context),
        }
    }
}

impl<AnnotatedT> Default for Expression<AnnotatedT> {
    fn default() -> Self {
        Self::Simple(Default::default())
    }
}

impl<AnnotatedT> fmt::Display for Expression<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(simple) => fmt::Display::fmt(simple, formatter),

            Self::List(list) => {
                write!(formatter, "[")?;

                for (item, last) in IterateWithLast::new(list) {
                    fmt::Display::fmt(item, formatter)?;
                    if !last {
                        write!(formatter, ",")?;
                    }
                }

                write!(formatter, "]")
            }

            Self::Map(map) => {
                write!(formatter, "{{")?;

                for ((key, value), last) in IterateWithLast::new(map) {
                    write!(formatter, "{}:{}", key, value)?;
                    if !last {
                        write!(formatter, ",")?;
                    }
                }

                write!(formatter, "}}")
            }

            Self::Call(call) => fmt::Display::fmt(call, formatter),
        }
    }
}

impl<AnnotatedT> PartialEq for Expression<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Simple(simple), Self::Simple(other_simple)) => simple == other_simple,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            (Self::Call(call), Self::Call(other_call)) => call == other_call,
            _ => false,
        }
    }
}

impl<AnnotatedT> Eq for Expression<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Expression<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Simple(simple), Self::Simple(other_simple)) => simple.partial_cmp(other_simple),
            (Self::List(list), Self::List(other_list)) => list.partial_cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.partial_cmp(other_map),
            (Self::Call(call), Self::Call(other_call)) => call.partial_cmp(other_call),
            _ => None,
        }
    }
}

impl<AnnotatedT> Ord for Expression<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Simple(simple), Self::Simple(other_simple)) => simple.cmp(other_simple),
            (Self::List(list), Self::List(other_list)) => list.cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.cmp(other_map),
            (Self::Call(call), Self::Call(other_call)) => call.cmp(other_call),

            (Self::Simple(_), _) => Ordering::Less,

            (Self::List(_), Self::Simple(_)) => Ordering::Greater,
            (Self::List(_), _) => Ordering::Less,

            (Self::Map(_), Self::Simple(_) | Self::List(_)) => Ordering::Greater,
            (Self::Map(_), _) => Ordering::Less,

            (Self::Call(_), _) => Ordering::Greater,
        }
    }
}

impl<AnnotatedT> Hash for Expression<AnnotatedT> {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Simple(simple) => {
                state.write_u8(1);
                simple.hash(state);
            }

            Self::List(list) => {
                state.write_u8(2);
                list.hash(state);
            }

            Self::Map(map) => {
                state.write_u8(3);
                map.hash(state);
            }

            Self::Call(call) => {
                state.write_u8(4);
                call.hash(state);
            }
        }
    }
}
