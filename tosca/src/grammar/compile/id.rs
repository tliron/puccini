use kutil::std::immutable::*;

/// Floria ID name delimiter.
pub const FLORIA_ID_NAME_DELIMIITER: char = ':';

/// To Floria ID name.
pub fn to_floria_id_name(prefix: &str, name: &str) -> ByteString {
    format!("{}{}{}", prefix, FLORIA_ID_NAME_DELIMIITER, name).into()
}

/// To Floria ID name.
pub fn to_floria_id_name_contained(prefix: &str, name: &str, container: &str) -> ByteString {
    format!("{}{}{}{}{}", container, FLORIA_ID_NAME_DELIMIITER, prefix, FLORIA_ID_NAME_DELIMIITER, name).into()
}
