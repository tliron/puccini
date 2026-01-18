#!/usr/bin/env python

from sys import exit

from puccini import Problem
from puccini.floria import Store
from puccini.tosca import compile_service_template
from readurl import Context


def main():
    source = b"""
tosca_definitions_version: tosca_2_0

node_types:
    MyNode: {}

service_template:
    node_templates:
        node:
            type: MyNode
"""

    # `register_internal_url()` accepts bytes, strings, or I/O objects

    url_context = Context()
    url_context.register_internal_url("/hello_world.yaml", source)

    floria_store = Store()
    service_template_id = compile_service_template(
        "internal:///hello_world.yaml", floria_store, url_context=url_context
    )

    service_template = floria_store.get_vertex_template(service_template_id)
    print(f"{service_template:pretty}")


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        print(f"Errors:\n{problem:annotate:pretty}")
        exit(1)
