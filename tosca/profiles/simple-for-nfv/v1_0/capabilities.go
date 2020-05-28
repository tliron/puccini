// This file was auto-generated from a YAML file

package v1_0

func init() {
	Profile["/tosca/simple-for-nfv/1.0/capabilities.yaml"] = `
tosca_definitions_version: tosca_simple_yaml_1_2

imports:
- data.yaml

capability_types:

  tosca.capabilities.nfv.VirtualBindable:
    metadata:
      puccini.normative: 'true'
      specification.citation: '[TOSCA-Simple-Profile-NFV-v1.0-csd04]'
      specification.location: 5.5.1
    description: >-
      A node type that includes the VirtualBindable capability indicates that it can be pointed by
      tosca.relationships.nfv.VirtualBindsTo relationship type.
    derived_from: tosca.capabilities.Node

  tosca.capabilities.nfv.Metric:
    metadata:
      puccini.normative: 'true'
      specification.citation: '[TOSCA-Simple-Profile-NFV-v1.0-csd04]'
      specification.location: 5.5.2
    description: >-
      A node type that includes the Metric capability indicates that it can be monitored using an
      nfv.relationships.Monitor relationship type.
    derived_from: tosca.capabilities.Endpoint

  tosca.capabilities.nfv.VirtualCompute:
    metadata:
      puccini.normative: 'true'
      specification.citation: '[TOSCA-Simple-Profile-NFV-v1.0-csd04]'
      specification.location: 5.5.3
    derived_from: tosca.capabilities.Root
    properties:
      requested_additional_capabilities:
        # ERRATUM: in section [5.5.3.1 Properties] the name of this property is
        # "request_additional_capabilities", and its type is not a map, but
        # tosca.datatypes.nfv.RequestedAdditionalCapability
        description: >-
          Describes additional capability for a particular VDU.
        type: map
        entry_schema:
          type: tosca.datatypes.nfv.RequestedAdditionalCapability
        required: false
      virtual_memory:
        description: >-
          Describes virtual memory of the virtualized compute.
        type: tosca.datatypes.nfv.VirtualMemory
        required: true
      virtual_cpu:
        description: >-
          Describes virtual CPU(s) of the virtualized compute.
        type: tosca.datatypes.nfv.VirtualCpu
        required: true
`
}
