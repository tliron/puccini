[⇐ to main site](https://puccini.cloud)

Frequently Asked Questions
==========================

Why is it called "Puccini"?
---------------------------

Named after [Giacomo Puccini](https://en.wikipedia.org/wiki/Giacomo_Puccini), the composer of the [*Tosca*](https://en.wikipedia.org/wiki/Tosca) opera (based on Victorien Sardou's play, [*La Tosca*](https://en.wikipedia.org/wiki/La_Tosca)), as well as *La bohème*, *Madama Butterfly*, and other famous works. The theme here is orchestration, orchestras, composition, and thus operas. Capiche?

How do I pronounce "Puccini"?
-----------------------------

Usually: "poo-CHEE-nee" ("ch" as in "change").

For a demonstration of its authentic 19th-century Italian pronunciation see [this clip](https://www.youtube.com/watch?v=dQw4w9WgXcQ).

What is the history of Puccini?
-------------------------------

The author, Tal Liron, has been an active contributor to the TOSCA standard and the TOSCA community since 2016. This is Tal's *third* TOSCA parser project.

The first effort in 2016-2017 was [AriaTosca](https://github.com/apache/incubator-ariatosca). Written in Python, it was accepted as an incubation project under the Apache Software Foundation. It did not graduate and is now archived. Work on AriaTosca was funded by Cloudify (currently [part of Dell](https://techcrunch.com/2023/01/25/dell-has-acquired-cloud-orchestration-startup-cloudify-sources-tell-us-for-around-100m/)).

Tal started Puccini in 2018 as a personal initiative, [initially written in Go](https://github.com/tliron/go-puccini) with an internal JavaScript interpreter for executing TOSCA functions. This early version of Puccini supports all versions of TOSCA since 1.0 as well as related dialects. TOSCA 2.0 support was contributed by Dario Mazza.

With the release of TOSCA 2.0 in 2025, Tal rewrote Puccini in Rust and Wasm, and [this is where work continues](https://github.com/tliron/puccini).
