use super::kind::*;

use floria_plugin_sdk::{entities::*, utils::*};

//
// ToscaInstance
//

/// TOSCA instance.
pub trait ToscaInstance {
    /// TOSCA kind.
    fn tosca_kind(&self) -> Option<ToscaKind>;

    /// TOSCA name.
    fn tosca_name(&self) -> Option<String>;

    /// True if TOSCA entity.
    fn is_tosca(&self, tosca_kind: Option<ToscaKind>, name: Option<&str>) -> bool {
        match (tosca_kind, name) {
            (Some(tosca_kind), Some(name)) => {
                if let Some(self_tosca_kind) = self.tosca_kind()
                    && let Some(self_name) = self.tosca_name()
                {
                    (self_tosca_kind == tosca_kind) && (self_name == name)
                } else {
                    false
                }
            }

            (Some(tosca_kind), None) => {
                if let Some(self_tosca_kind) = self.tosca_kind() {
                    self_tosca_kind == tosca_kind
                } else {
                    false
                }
            }

            (None, Some(name)) => {
                if let Some(self_name) = self.tosca_name() {
                    self_name == name
                } else {
                    false
                }
            }

            _ => self.tosca_kind().is_some(),
        }
    }

    /// Error if not TOSCA entity.
    fn assert_tosca(&self, tosca_kind: Option<ToscaKind>, name: Option<&str>) -> Result<(), String>;
}

impl<MetadataT> ToscaInstance for MetadataT
where
    MetadataT: Metadata + Instance,
{
    fn tosca_kind(&self) -> Option<ToscaKind> {
        self.metadata_map_string("tosca", "kind").and_then(|entity_kind| entity_kind.parse().ok())
    }

    fn tosca_name(&self) -> Option<String> {
        self.metadata_map_string("tosca", "name")
    }

    fn assert_tosca(&self, tosca_kind: Option<ToscaKind>, name: Option<&str>) -> Result<(), String> {
        match (tosca_kind, name) {
            (Some(tosca_kind), Some(name)) => {
                if let Some(self_tosca_kind) = self.tosca_kind()
                    && let Some(self_name) = self.tosca_name()
                    && (self_tosca_kind == tosca_kind)
                    && (self_name == name)
                {
                    Ok(())
                } else {
                    Err(format!(
                        "|name|{}| not a TOSCA |meta|{}| named {:?}",
                        escape_depiction_markup(self.id()),
                        tosca_kind.as_str(),
                        escape_depiction_markup(name)
                    ))
                }
            }

            (Some(tosca_kind), None) => {
                if let Some(self_tosca_kind) = self.tosca_kind()
                    && (self_tosca_kind == tosca_kind)
                {
                    Ok(())
                } else {
                    Err(format!(
                        "|name|{}| not a TOSCA |meta|{}|",
                        escape_depiction_markup(self.id()),
                        tosca_kind.as_str()
                    ))
                }
            }

            (None, Some(name)) => {
                if let Some(self_name) = self.tosca_name()
                    && (self_name == name)
                {
                    Ok(())
                } else {
                    Err(format!(
                        "|name|{}| not a TOSCA entity named {:?}",
                        escape_depiction_markup(self.id()),
                        escape_depiction_markup(name)
                    ))
                }
            }

            _ => match self.tosca_kind() {
                Some(_) => Ok(()),
                None => Err(format!("|name|{}| not a TOSCA entity", escape_depiction_markup(self.id()),)),
            },
        }
    }
}
