use {
    floria_plugin_sdk::{data::*, utils::*, *},
    serde::{de::*, *},
    std::io,
    struson::{reader::*, serde::*, writer::*},
};

/// Namespace in URL.
pub fn namespace_in_url<NamespaceT>(namespace: Option<NamespaceT>) -> String
where
    NamespaceT: AsRef<str>,
{
    match namespace {
        Some(namespace) => format!("namespaces/{}/", namespace.as_ref()),
        None => "/".into(),
    }
}

/// HTTP client.
pub fn http_client(configuration: &Map) -> Result<HttpClient, DispatchError> {
    let mut client = HttpClient::default();
    add_header(&mut client, "xx-root-certificates", configuration, "root-certificates")?;
    add_header(&mut client, "xx-certificates", configuration, "user-certificates")?;
    add_header(&mut client, "xx-private-key", configuration, "user-private-key")?;
    Ok(client)
}

/// Add header.
pub fn add_header(client: &mut HttpClient, name: &str, configuration: &Map, key: &str) -> Result<(), DispatchError> {
    if let Some(value) = configuration.into_get(key)
        && let Expression::Text(value) = value
    {
        client.add_header(name, value)?;
    }
    Ok(())
}

/// To expression.
pub fn to_expression<SerializeT>(value: SerializeT) -> Result<Expression, DispatchError>
where
    SerializeT: Serialize,
{
    value.serialize(&ExpressionSerializer::new(Some("k8s:".into()))).map_err(|error| error.0)
}

/// To JSON.
pub fn to_json<SerializeT>(value: SerializeT) -> Result<Vec<u8>, DispatchError>
where
    SerializeT: Serialize,
{
    let mut bytes = Vec::<u8>::default();
    let mut writer = JsonStreamWriter::new(&mut bytes);
    let mut serializer = JsonWriterSerializer::new(&mut writer);
    value.serialize(&mut serializer).map_err(|error| escape_depiction_markup(error))?;
    writer.finish_document().map_err(|error| escape_depiction_markup(error))?;
    Ok(bytes)
}

/// To JSON.
pub fn to_json_string<SerializeT>(value: SerializeT) -> Result<String, DispatchError>
where
    SerializeT: Serialize,
{
    String::from_utf8(to_json(value)?).map_err(|error| escape_depiction_markup(error))
}

/// From JSON.
pub fn from_json<ReadT, DeserializeT>(reader: ReadT) -> Result<DeserializeT, DispatchError>
where
    ReadT: io::Read,
    DeserializeT: DeserializeOwned,
{
    let mut reader = JsonStreamReader::new(reader);
    reader.deserialize_next().map_err(|error| escape_depiction_markup(error))
}
