use {
    compris::annotate::*,
    depiction::*,
    floria::*,
    kutil::std::error::*,
    puccini_tosca::{dialect::tosca_2_0, grammar::*},
    read_url::*,
};

pub fn main() {
    let source = b"
tosca_definitions_version: tosca_2_0

node_types:
    MyNode: {}

service_template:
    node_templates:
        node:
            type: MyNode
";

    // Add source to URL context as internal URL

    let url_context = UrlContext::new();
    url_context
        .register_internal_url("/hello_world.yaml".into(), true, None, Some("text".into()), source)
        .expect("register_internal_url");

    let source_id = SourceID::URL("internal:///hello_world.yaml".into());

    // Create catalog with TOSCA 2.0 dialect

    let mut catalog = Catalog::default();
    catalog.add_dialect_ref(tosca_2_0::Dialect::default().into());
    catalog.add_sources(tosca_2_0::Dialect::built_in_sources::<WithAnnotations>().expect("built_in_sources"));

    // Load our source

    catalog
        .load_source_with_annotations::<WithAnnotations, _>(&source_id, &url_context, &mut FailFastErrorReceiver)
        .expect("load_source_with_annotations");

    // Complete

    catalog.complete_entities::<WithAnnotations, _>(&mut FailFastErrorReceiver).expect("complete_entities");

    // Compile service template into Floria in-memory store

    let store = InMemoryStore::default();

    let mut errors = FailFastErrorReceiver;
    let directory = Directory::default();
    let mut context =
        CompilationContext::new(&source_id, &catalog, &directory, store.clone().into_ref(), errors.as_ref());

    let service_template_id = catalog
        .compile_service_template(&mut context)
        .expect("compile_service_template")
        .expect("compile_service_template");

    // Print service template

    let service_template =
        store.get_vertex_template(&service_template_id).expect("get_vertex_template").expect("get_vertex_template");
    service_template.as_depict(&store).print_default_depiction();
}
