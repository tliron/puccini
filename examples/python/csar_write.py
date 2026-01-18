#!/usr/bin/env python

from io import BytesIO
from os.path import dirname, join
from sys import exit

from puccini import Problem
from puccini.csar import Creator


def main():
    service = b"""
tosca_definitions_version: tosca_2_0
"""

    creator = Creator()

    # When using a writer we have to set TOSCA.meta fields manually
    creator.entry_definitions = "service.yaml"
    creator.additional_other_definitions = ["service2.yaml", "namespaces.yaml"]

    # The default format is a non-compressed tarball ("tar")
    creator.format = "zip"

    # We can write to any file-like object;
    # Note that the ZIP format requires the file-like object to be `seekable()`
    with creator.writer(open("/tmp/csar.zip", "wb")) as writer:
        # `add()` supports bytes, strings, and file-like objects
        writer.add("service.yaml", service)

        # (with file-like objects we *must* pass the size as the third argument)
        writer.add("service2.yaml", BytesIO(service), len(service))

        # prefer `add_file()` for optimized reading from a file path:
        examples_dir = dirname(dirname(__file__))
        example_file = join(examples_dir, "tour", "namespaces.yaml")
        writer.add_file("namespaces.yaml", example_file)

    # Now let's try optimized writing to a buffer

    creator.format = "gz"
    creator.additional_other_definitions = []

    writer = creator.buffer_writer()
    writer.add("service.yaml", service)

    # When we're done we can get the writer's underlying buffer
    buffer = writer.buffer()

    print(f"{creator.format} CSAR is {len(buffer)} bytes")

    with open("/tmp/csar.tar.gz", "wb") as f:
        f.write(buffer)


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        print(f"Errors:\n{problem:annotate:pretty}")
        exit(1)
