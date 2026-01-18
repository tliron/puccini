use {
    compris::annotate::*,
    depiction::*,
    floria::*,
    problemo::*,
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

    // Put source in internal URL

    let url_context = UrlContext::new();
    url_context.register_internal_url("/hello_world.yaml".into(), true, None, None, source);

    let source_id = SourceID::URL("internal:///hello_world.yaml".into());

    // Catalog with TOSCA 2.0 dialect

    let mut catalog = Catalog::default();
    tosca_2_0::Dialect::add_to_catalog::<WithAnnotations>(&mut catalog).expect("built_in_sources");

    // Compile source

    let store = InMemoryStore::default();

    let id = compile_tosca_to_floria_with_annotations(
        &source_id,
        &url_context,
        store.clone().as_ref(),
        &Directory::default(),
        &mut catalog,
        &mut FailFast,
    )
    .expect("compile_source")
    .expect("compile_source");

    // Print service template

    let service_template = store.get_vertex_template(&id).expect("get_vertex_template").expect("get_vertex_template");
    service_template.as_depict(&store).print_default_depiction();
}
