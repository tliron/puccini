use {
    floria_plugin_sdk::{data::*, utils::*, *},
    serde::*,
    struson::{serde::*, writer::*},
};

pub fn namespace_in_url<NamespaceT>(namespace: Option<NamespaceT>) -> String
where
    NamespaceT: AsRef<str>,
{
    match namespace {
        Some(namespace) => format!("namespaces/{}/", namespace.as_ref()),
        None => "/".into(),
    }
}

pub fn http_client(configuration: &Map) -> Result<HttpClient, DispatchError> {
    let mut client = HttpClient::default();
    add_header(&mut client, "xx-root-certificates", configuration, "root-certificates")?;
    add_header(&mut client, "xx-certificates", configuration, "user-certificates")?;
    add_header(&mut client, "xx-private-key", configuration, "user-private-key")?;
    Ok(client)
}

pub fn add_header(client: &mut HttpClient, name: &str, configuration: &Map, key: &str) -> Result<(), DispatchError> {
    if let Some(value) = configuration.into_get(key)
        && let Expression::Text(value) = value
    {
        client.add_header(name, value)?;
    }
    Ok(())
}

/// To JSON.
pub fn to_json<SerializeT>(value: SerializeT) -> Result<String, DispatchError>
where
    SerializeT: Serialize,
{
    let mut buffer = Vec::<u8>::default();
    let mut writer = JsonStreamWriter::new(&mut buffer);
    let mut serializer = JsonWriterSerializer::new(&mut writer);
    value.serialize(&mut serializer).map_err(|error| escape_depiction_markup(error))?;
    writer.finish_document().map_err(|error| escape_depiction_markup(error))?;
    let string = String::from_utf8(buffer).map_err(|error| escape_depiction_markup(error))?;
    Ok(string)
}
