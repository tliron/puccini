#!/usr/bin/env python

from puccini import Problem
from puccini.floria import Store
from puccini.readurl import Context
from puccini.tosca import compile_service_template


def main():
    # We're purposely putting errors in this TOSCA source

    source = b"""
tosca_definitions_version: tosca_2_0

node_types:
    MyNode:
        unknown_keyname: 123

service_template:
    node_templates:
        node:
            type: UnknownNode
"""

    url_context = Context()
    url_context.register_internal_url("/hello_world.yaml", source)
    compile_service_template("internal:///hello_world.yaml", Store(), url_context=url_context)


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        for problem in problem.as_unique_problems():
            print(f"⦁︎ Error:  {problem:pretty}")
            if annotations := problem.annotations:
                print(f"  Path:   {annotations.path}")
                print(f"  Source: {annotations.source}", end="")
                if start := annotations.start:
                    print(f" §{start.row}:{start.column}", end="")
                    if end := annotations.end:
                        print(f"→{end.row}:{end.column}", end="")
                print()
        # exit(1)
