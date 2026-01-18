#!/usr/bin/env python

import os

from rich import print
from rich.pretty import Pretty

from puccini import csar, floria, tosca, url


def pprint(value):
    print(Pretty(value, indent_size=2))


def main():
    examples_dir = os.path.dirname(os.path.dirname(__file__))
    example_file = os.path.join(examples_dir, "csar", "online-boutique.tar.zst")

    # TODO
    pprint(csar.create(example_file))

    # The optional True means we include the current working directory as a base path
    # (defaults to False)
    url_context = url.Context(True)

    # Without arguments it will be an in-memory store
    store = floria.Store()

    # `directory` and `url_context` are both optional
    id = tosca.compile_service_template(
        example_file, store, directory="boutique", url_context=url_context
    )

    pprint(store.get_vertex_template(id))


if __name__ == "__main__":
    main()
