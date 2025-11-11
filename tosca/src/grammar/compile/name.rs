use kutil::std::immutable::*;

/// Floria ID name delimiter.
pub const FLORIA_ID_NAME_DELIMITER: char = ':';

//
// ToFloriaName
//

/// To Floria name.
pub trait ToFloriaName {
    /// To Floria name.
    fn to_floria_name(&self, prefix: &str) -> ByteString;

    /// To Floria name (contained).
    fn to_floria_name_contained(&self, prefix: &str, container: &str) -> ByteString;
}

impl<ToStringT> ToFloriaName for ToStringT
where
    ToStringT: ToString,
{
    fn to_floria_name(&self, prefix: &str) -> ByteString {
        format!("{}{}{}", prefix, FLORIA_ID_NAME_DELIMITER, self.to_string()).into()
    }

    fn to_floria_name_contained(&self, prefix: &str, container: &str) -> ByteString {
        format!("{}{}{}{}{}", container, FLORIA_ID_NAME_DELIMITER, prefix, FLORIA_ID_NAME_DELIMITER, self.to_string())
            .into()
    }
}
