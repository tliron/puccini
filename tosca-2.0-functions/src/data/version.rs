use super::comparator::*;

use {
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, num::*, str::*},
};

//
// Version
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA version type represents a version string. TOSCA versions provide a normative means to
/// represent a version string which enables the comparison and management of version information
/// over time.
#[derive(Clone, Debug, Default)]
pub struct Version {
    /// Major. A mandatory integer value greater than or equal to 0 (zero).
    pub major: u64,

    /// Minor. A mandatory integer value greater than or equal to 0 (zero).
    pub minor: u64,

    /// Fix. An optional integer value greater than or equal to 0 (zero).
    pub fix: Option<u64>,

    /// Qualifier. an optional string that indicates a named, pre-release version of the associated
    /// code that has been derived from the version of the code identified by the combination
    /// <major_version>, <minor_version> and <fix_version> numbers.
    pub qualifier: Option<String>,

    /// Build. An optional integer value greater than or equal to 0 (zero) that can be used to
    /// further qualify different build versions of the code that has the same <qualifer_string>.
    pub build: Option<u64>,
}

impl Version {
    /// Constructor.
    #[allow(unused)]
    pub fn new(major: u64, minor: u64, fix: Option<u64>, qualifier: Option<String>, build: Option<u64>) -> Self {
        Self { major, minor, fix, qualifier, build }
    }
}

impl Comparator for Version {
    fn comparator(&self) -> Any {
        normal_vec!(
            self.major,
            self.minor,
            self.fix.map(|fix| fix.into()).unwrap_or(Any::Null),
            self.qualifier.as_ref().map(|qualifier| qualifier.clone().into()).unwrap_or(Any::Null),
            self.build.map(|build| build.into()).unwrap_or(Any::Null),
        )
        .into()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)?;

        if let Some(fix) = &self.fix {
            write!(formatter, ".{}", fix)?;
        }

        match &self.qualifier {
            Some(qualifier) => match &self.build {
                Some(build) => write!(formatter, "-{}{}", qualifier, build)?,
                None => write!(formatter, "-{}", qualifier)?,
            },

            None => {
                if let Some(build) = &self.build {
                    write!(formatter, "-{}", build)?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // Our parsing method does no additional string allocation;
        // It's based on different slices in the same str

        let mut pre_dash = None;
        let mut post_dash = None;
        for (index, segment) in string.split('-').enumerate() {
            match index {
                0 => pre_dash = Some(segment),
                1 => post_dash = Some(segment),
                _ => return Err("version has more than one \"-\"".into()),
            }
        }

        let mut major = None;
        let mut minor = None;
        let mut fix = None;
        for (index, segment) in pre_dash.expect("pre_dash").split('.').enumerate() {
            match index {
                0 => major = Some(segment),
                1 => minor = Some(segment),
                2 => fix = Some(segment),
                _ => return Err("version has more than two \".\"".into()),
            }
        }

        if major.is_none() || minor.is_none() {
            return Err("version does not have at least \"major.minor\"".into());
        }

        let mut version = Version::default();

        version.major = match major.expect("major").parse() {
            Ok(major) => major,
            Err(_) => return Err("version \"major\" is not a number".into()),
        };

        version.minor = match minor.expect("minor").parse() {
            Ok(minor) => minor,
            Err(_) => return Err("version \"minor\" is not a number".into()),
        };

        if let Some(fix) = fix {
            version.fix = match fix.parse() {
                Ok(fix) => Some(fix),
                Err(_) => return Err("version \"fix\" is not a number".into()),
            };
        };

        if let Some(post_dash) = post_dash {
            // The spec is unclear, but we will assume the qualifier cannot have digits
            // In other words, the build number starts with the first digit
            match post_dash.find(|c: char| c.is_digit(10)) {
                Some(number_start) => {
                    version.qualifier = Some(post_dash[..number_start].into());

                    version.build = match post_dash[number_start..].parse() {
                        Ok(build) => Some(build),
                        Err(_) => return Err("version \"build\" is not a number".into()),
                    };
                }

                None => version.qualifier = Some(post_dash.into()),
            }
        };

        Ok(version)
    }
}

impl TryFrom<&Any> for Version {
    type Error = String;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        match any {
            Any::Text(text) => text.parse(),
            Any::AnyMap(any_map) => (&any_map.to_map().inner).try_into(),
            _ => Err("version is not a string or a map".into()),
        }
    }
}

macro_rules! to_unsigned_integer (
    ( $key:tt, $value:ident $(,)? ) => {
        match $value {
            Any::Integer(integer) => {
                (*integer).try_into().map_err(|error: TryFromIntError| error.to_string())?
            }
            Any::UnsignedInteger(unsigned_integer) => *unsigned_integer,
            _ => return Err(format!("version \"{}\" key is not an integer", stringify!($key))),
        }
    }
);

impl TryFrom<&BTreeMap<Any, Any>> for Version {
    type Error = String;

    fn try_from(map: &BTreeMap<Any, Any>) -> Result<Self, Self::Error> {
        let mut major = None;
        let mut minor = None;
        let mut fix = None;
        let mut qualifier = None;
        let mut build = None;

        for (key, value) in map {
            match key {
                Any::Text(text) => match text.as_str() {
                    "major" => major = Some(to_unsigned_integer!(major, value)),
                    "minor" => minor = Some(to_unsigned_integer!(minor, value)),
                    "fix" => fix = Some(to_unsigned_integer!(fix, value)),

                    "qualifier" => {
                        qualifier = Some(match value {
                            Any::Text(text) => text.clone(),
                            _ => return Err("version \"qualifier\" key is not a string".into()),
                        });
                    }

                    "build" => build = Some(to_unsigned_integer!(build, value)),

                    _ => return Err(format!("version has unsupported key: {}", key)),
                },

                _ => return Err(format!("version has unsupported key: {}", key)),
            }
        }

        if let Some(major) = major
            && let Some(minor) = minor
        {
            Ok(Self::new(major, minor, fix, qualifier, build))
        } else {
            Err("version is missing keys".into())
        }
    }
}

impl Into<Any> for Version {
    fn into(self) -> Any {
        let mut map = BTreeMap::default();

        map.insert("major".into(), self.major.into());
        map.insert("minor".into(), self.minor.into());

        if let Some(fix) = self.fix {
            map.insert("fix".into(), fix.into());
        }

        if let Some(qualifier) = self.qualifier {
            map.insert("qualifier".into(), qualifier.into());
        }

        if let Some(build) = self.build {
            map.insert("build".into(), build.into());
        }

        let map: Map = map.into();
        map.into()
    }
}
