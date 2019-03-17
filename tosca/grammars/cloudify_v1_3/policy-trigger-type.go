package cloudify_v1_3

import (
	"github.com/tliron/puccini/tosca"
)

//
// PolicyTriggerType
//
// [https://docs.cloudify.co/4.5.5/developer/blueprints/spec-policy-triggers/]
//

type PolicyTriggerType struct {
	*Entity `name:"policy trigger type"`
	Name    string `namespace:""`

	Source     *string             `read:"source" require:"source"`
	Parameters PropertyDefinitions `read:"parameters,PropertyDefinition"`
}

func NewPolicyTriggerType(context *tosca.Context) *PolicyTriggerType {
	return &PolicyTriggerType{
		Entity:     NewEntity(context),
		Name:       context.Name,
		Parameters: make(PropertyDefinitions),
	}
}

// tosca.Reader signature
func ReadPolicyTriggerType(context *tosca.Context) interface{} {
	self := NewPolicyTriggerType(context)
	context.ValidateUnsupportedFields(context.ReadFields(self))
	return self
}

var policyTriggerTypeRoot *PolicyTriggerType

// tosca.Hierarchical interface
func (self *PolicyTriggerType) GetParent() interface{} {
	return policyTriggerTypeRoot
}
