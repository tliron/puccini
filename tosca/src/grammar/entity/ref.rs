use super::entity::*;

use {
    compris::annotate::*,
    depiction::*,
    kutil::std::any::*,
    std::{any::*, fmt},
};

//
// EntityRef
//

/// Common reference type for [Entity].
pub type EntityRef = Box<dyn Entity>;

impl fmt::Debug for EntityRef {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "EntityRef")
    }
}

impl<EntityT> From<EntityT> for EntityRef
where
    EntityT: 'static + Entity,
{
    fn from(value: EntityT) -> Self {
        Box::new(value)
    }
}

impl AsAnyRef for EntityRef {
    fn as_any_ref(&self) -> Option<&dyn Any> {
        Some(self.as_ref())
    }
}

impl Annotated for EntityRef {
    fn can_have_annotations() -> bool {
        true
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.as_ref().dyn_annotations()
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.as_mut().dyn_annotations_mut()
    }
}

impl Depict for EntityRef {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> std::io::Result<()>
    where
        WriteT: std::io::Write,
    {
        self.as_ref().dyn_depict(Box::new(writer), context)
    }
}
