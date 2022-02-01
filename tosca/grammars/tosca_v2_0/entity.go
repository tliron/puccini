package tosca_v2_0

import (
	"sync"

	"github.com/tliron/puccini/tosca"
)

//
// Entity
//

type Entity struct {
	Context *tosca.Context `traverse:"ignore" json:"-" yaml:"-"`

	renderOnce sync.Once
}

func NewEntity(context *tosca.Context) *Entity {
	return &Entity{
		Context: context,
	}
}

// tosca.Contextual interface
func (self *Entity) GetContext() *tosca.Context {
	return self.Context
}
