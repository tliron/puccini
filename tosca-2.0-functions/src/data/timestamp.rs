use super::comparator::*;

use {
    chrono::*,
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, num::*, str::*},
};

//
// Timestamp
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA timestamp type represents a local instant in time containing two elements: the local
/// notation plus the time zone offset.
///
/// TOSCA timestamps are represented as strings following RFC 3339, which in turn uses a simplified
/// profile of ISO 8601. TOSCA adds an exception to RFC 3339: though RFC 3339 supports timestamps
/// with unknown local offsets, represented as the "-0" timezone, TOSCA does not support this
/// feature and will treat the unknown time zone as UTC. There are two reasons for this exception:
/// the first is that many systems do not support this distinction and TOSCA aims for
/// interoperability, and the second is that timestamps with unknown time zones cannot be converted
/// to UTC, making it impossible to apply comparison functions. If this feature is required, it can
/// be supported via a custom data type.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// Datetime.
    pub datetime: DateTime<FixedOffset>,
}

impl Timestamp {
    /// Constructor.
    pub fn new(datetime: DateTime<FixedOffset>) -> Self {
        Self { datetime }
    }
}

impl Comparator for Timestamp {
    fn comparator(&self) -> Any {
        self.datetime.timestamp_micros().into()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.datetime.to_rfc3339(), formatter)
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        // Note: chrono treats "-00:00" as UTC, as expected by TOSCA
        let Ok(datetime) = DateTime::parse_from_rfc3339(representation) else {
            return Err("not RFC 3339".into());
        };

        Ok(Self::new(datetime))
    }
}

impl TryFrom<&Any> for Timestamp {
    type Error = String;

    fn try_from(any: &Any) -> Result<Self, Self::Error> {
        match any {
            Any::Text(text) => text.parse(),
            Any::AnyMap(any_map) => (&any_map.to_map().inner).try_into(),
            _ => Err("timestamp is not a string or a map".into()),
        }
    }
}

macro_rules! to_integer (
    ( $key:tt, $value:ident $(,)? ) => {
        Some(match $value {
            Any::Integer(integer) => {
                (*integer).try_into().map_err(|error: TryFromIntError| error.to_string())?
            }
            Any::UnsignedInteger(unsigned_integer) => {
                (*unsigned_integer).try_into().map_err(|error: TryFromIntError| error.to_string())?
            }
            _ => return Err(format!("timestamp \"{}\" key is not an integer", stringify!($key))),
        })
    }
);

impl TryFrom<&BTreeMap<Any, Any>> for Timestamp {
    type Error = String;

    fn try_from(map: &BTreeMap<Any, Any>) -> Result<Self, Self::Error> {
        let mut year: Option<i32> = None;
        let mut month: Option<u32> = None;
        let mut day: Option<u32> = None;
        let mut hour: Option<u32> = None;
        let mut minute: Option<u32> = None;
        let mut second: Option<u32> = None;
        let mut nanosecond: Option<u32> = None;
        let mut utc_offset_seconds: Option<i32> = None;

        for (key, value) in map {
            match key {
                Any::Text(text) => match text.as_str() {
                    "year" => year = to_integer!(year, value),
                    "month" => month = to_integer!(month, value),
                    "day" => day = to_integer!(day, value),
                    "hour" => hour = to_integer!(hour, value),
                    "minute" => minute = to_integer!(minute, value),
                    "second" => second = to_integer!(second, value),
                    "nanosecond" => nanosecond = to_integer!(nanosecond, value),
                    "utc_offset_seconds" => utc_offset_seconds = to_integer!(utc_offset_seconds, value),
                    _ => return Err(format!("timestamp has unsupported key: {}", key)),
                },

                _ => return Err(format!("timestamp has unsupported key: {}", key)),
            }
        }

        if let Some(year) = year
            && let Some(month) = month
            && let Some(day) = day
            && let Some(hour) = hour
            && let Some(minute) = minute
            && let Some(second) = second
            && let Some(nanosecond) = nanosecond
            && let Some(utc_offset_seconds) = utc_offset_seconds
        {
            let Some(offset) = FixedOffset::east_opt(utc_offset_seconds) else {
                return Err(format!("timestamp has invalid \"utc_offset_seconds\" key: {}", utc_offset_seconds));
            };

            let MappedLocalTime::Single(datetime) = offset.with_ymd_and_hms(year, month, day, hour, minute, second)
            else {
                return Err("invalid timestamp".into());
            };

            let Some(datetime) = datetime.with_nanosecond(nanosecond) else {
                return Err(format!("timestamp has invalid \"nanosecond\" key: {}", nanosecond));
            };

            Ok(Self::new(datetime))
        } else {
            Err("timestamp is missing keys".into())
        }
    }
}

impl Into<Any> for Timestamp {
    fn into(self) -> Any {
        // Note: all the values here are either i32 or u32, so they will always be castable to i64 and u64
        normal_map!(
            ("year", self.datetime.year() as i64),
            ("month", self.datetime.month() as u64),
            ("day", self.datetime.day() as u64),
            ("hour", self.datetime.hour() as u64),
            ("minute", self.datetime.minute() as u64),
            ("second", self.datetime.second() as u64),
            ("nanosecond", self.datetime.nanosecond() as u64),
            ("utc-offset-seconds", self.datetime.offset().local_minus_utc() as i64),
        )
    }
}
