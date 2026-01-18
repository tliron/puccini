#!/usr/bin/env python

from os.path import dirname, join
from sys import exit

from puccini import Problem
from puccini.csar import Creator


def main():
    creator = Creator()

    # `create_from_directory()` will automatically determine the format via the file extension
    # (unless we set `creator.format` manually);
    # it can will also attempt to automatically select the "entry_definitions" for TOSCA.meta
    # (as long as it's not ambiguous)

    print("Creating from directory...")

    examples_dir = dirname(dirname(__file__))
    example_dir = join(examples_dir, "online-boutique")

    created = creator.create_from_directory("/tmp/csar.tar.zst", example_dir)
    print(f"{created.format} CSAR is {created.size} bytes")


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        print(f"Errors:\n{problem:annotate:pretty}")
        exit(1)
