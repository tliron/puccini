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

Haven't we been here before?
----------------------------

Yes. Yes, we have. This is the *third* TOSCA parser with Tal Liron's fingerprints.

The first was a group effort in Python: [AriaTosca](https://github.com/apache/incubator-ariatosca), 2016-2017. It was accepted as an incubation project at the Apache Software Foundation and was a component of the Linux Foundation's OPEN-O telecommunications cloud orchestrator, later [merged with ONAP](https://www.linuxfoundation.org/press/press-release/the-linux-foundation-announces-merger-of-open-source-ecomp-and-open-oto-form-new-open-network-automation-platform-onap-project). Work on AriaTosca was funded by Cloudify (currently [part of Dell](https://techcrunch.com/2023/01/25/dell-has-acquired-cloud-orchestration-startup-cloudify-sources-tell-us-for-around-100m/)). Alas, the project did not graduate from incubation and is now archived. May it rest in peace.

Tal first revealed Puccini in 2018 as a personal initiative. It was [initially written in Go](https://github.com/tliron/go-puccini) and used an internal JavaScript interpreter to execute TOSCA functions. This early version of Puccini supports all versions of TOSCA since 1.0 as well as related dialects. TOSCA 2.0 support was contributed by [Dario Mazza](https://github.com/xDaryamo).

Around the release of TOSCA 2.0 in 2025 it became clear that Puccini could use a rewrite. Tal decided to do it in Rust and Wasm, and [this is where work continues](https://github.com/tliron/puccini). So far this new direction has proven to be very excellent. Third time's a charm?

Ever since the AriaTosca days Tal has been an active contributor to the [TOSCA standard](https://www.oasis-open.org/committees/tosca/) and the [TOSCA community](https://github.com/oasis-open/tosca-community-contributions). Join us!
