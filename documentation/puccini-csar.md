[⇐ to main site](https://puccini.cloud)

Guide to `puccini-csar`
=======================

Creating CSARs
--------------

Create a CSAR from a source directory with `create`. A single CSAR can contain one or more TOSCA service templates and/or TOSCA profiles, as well as arbitrary artifacts such as configuration files, machine images, Floria plugins (Wasm), and other implementations.

The source directory can include nested subdirectories. Symbolic links *will* be followed. Hidden files and directories *will* be skipped.

Supported formats are tarballs (Gzip, Zstandard, or plain) and legacy ZIP files (using DEFLATE compression). Use the `--compression` flag to set the compression level (not used for plain tarballs). Examples:

```sh
# Gzip tarball
# (most portable)
puccini-csar create services/my-service csars/my-service.tar.gz

# Zstandard tarball
# (much faster at lower compression levels, better compression at higher levels)
puccini-csar create services/my-service csars/my-service.tar.zst

# plain tarball (no compression)
puccini-csar create profiles/my-profile csars/my-profile.tar

# ZIP
puccini-csar create profiles/profiles csars/profiles.zip

# legacy ".csar" ZIP extension
puccini-csar create services/my-service csars/my-service.csar
```

Note that timestamps in ZIP file entries are naive (no timezone information). Puccini assumes UTC, but other tools may interpret them differently. For example, the Windows file manager treats them as local time when unpacking.

For tarballs, if you need a compression algorithm other than Gzip and Zstandard it's easy enough to pipe to external tools: When the target file is not provided `create` will output a plain tarball to stdout. Examples:

```sh
# XZ
puccini-csar create services/my-service | xz --verbose > csars/my-service.tar.xz

# Bzip2
puccini-csar create services/my-service | bzip2 > csars/my-service.tar.bz2
```

If you have a `TOSCA.meta` file in the source directory then Puccini will parse and validate it. It can be either in the root of the source directory or in its `TOSCA-Metadata` subdirectory (but not both). But it's not required: If Puccini doesn't find the file then it will insert a `TOSCA.meta` into the CSAR for you. This will only fill in the standard "block_0", so if you need additional blocks then you will have to provide your own `TOSCA.meta` file.

You can explicitly set or add (or override) `TOSCA.meta` keys from the command line via flags. Example:

```sh
puccini-csar create \
  --created-by='My Organization' \
  --entry-definitions=definitions/extra.yaml \
  --entry-definitions='definitions/another extra.yaml' \
  services/my-service \
  csars/my-service.tar.gz
```

If the `Entry-Definitions` key is not provided, neither via a flag nor via a preexisting `TOSCA.meta` file, Puccini will look for a single `.yaml` or `.yml` file in the root of the source directory and set the key accordingly.

Validation includes checking that both `Entry-Definitions` and all the `Other-Definitions` point to existing files within the source directory before creating the CSAR.

Note that `create` will *not* validate the TOSCA contents of the definitions files. For that you can use [`puccini-tosca`](puccini-tosca), either before creating the CSAR or on the created CSAR itself.

Add `--dry-run` to test all these heuristics without actually outputting the CSAR.

When writing `TOSCA.meta`, Puccini will format (or reformat) it to fit in 80 columns by default. Also note that `TOSCA.meta` is always added to the CSAR archive *first*, before other entries. This is an optimization specifically for tarballs (see below). Other file entries are added in alphabetical order to ensure deterministic results.

Validating Metadata
-------------------

Validate and examine a CSAR's `TOSCA.meta` with `meta`.

The command accepts either a file path *or* a URL. A wide variety of URL types is supported, including URLs that access individual entries in remote archives and repositories. In all cases it will ensure that `Entry-Definitions` and all `Other-Definitions` exist as relative paths within the CSAR.

As with `create`, it will *not* validate the TOSCA contents. Again, for that you can use [`puccini-tosca`](puccini-tosca).

Examples:

```sh
# paths
puccini-csar meta csars/my-service.tar.gz

# HTTP
puccini-csar meta https://site.org/csars/my-service.tar.gz
puccini-csar meta https://site.org/csars/my-profile.zip
puccini-csar meta https://site.org/csars/my-service.csar

# an archive inside a remote archive!
# (URL is wrapped in quotes to avoid shell processing of ! character)
puccini-csar meta 'tar:https://site.org/collection.tar!csars/my-service.tar.gz'

# in a remote git repository
puccini-csar meta 'git:https://my-git.org/my-user/my-repo#main!csars/my-service.tar.gz'
```

The `meta` command can output the meta information in a variety of formats. Examples:

```sh
puccini-csar meta csars/my-service.tar.gz --format json

# format can be automatically determined by the output extension
puccini-csar meta https://site.org/csars/my-service.tar.gz --output=meta.yaml
```

Note that when using `meta` with remote CSARs, tarballs have an advantage: The `TOSCA.meta` entry will be read individually by streaming it, such that other entries will be skipped and ignored. (And if it was created by `puccini-csar`, it is guaranteed to be the first tar entry.) By contrast, ZIP files must be *entirely* downloaded to the local filesystem in order to access even one entry, which can be costly for large CSARs.

Accessing Contents
------------------

Because a CSAR is a standard tarball (or ZIP file) your standard tools will work. For example, let's list contents:

```sh
tar --list --file csars/my-service.tar.gz
unzip -l csars/my-service.zip
```

And now let's extract an artifact to stdout:

```sh
tar --extract --to-stdout --file csars/my-service.tar.gz TOSCA.meta
unzip -p csars/my-service.zip TOSCA.meta
```
