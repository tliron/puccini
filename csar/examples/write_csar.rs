use {puccini_csar::creator::*, std::path::*};

pub fn main() {
    // We must set up the creator with at least entry_definitions
    // The default format is GzipTarball
    // Note that paths for entry_definitions and other_definitions will *not* be validated
    // You need to make sure to add the entries

    let mut creator = CsarCreator::default();
    creator.entry_definitions = Some("service.yaml".into());

    let mut csar = Vec::default();
    let mut archive = creator.create_for_writer(csar).expect("create_for_writer");

    // Note that create_for_writer does not support Format::ZIP;
    // Use create_for_seek_writer with a File instead
    //
    // let file = std::fs::File::create("test.zip").expect("File::create");
    // let mut archive = creator.create_for_seek_writer(file).expect("create_for_seek_writer");

    // Let's add the entry_definitions

    let content = "tosca_definitions_version: tosca_2_0\n";
    archive.add_string("service.yaml", content, creator.compression_level, None).expect("add_string");

    // Let's add an artifact
    // 10 million identical bytes should compress nicely...

    let content = vec![123; 10_000_000];
    archive
        .add_bytes(Path::new("artifacts").join("stuff"), &content, creator.compression_level, None)
        .expect("add_bytes");

    // When we're done, we can get the writer back

    csar = *archive.into_writer().expect("into_writer");
    println!("CSAR is {} bytes", csar.len());

    // For demonstration purposes you can write the buffer to a file
    // (you would normally be using creator.create_for_seek_writer directly to the file)

    // use std::io::Write;
    // let mut file = std::fs::File::create("test.tar.gz").expect("create_file");
    // file.write_all(&buffer).expect("write_all");
}
