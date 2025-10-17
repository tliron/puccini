use {
    compris::{annotate::*, normal::*, ser::*, *},
    kutil::{cli::depict::*, std::error::*},
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

    // Note that the CSAR meta format is very strict!

    let meta = Meta::from_bytes(source, &mut FailFastErrorRecipient).expect("Meta::from_bytes");
    meta.print_default_depiction();

    // Stringify with max columns

    let source = meta.stringify(Some(20)).expect("Meta::stringify");
    println!("\n{}", source);

    // Read it back
    // (just to show that a round-trip works)

    let meta = Meta::from_string(&source, &mut FailFastErrorRecipient).expect("Meta::from_string");

    // Into Compris variant and then JSON

    let meta: Variant<WithoutAnnotations> = meta.into();
    Serializer::new(Format::JSON).with_pretty(true).print(&meta).expect("print");
}
