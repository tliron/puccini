#!/usr/bin/env python

from puccini import csar, problemo, readurl


def main():
    # TODO
    # pprint(csar.create(example_file))
    pass


if __name__ == "__main__":
    try:
        main()
    except problemo.Problem as problem:
        print(f"\nErrors:\n{problem}")
