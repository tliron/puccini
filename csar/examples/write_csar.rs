use {
    puccini_csar::creator::*,
    std::{io, path::*},
};

pub fn main() {
    // We must set up the creator with at least "entry_definitions"
    // The default format is GzipTarball
    // Note that paths for "entry_definitions" and "other_definitions" will *not* be validated
    // You need to make sure to add the entries

    let mut creator = CsarCreator::default();
    creator.format = Some(Format::ZIP);
    creator.entry_definitions = Some("service.yaml".into());

    let mut csar = Vec::default();

    // Note that ZIP requires `seek_writer()` so we need to wrap with a Cursor;
    // For the other formats `stream_writer()` would be enough
    let mut writer = creator.seek_writer(io::Cursor::new(csar)).expect("create_for_writer");

    // Let's add the entry_definitions

    let content = "tosca_definitions_version: tosca_2_0\n";
    writer.add_string("service.yaml", content, creator.compression_level, None).expect("add_string");

    // Let's add an artifact
    // 10 million identical bytes should compress nicely...

    let content = vec![123; 10_000_000];
    writer
        .add_bytes(Path::new("artifacts").join("stuff"), &content, creator.compression_level, None)
        .expect("add_bytes");

    // When we're done, we can get the writer back

    let cursor: io::Cursor<_> = *writer.into_writer().expect("into_writer");
    csar = cursor.into_inner();
    println!("CSAR is {} bytes", csar.len());

    // For demonstration purposes you can write the buffer to a file
    // (you would normally be using `creator.seek_writer` directly to the file)

    // use std::io::Write;
    // let mut file = std::fs::File::create("test.tar.gz").expect("create_file");
    // file.write_all(&csar).expect("write_all");
}
