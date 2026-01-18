#!/usr/bin/env python

import os

from puccini import problemo, readurl


def main():
    context = readurl.Context()
    context = context.with_base_urls([context.working_dir_url()])

    print("Our base URLs:")
    print([str(url) for url in context.base_urls])

    examples_dir = os.path.dirname(os.path.dirname(__file__))
    example_file = os.path.join(examples_dir, "csar", "online-boutique.tar.zst")

    print("\nFile:")
    print(example_file)

    # A "tar:" URL to refer to a file within the archive
    example_url = context.url(
        readurl.format_archive_url("tar", example_file, "TOSCA.meta")
    )

    print("\nURL:")
    print(example_url)

    print("\nRead from URL:")
    with example_url.open() as reader:
        for line in reader:
            print(line.rstrip())

        # This also works:
        # while line := reader.readline():
        #    print(line.rstrip())

        # This also works:
        # print(reader.readall().decode("utf-8").rstrip())

        # This also works:
        # buffer = bytearray(1000)
        # buffer.resize(reader.readinto(buffer))
        # print(buffer.decode("utf-8").rstrip())


if __name__ == "__main__":
    try:
        main()
    except problemo.Problem as problem:
        print(f"\nErrors:\n{problem}")
