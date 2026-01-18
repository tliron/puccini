#!/usr/bin/env python

from io import BytesIO
from os.path import dirname, join
from sys import exit

from puccini import Problem
from puccini.readurl import Context, format_archive_url


def main():
    context = Context()
    context = context.with_base_urls([context.working_dir_url()])

    print("Our base URLs:")
    print([str(url) for url in context.base_urls])

    examples_dir = dirname(dirname(__file__))
    example_file = join(examples_dir, "csar", "online-boutique.tar.zst")

    print(f"\nFile: {example_file}")

    # A "tar:" URL to refer to a file within the archive
    example_url = context.url(format_archive_url("tar", example_file, "TOSCA.meta"))

    print(f"\nURL: {example_url}")

    print("\nIterate lines:")
    with example_url.open() as reader:
        for line in reader:
            print(line, end="")

    print("\nRead lines:")
    with example_url.open() as reader:
        while line := reader.readline():
            print(line, end="")

    print("\nRead all:")
    with example_url.open() as reader:
        print(reader.readall().decode("utf-8"), end="")

    print("\nRead into BytesIO:")
    with example_url.open() as reader:
        buffer = BytesIO()
        reader.readinto(buffer)
        print(buffer.getvalue().decode("utf-8"), end="")

    print("\nRead into bytearray:")
    with example_url.open() as reader:
        buffer = bytearray(1000)
        buffer.resize(reader.readinto(buffer))
        print(buffer.decode("utf-8"), end="")


if __name__ == "__main__":
    try:
        main()
    except Problem as problem:
        print(f"Errors:\n{problem:annotate:pretty}")
        exit(1)
