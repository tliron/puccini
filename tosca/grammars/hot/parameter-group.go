package hot

import (
	"github.com/tliron/puccini/tosca"
)

//
// ParameterGroup
//
// [https://docs.openstack.org/heat/rocky/template_guide/hot_spec.html#parameter-groups-section]
//

type ParameterGroup struct {
	*Entity `name:"parameter group"`

	Label       *string   `read:"label"`
	Description *string   `read:"description"`
	Parameters  []*string `read:"parameters"`
}

func NewParameterGroup(context *tosca.Context) *ParameterGroup {
	return &ParameterGroup{Entity: NewEntity(context)}
}

// tosca.Reader signature
func ReadParameterGroup(context *tosca.Context) interface{} {
	self := NewParameterGroup(context)
	context.ValidateUnsupportedFields(append(context.ReadFields(self)))
	return self
}