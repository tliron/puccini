#!/usr/bin/env python

from sys import exit

from puccini import Problem
from puccini.csar import Meta, MetaBlock


def main():
    source = b"""CSAR-Version: 2.0
Created-By: Puccini
Entry-Definitions: tosca_elk.yaml
Other-Definitions: "definitions/tosca moose.yaml" definitions/tosca_deer.yaml

MyDef1: "Hello, world"
MyDef2: Puccini

Another: One
"""

    # The constructor can optionally accept bytes, string, or file-like objects to parse
    meta = Meta(source)

    # Modify the TOSCA.meta
    # (you will get exceptions if you set invalid data)

    extra_blocks = meta.get_extra_blocks()

    for block in extra_blocks:
        block["String"] = "hello"

    block = MetaBlock()
    block["List"] = ["first item", "second"]
    print(f"List:\n{block.get_list('List')}")

    del extra_blocks[1]
    extra_blocks.append(block)

    meta.set_extra_blocks(extra_blocks)

    print(f"\n{meta:pretty}")

    # The optional "." after the "raw" format spec sets max columns
    print(f"\n{meta:raw.20}", end="")

    # Inspect the extra blocks

    print("\nExtra blocks:")
    for n, block in enumerate(meta.get_extra_blocks()):
        print(f"  {n}:")
        for k in block:
            print(f"    {k} = {block[k]}")


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        print(f"Errors:\n{problem:annotate:pretty}")
        exit(1)
