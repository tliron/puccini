package parser

import (
	"github.com/tliron/commonlog"
	"github.com/tliron/kutil/reflection"
	"github.com/tliron/puccini/tosca"
)

func (self *ServiceContext) TraverseEntities(log commonlog.Logger, work reflection.EntityWork, traverse reflection.EntityTraverser) {
	if work == nil {
		work = make(reflection.EntityWork)
	}

	// Root
	work.TraverseEntities(self.Root.EntityPtr, traverse)

	// Types
	self.Root.GetContext().Namespace.Range(func(entityPtr tosca.EntityPtr) bool {
		work.TraverseEntities(entityPtr, traverse)
		return true
	})
}
