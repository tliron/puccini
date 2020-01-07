// This file was auto-generated from a YAML file

package v1_0

func init() {
	Profile["/tosca/simple/1.0/policies.yaml"] = `
# Modified from a file that was distributed with this NOTICE:
#
#   Apache AriaTosca
#   Copyright 2016-2017 The Apache Software Foundation
#
#   This product includes software developed at
#   The Apache Software Foundation (http://www.apache.org/).

tosca_definitions_version: tosca_simple_yaml_1_0

policy_types:

  tosca.policies.Root:
    metadata:
      normative: 'true'
      citation: '[TOSCA-Simple-Profile-YAML-v1.0]'
      citation_location: 5.10.1
    description: >-
      This is the default (root) TOSCA Policy Type definition that all other TOSCA base Policy Types
      derive from.

  tosca.policies.Placement:
    metadata:
      normative: 'true'
      citation: '[TOSCA-Simple-Profile-YAML-v1.0]'
      citation_location: 5.10.2
    description: >-
      This is the default (root) TOSCA Policy Type definition that is used to govern placement of
      TOSCA nodes or groups of nodes.
    derived_from: tosca.policies.Root

  tosca.policies.Scaling:
    metadata:
      normative: 'true'
      citation: '[TOSCA-Simple-Profile-YAML-v1.0]'
      citation_location: 5.10.3
    description: >-
      This is the default (root) TOSCA Policy Type definition that is used to govern scaling of
      TOSCA nodes or groups of nodes.
    derived_from: tosca.policies.Root

  tosca.policies.Update:
    metadata:
      normative: 'true'
      citation: '[TOSCA-Simple-Profile-YAML-v1.0]'
      citation_location: 5.10.4
    description: >-
      This is the default (root) TOSCA Policy Type definition that is used to govern update of TOSCA
      nodes or groups of nodes.
    derived_from: tosca.policies.Root

  tosca.policies.Performance:
    metadata:
      normative: 'true'
      citation: '[TOSCA-Simple-Profile-YAML-v1.0]'
      citation_location: 5.10.5
    description: >-
      This is the default (root) TOSCA Policy Type definition that is used to declare performance
      requirements for TOSCA nodes or groups of nodes.
    derived_from: tosca.policies.Root
`
}
