[⇐ to main site](https://puccini.cloud)

Guide to `puccini-tosca`
========================

Compiling TOSCA
---------------

The `compile` command will parse, validate, and compile a TOSCA service template or TOSCA profile into a set of Floria vertex templates, edge templates, and/or classes. This result can be understood as "Day 0" of orchestration. See [this document](floria) for a in-depth discussion of the details.

If there are errors it will print them out in a detailed human-readable format, which includes the object path and row/column span in the TOSCA sources where the error was detected.

Otherwise, *if a Floria service is not specified*, it will print the resulting Floria entities to stdout, again in a human-readable format. (More about this behavior below.)

The argument can be either a TOSCA file (`.yaml`) or a CSAR file. This file can be specified as a filesystem path or a URL. A [wide variety of URL types](https://github.com/tliron/rust-read-url?tab=readme-ov-file#supported-url-types) is supported, including URLs that access individual entries in remote archives and repositories.

Relative paths in TOSCA (e.g. in `imports`) are resolved relative to the location of the initial file, even if it's inside an archive, such as a CSAR (and even if it's inside a *remote* archive).

When the argument is omitted `compile` will process stdin as a TOSCA YAML, though note this feature has limited usability because relative paths, e.g. in `imports`, cannot be resolved.

TODO: other definitions in CSAR?

Examples:

```sh
# TOSCA service template file
puccini-tosca compile services/my-service/service-template.yaml

# TOSCA profile file
puccini-tosca compile services/my-profile/profile.yaml

# local CSAR
puccini-tosca compile csars/my-service.tar.gz

# remote CSAR
puccini-tosca compile https://site.org/csars/my-service.tar.gz

# in a git repository
# (wrapped in quotes to avoid shell processing of ! character)
puccini-tosca compile \
  'git:https://my-git.org/my-user/my-repo#main!my-service/service-template.yaml'

# stdin
cat services/my-service/service-template.yaml | puccini-tosca compile
```

#### Note About the Default Printout

It might seem as if the output of `compile` is a human-readable printout of Floria entities, but that is not in fact the intended use. Actually, what you are seeing is *debug* output, enabled as a side effect of *not* specifying a Floria service (with the `--floria` flag). Otherwise, you could produce this printout explicitly via the `--debug=compiled` flag.

We've chosen to default to debug behavior in order to allow `compile` to be minimally usable without having to stand up a Floria service. What is happening in this case is that Puccini is storing the Floria entities in an in-memory Floria store, which is discarded as soon as the `puccini-tosca` exits. By printing out these entities we are at least producing *some* useful output.

For `compile` to be truly useful for cloud orchestration you would want the results to be stored in a running Floria service, or at least output a format that can be imported into Floria (with the `--format` flag).

Simulating Cloud Instances
--------------------------

The compilation phase will validate much of TOSCA's syntax and grammar—but not all of it, because compilation is "Day 0", and many TOSCA features are intended for "Day 1" and even "Day 2" of orchestration.

Specifically, they are only applicable in the context of TOSCA *topology representations*. This includes satisfying requirements (and forming actual relationships), validating data types (including the `validation` keyword), calling any embedded TOSCA functions in value assignments, as well as calling operations or responding to event notifications.

Moreover, custom TOSCA functions and operation/notification implementation may require loading Wasm plugins or other artifacts. Indeed, *full* validation may only be possible in a real cloud environment, where these plugins and artifacts are expected to interact with running services.

That said, `puccini-tosca` can *simulate* a cloud environment in order to allow for extended (if incomplete) service template validation, which can be understood as "Day 1" of orchestration.

To enable this simulation instantiate the compiled Floria entities into the in-memory Floria store using `--instantiate`:

```sh
puccini-tosca compile \
  services/my-service/service-template.yaml \
  --instantiate
```

This will attempt to satisfy all TOSCA requirements and create actual relationships (Floria edges) between nodes (Floria vertexes). Unsatisfied requirements will result in Floria errors.

Following the "minimally usable" default behavior approach detailed above, this will implicitly enable `--debug=instance` to produce a human-readable printout of the (instantiated) Floria vertexes, edges, and/or classes.

Note that if the TOSCA service template has required `inputs`, that do not have default values, then these must be provided using the `--inputs` or `--input` flags. For example:

```sh
puccini-tosca compile \
  services/my-service/service-template.yaml \
  --instantiate \
  --inputs='{"input1": "value1", "input2": "value2"}'
```

Simulating Events: Day 1
------------------------

TODO

`--update`

Simulating Events: Day 2
------------------------
