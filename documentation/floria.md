[⇐ to main site](https://puccini.cloud)

Puccini and Floria
==================

[TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) is, at face value, a language for describing *templates*, and indeed the Puccini compiler output is [Floria](https://floria.khutulun.org) templates, specifically Floria vertex templates and edge templates.

However, since TOSCA 2.0 it doesn't stop at templates. Its [operational model](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html#tosca-operational-model) specifies an association between these templates and their "topology representations". In Puccini these topology representations are implemented as Floria vertexes and edges.

Here is a high-level summary of TOSCA to Floria mappings. The rest of this document will elaborate on these:

| TOSCA entity                   | Floria entity                                                            |
| ------------------------------ | ------------------------------------------------------------------------ |
| type                           | class                                                                    |
| service template               | vertex template                                                          |
| node template                  | vertex template (contained in the service template's vertex template)    |
| capability                     | vertex template (contained in a node template's vertex template)         |
| artifact                       | vertex template (contained in a node template's vertex template)         |
| group                          | vertex template                                                          |
| policy                         | vertex template                                                          |
| requirement + relationship     | edge template (connects a node template to a capability vertex template) |
| property, attribute, parameter | property "value" or "updater" in vertex or edge                          |

| TOSCA topology representation  | Floria entity                                        |
| ------------------------------ | ---------------------------------------------------- |
| type                           | class                                                |
| service                        | vertex                                               |
| node                           | vertex (contained in the service's vertex)           |
| capability                     | vertex (contained in a node's vertex)                |
| artifact                       | vertex (contained in a node's vertex)                |
| group                          | vertex                                               |
| policy                         | vertex                                               |
| relationship                   | edge (connects a node vertex to a capability vertex) |
| property, attribute, parameter | property "value" or "updater" in vertex or edge      |

Because most TOSCA functions operate in the world of topology representations, in order to call them with Puccini you would need to not only *compile* the TOSCA, but also then *instantiate* the resulting Floria templates.

In an orchestration environment you would be doing this on a running Floria service, but that's not necessary for simple validation and testing. Unless you specifically point Puccini at a Floria service, it will by default use an in-memory Floria store. `puccini-tosca compile` will compile into this store and then dump the resulting Floria templates to stdout. If you add the `--instantiate` flag then it will also instantiate those templates, again dumping them to stdout. Add the `--update` flag to also update all the instance properties, which will finally call your embedded TOSCA functions.

Shortcut to apply both flags: `puccini-tosca compile -iu`.

Design Principles
-----------------

An important design principle of Floria is that instances should not depend on templates. In other words, the template is optional: it can be deleted after instantiation, and indeed instances can be created directly without a template. This guarantees the fullest freedom for "Day 2" operations to modify the orchestration state even if we have not anticipated the modification with a design template. One way to guide and restrict this Day 2 behavior is to apply policies, which could potentially include design templates. The point is that templates can be useful even if they are unnecessary, and TOSCA provides a powerful language for designing them.

A related design principle of Floria is that entities are untyped. Floria can associate entities with "classes", but these are explicitly *not* types, rather they are meant for organization, categorization, selection, indexing, etc. In other words, classes are metadata rather than data. Like Floria templates, they are optional.

Puccini adheres to these design principles by ensuring that all its generated Floria templates are *self-contained*. Specifically, TOSCA type information is stored entirely within the Floria templates and *nowhere else*. This includes TOSCA data types, meaning that a Floria property compiled from a TOSCA attribute has everything it needs to validate its data schema.

The details and implications of these decisions are discussed below.

Types
-----

TOSCA types become Floria classes.

Note that Floria offers no intrinsic handling of class inheritance. For this reason, each Floria entity (vertex templates, edge templates, properties) will be associated with not only the class representing its nominal TOSCA type, but also all the classes representing the type's ancestors. For example, if a node is of type "VirtualMachine", which inherits from "Compute", that node will be associated with *both* the "VirtualMachine" *and* the "Compute" classes.

This allows for efficient indexing. Selecting the "Compute" class will include all vertexes that are also of type "VirtualMachine". Because each class association is a simple ID, this is an efficient mechanism.

Service Template, Node Templates, and Capabilities
--------------------------------------------------

Puccini generates 3 levels of Floria vertex template nestings:

At the bottom, the TOSCA service template becomes a single Floria vertex template. Contained within it, each TOSCA node template also becomes a Floria vertex template. If the node has capabilities, each capability *also* becomes a Floria vertex template. This final step allows Floria edges to connect to capabilities.

Groups and Policies
-------------------

TODO: both a vertex template and a class?

Requirements and Relationships
------------------------------

TOSCA requirements become Floria edge templates. Because a TOSCA relationship is always contained in a requirement, that relationship is embedded in the edge template rather than as a separate entity.

When this Floria edge template is instantiated, the Floria edge is the implementation of the "relationship representation" in the TOSCA operational model.

TODO: finding the target capability

Properties, Attributes, and Parameters
--------------------------------------

All of these become Floria properties.

### Functions in Values

When you assign a value to a property, attribute, or parameter in TOSCA (including in the `default` and `value` keynames), you are allowed to embed function calls.

If there are no function calls (a "purely literal" value), Puccini optimizes by simply placing the value as is in the Floria property value.

Otherwise, Puccini sets the expression as the Floria property's "updater". The "updater" is called whenever we issue an update operation on the property. All the embedded functions will be called, and if successful the result will be a candidate for the value. (The "preparer" is then called to ensure that this candidate is valid. See below.)

TOSCA properties are marked as read-only Floria properties. As such that they can only be updated once. An update operation normally triggered during instantiation, thus locking the property values in place.

### Data Types

TOSCA's powerful data validation requires special handling.

Because each Floria property would be associated with the Floria classes representing its TOSCA data type, we could have used the class to store schema information that would apply to all properties of that type. For example, a custom scalar data type would store its unit and prefix tables there.

However, Floria classes are not meant to store data: they only have metadata. They are meant to be used for selection, e.g. for applying policies, transformations, etc., on entire classes of entities.

Thus we opted to *not* store schema information in the class and instead store it in the property. The disadvantage is that this data is duplicated for each property of that type. However, the important advantage is that we allow for properties to be self-contained, following the design principles discussed above. Individual property schemas can be modified, and indeed be moved between classes, without affecting other properties of that type. This also has a performance advantage as classes do not need to be retrieved from the Floria store in order to apply schemas.

### `$puccini:schema`

Puccini implements the schema by introducing a handful of internal functions in a built-in `puccini` profile. You can import it explicitly:

```
imports:
- profile: puccini
  namespace: puccini
```

Central is the `$puccini:apply` function, which applies a sequence of *coercion* expressions (which also, as a side effect, act as validators) to the familiar TOSCA `$value`.

The most important coercion function is `$puccini:schema`, which coerces any value to adhere to a schema descriptor. This descriptor contains all the TOSCA data type information: primitive type validation, required properties, default values, key and entry schema for collections, special types (timestamp, version, and scalar—which has its own special schema), and of course arbitrary function calls via the user-defined `validation` keyname. All of these can be nested, too, for lists, maps, and "structs" (data types with `properties`).

Note that for TOSCA parameters the `type` keyname is optional. In other words, they can be untyped, which simply means that they will not use the `$puccini:schema` function. (They might still have `validation`, though; see below.)

The schema descriptor ends up having a non-trivial design because, unlike system programming languages, TOSCA allows for recursive data types. For example, a struct data type can have a field which is of the same type. If we were to naively nest these two schema descriptors we would hit infinite recursion, so instead Puccini's schema descriptor is organized as a list of descriptors, such that any descriptor can refer to another descriptor by a numerical index. This moves the application of nesting to runtime, where the bounds of recursion are limited by the (finite) size of the value itself.

Note that because the TOSCA `validation` keyname expects a boolean expression, it must be turned into a coercion expression in order to be used in `$puccini:apply`. We do this by wrapping it in an `$puccini:assert` function, which simply raises an error if the expression does not evaluate to true.

The final complex expression, which may combine `$puccini:apply`, `$puccini:schema`, and `$puccini:assert` calls, is set as the Floria property's "preparer", which is evaluated whenever its value is updated.

When does an update happen? If you recall from discussion above, a TOSCA property, attribute, or parameter might have functions embedded in its value assignment, which will then be inserted in the Floria property "updater". This is the "pull" approach to update. However, various orchestration events can cause properties to be "pushed" in from external data. This is the intended use for TOSCA attributes, and indeed TOSCA provides another way of updating attributes: interface notifications (see below). Whatever the source of the update, the "preparer" will always be called to ensure that the value is valid.

Here's an example of a "preparer" with a schema descriptor compiled from [`data-types.yaml`](https://github.com/tliron/puccini/blob/main/examples/2.0/data-types.yaml):

```
_apply(
  _schema([
    {
      "kind": "list",
      "entry": 6
    },
    "float",
    "integer",
    {
      "kind": "struct",
      "fields": {
        "nested-bytes": [4, false],
        "nested-float": 5
      }
    },
    "bytes",
    {
      "kind": "float",
      "validation": &_assert(greater_or_equal(value(), 0.0))
    },
    {
      "kind": "struct",
      "fields": {
        "float": [1, false],
        "integer": 2,
        "nested": 3,
        "self": [6, false],
        "string": 7
      }
    },
    {
      "kind": "string",
      "default": "Default Value"
    }
  ])
)
```

Careful observers will note that it may not be immediately clear *when* functions should be called. Some functions, specifically those in TOSCA `validation` expressions, would be called "on demand", and may indeed be called more than once, e.g. when validating the items of a list. Fortunately, Floria supports passing functions "by value", by marking them for "lazy" execution (marked with a `&` prefix above). While this difference cannot be expressed in TOSCA 2.0, Puccini makes use of it internally when constructing the "preparer" expression.

Built-In Functions
------------------

All of TOSCA's built-in functions are provided as a single Wasm file. They are written in Rust using the [Floria Plugin SDK](https://floria.khutulun.org). This Wasm is embedded in the Puccini executable for convenience and will be delivered to a running Floria service during compilation.

Note that these are *Floria functions* and so they work with the Floria graph of vertexes, edges, and their properties (the TOSCA "topology representations"). It is thus relatively straightforward to implement TOSCA Path functions, such as `$get_property` and `$get_attribute`, as they can straightforwardly traverse the graph. We provide a general-purpose TOSCA Path parser/follower that can be reused by other functions.

The comparison functions (`$less_than`, `$greater_or_equal`, etc.) are a bit more subtle, specifically when it comes to comparing the special TOSCA types: version, timestamp, and scalar, because TOSCA allows you to compare these to their unparsed forms, e.g. `{ $less_than: [ $value, "2 GB" ] }`. Thus each of these must provide a specialized comparison implementation. Moreover, scalars must embed their schema so that the other expression could be parsed to a comparable form. The scalar schema is part of the schema descriptor detailed above.

By wonderful coincidence, Floria supports a custom data type exactly for these kinds of situations. We can thus simply mark our special types as custom so that we know to treat them specially.

Custom Functions
----------------

Since TOSCA 2.0, you can formally declare [custom functions](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html#104-function-definitions-).

With Puccini, these can be implemented in the same way we've implemented the built-in functions: Use the Floria Plugin SDK to program your own functions into a Wasm file. Include it in your CSAR, and that's it. Puccini will send it to the running Floria service in addition to the built-in Wasm.

If you're just testing locally this will work, too, using the in-memory Floria store included in Puccini.

Interfaces, Operations, and Notifications
-----------------------------------------

TODO: Notifications -> Floria property updater?

Substitution Mapping
--------------------

TODO
