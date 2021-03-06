// This file was auto-generated from a YAML file

package v1_0

func init() {
	Profile["/tosca/simple-for-nfv/1.0/artifacts.yaml"] = `
tosca_definitions_version: tosca_simple_yaml_1_2

artifact_types:

  tosca.artifacts.nfv.SwImage:
    metadata:
      tosca.normative: 'true'
      specification.citation: '[TOSCA-Simple-Profile-NFV-v1.0-csd04]'
      specification.location: 5.4.1
    derived_from: tosca.artifacts.Deployment.Image
    properties:
      name:
        description: >-
          Name of this software image.
        type: string
        required: true
      version:
        description: >-
          Version of this software image.
        type: string
        required: true
      checksum:
        description: >-
          Checksum of the software image file.
        type: string
      container_format:
        description: >-
          The container format describes the container file format in which software image is
          provided.
        type: string
        required: true
      disk_format:
        description: >-
          The disk format of a software image is the format of the underlying disk image.
        type: string
        required: true
      min_disk:
        description: >-
          The minimal disk size requirement for this software image.
        type: scalar-unit.size
        required: true
      min_ram:
        description: >-
          The minimal disk size requirement for this software image.
        type: scalar-unit.size
        required: false
      size: # ERRATUM: section [5.4.1.1 Properties] calls this field 'Size'
        description: >-
          The size of this software image
        type: scalar-unit.size
        required: true
      sw_image:
        description: >-
          A reference to the actual software image within VNF Package, or url.
        type: string
        required: true
      operating_system:
        description: >-
          Identifies the operating system used in the software image.
        type: string
        required: false
      supported _virtualization_environment:
        description: >-
          Identifies the virtualization environments (e.g. hypervisor) compatible with this software
          image.
        type: list
        entry_schema:
          type: string
        required: false
`
}
