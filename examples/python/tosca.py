#!/usr/bin/env python


from os.path import dirname, join
from sys import exit

import rich
import rich.pretty
from puccini import Problem
from puccini.floria import Store
from puccini.readurl import Context
from puccini.tosca import compile_service_template


def pprint(value):
    rich.print(rich.pretty.Pretty(value, indent_size=2))


def main():
    examples_dir = dirname(dirname(__file__))
    # example_file = join(examples_dir, "csar", "online-boutique.tar.zst")
    example_file = join(examples_dir, "tour", "namespaces.yaml")

    print(f"Compiling: {example_file}")

    # Creating a Floria store without arguments will result in an in-memory store
    floria_store = Store()

    # Using the same URL context with all API calls that support it will allow for caching
    # (otherwise a new URL context will be created per call)
    url_context = Context()

    # The `directory` (target Floria directory) and `url_context` arguments are both optional;
    # Note that we can use file paths or any URL type compatible with readurl;
    # It can be an individual YAML file or a complete CSAR file

    service_template_id = compile_service_template(
        example_file, floria_store, directory="boutique", url_context=url_context
    )

    service_template = floria_store.get_vertex_template(service_template_id)

    print(f"\n{service_template:pretty}")

    # `cps()` converts to Composite Primitive Schema (strings, integers, lists, dicts, etc.);
    # use `cps(True)` to embed contained entities
    print("\nAs CPS:")
    pprint(service_template.cps())


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        print(f"Errors:\n{problem:annotate:pretty}")
        exit(1)
