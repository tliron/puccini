use super::id::*;

use {problemo::*, read_url::*};

/// URL to source ID.
///
/// If the URL points to a CSAR then it will be converted to a URL pointing to the CSAR's entry
/// definitions. (This functionality requires the `csar` feature to be enabled.)
#[cfg(feature = "csar")]
pub fn url_to_source_id(
    url: String,
    url_context: &UrlContextRef,
    problems: &mut Problems,
) -> Result<SourceID, Problem> {
    use puccini_csar::url::*;
    Ok(match CsarUrl::to_entry_definitions_url(url.clone(), url_context.clone(), problems)? {
        Some(entry_definitions_url) => SourceID::URL(entry_definitions_url.to_string().into()),
        None => SourceID::URL(url.into()),
    })
}

/// URL to source ID.
///
/// If the URL points to a CSAR then it will be converted to a URL pointing to the CSAR's entry
/// definitions. (This functionality requires the `csar` feature to be enabled.)
#[cfg(not(feature = "csar"))]
pub fn url_to_source_id(
    url: String,
    _url_context: &UrlContextRef,
    _problems: &mut Problems,
) -> Result<SourceID, Problem> {
    Ok(SourceID::URL(url.into()))
}
