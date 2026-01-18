#!/usr/bin/env python

import os

import rich
import rich.pretty

from puccini import floria, problemo, readurl, tosca


def pprint(value):
    rich.print(rich.pretty.Pretty(value, indent_size=2))


def main():
    # Using the same URL context with all API calls that support it will allow for caching
    # (otherwise a new URL context will be created per call)
    url_context = readurl.Context()

    # Creating a Floria store without arguments will result in an in-memory store
    store = floria.Store()

    examples_dir = os.path.dirname(os.path.dirname(__file__))
    # example_file = os.path.join(examples_dir, "csar", "online-boutique.tar.zst")
    example_file = os.path.join(examples_dir, "tour", "policies-and-groups.yaml")

    print(f"Compiling: {example_file}")

    # The `directory` and `url_context` arguments are both optional
    id = tosca.compile_service_template(
        example_file, store, directory="boutique", url_context=url_context
    )

    # Setting embed to True will embed contained entities (defaults to False)
    service_template = store.get_vertex_template(id, embed=False)

    print("\nFloria service template:")
    pprint(service_template)


if __name__ == "__main__":
    try:
        main()
    except problemo.Problem as problem:
        print(f"\nErrors:\n{problem}")
        # for problem in problem.as_unique_problems():
        #     if annotations := problem.annotations:
        #         print(annotations)
        #         print(annotations.path)
        #         print(annotations.start.row)
