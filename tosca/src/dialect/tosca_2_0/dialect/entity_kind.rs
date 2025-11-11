use super::super::super::super::grammar::*;

//
// Type Entities
//

/// Artifact type.
pub const ARTIFACT_TYPE: EntityKind = EntityKind(1);

/// Artifact type name.
pub const ARTIFACT_TYPE_NAME: &str = "artifact-type";

/// Capability type.
pub const CAPABILITY_TYPE: EntityKind = EntityKind(2);

/// Capability type name.
pub const CAPABILITY_TYPE_NAME: &str = "capability-type";

/// Data type.
pub const DATA_TYPE: EntityKind = EntityKind(3);

/// Data type name.
pub const DATA_TYPE_NAME: &str = "data-type";

/// Group type.
pub const GROUP_TYPE: EntityKind = EntityKind(4);

/// Group type name.
pub const GROUP_TYPE_NAME: &str = "group-type";

/// Interface type.
pub const INTERFACE_TYPE: EntityKind = EntityKind(5);

/// Interface type name.
pub const INTERFACE_TYPE_NAME: &str = "interface-type";

/// Node type.
pub const NODE_TYPE: EntityKind = EntityKind(6);

/// Node type name.
pub const NODE_TYPE_NAME: &str = "node-type";

/// Policy type.
pub const POLICY_TYPE: EntityKind = EntityKind(7);

/// Policy type name.
pub const POLICY_TYPE_NAME: &str = "policy-type";

/// Relationship type.
pub const RELATIONSHIP_TYPE: EntityKind = EntityKind(8);

/// Relationship type name.
pub const RELATIONSHIP_TYPE_NAME: &str = "relationship-type";

// Template entities

/// Service template.
pub const SERVICE_TEMPLATE: EntityKind = EntityKind(100);

/// Service template name.
pub const SERVICE_TEMPLATE_NAME: &str = "service";

/// Group template.
pub const GROUP_TEMPLATE: EntityKind = EntityKind(104);

/// Group template name.
pub const GROUP_TEMPLATE_NAME: &str = "group";

/// Node template.
pub const NODE_TEMPLATE: EntityKind = EntityKind(106);

/// Node template name.
pub const NODE_TEMPLATE_NAME: &str = "node";

/// Policy template.
pub const POLICY_TEMPLATE: EntityKind = EntityKind(107);

/// Policy template name.
pub const POLICY_TEMPLATE_NAME: &str = "policy";

/// Relationship template.
pub const RELATIONSHIP_TEMPLATE: EntityKind = EntityKind(108);

/// Relationship template name.
pub const RELATIONSHIP_TEMPLATE_NAME: &str = "relationship";

//
// Other entities
//

/// Repository.
pub const REPOSITORY: EntityKind = EntityKind(200);

/// Repository name.
pub const REPOSITORY_NAME: &str = "repository";

/// Function.
pub const FUNCTION: EntityKind = EntityKind(201);

/// Function name.
pub const FUNCTION_NAME: &str = "function";

//
// Definition entities
//

/// Interface name.
pub const INTERFACE_NAME: &str = "interface";

/// Artifact name.
pub const ARTIFACT_NAME: &str = "artifact";

/// Capability name.
pub const CAPABILITY_NAME: &str = "capability";

/// Requirement name.
pub const REQUIREMENT_NAME: &str = "requirement";

/// Property name.
pub const PROPERTY_NAME: &str = "property";

/// Attribute name.
pub const ATTRIBUTE_NAME: &str = "attribute";

/// Parameter name.
pub const PARAMETER_NAME: &str = "parameter";

impl super::Dialect {
    /// TOSCA 2.0 supported entity kinds.
    pub fn entity_kinds() -> EntityKinds {
        let mut entity_kinds = EntityKinds::default();

        entity_kinds.add(ARTIFACT_TYPE, ARTIFACT_TYPE_NAME.into());
        entity_kinds.add(CAPABILITY_TYPE, CAPABILITY_TYPE_NAME.into());
        entity_kinds.add(DATA_TYPE, DATA_TYPE_NAME.into());
        entity_kinds.add(GROUP_TYPE, GROUP_TYPE_NAME.into());
        entity_kinds.add(INTERFACE_TYPE, INTERFACE_TYPE_NAME.into());
        entity_kinds.add(NODE_TYPE, NODE_TYPE_NAME.into());
        entity_kinds.add(POLICY_TYPE, POLICY_TYPE_NAME.into());
        entity_kinds.add(RELATIONSHIP_TYPE, RELATIONSHIP_TYPE_NAME.into());

        entity_kinds.add(SERVICE_TEMPLATE, SERVICE_TEMPLATE_NAME.into());
        entity_kinds.add(GROUP_TEMPLATE, GROUP_TEMPLATE_NAME.into());
        entity_kinds.add(NODE_TEMPLATE, NODE_TEMPLATE_NAME.into());
        entity_kinds.add(POLICY_TEMPLATE, POLICY_TEMPLATE_NAME.into());
        entity_kinds.add(RELATIONSHIP_TEMPLATE, RELATIONSHIP_TEMPLATE_NAME.into());

        entity_kinds.add(REPOSITORY, REPOSITORY_NAME.into());
        entity_kinds.add(FUNCTION, FUNCTION_NAME.into());

        entity_kinds
    }
}
