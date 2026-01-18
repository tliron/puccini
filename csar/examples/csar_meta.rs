use {
    anstream::{println, stdout},
    compris::{annotate::*, normal::*, ser::*, *},
    depiction::*,
    problemo::*,
    puccini_csar::*,
};

pub fn main() {
    let source = br#"CSAR-Version: 2.0
Created-By: Puccini
Entry-Definitions: tosca_elk.yaml
Other-Definitions: "definitions/tosca moose.yaml" definitions/tosca_deer.yaml

MyDef1: Hello
MyDef2: Puccini

Another: One
"#;

    // Note that the TOSCA.meta format is very strict!

    let tosca_meta = ToscaMeta::from_bytes(source, &mut FailFast).expect("from_bytes");
    tosca_meta.print_default_depiction();

    // Stringify with max columns

    let source = tosca_meta.stringify(Some(20)).expect("stringify");
    println!("\n{}", source);

    // Read it back
    // (just to show that a round-trip works)

    let tosca_meta = ToscaMeta::from_string(&source, &mut FailFast).expect("from_string");

    // Into Compris variant and then JSON

    let tosca_meta: Variant<WithoutAnnotations> = tosca_meta.into();
    Serializer::new(Format::JSON).with_pretty(true).write_modal(&tosca_meta, &mut stdout()).expect("write_modal");
}
