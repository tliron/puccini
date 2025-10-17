use {clap::*, compris::*};

//
// OutputFormat
//

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    YAML,
    JSON,
    XJSON,
    //XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
    Depict,
}

impl OutputFormat {
    /// To Compris format.
    pub fn to_compris(&self) -> Option<Format> {
        match self {
            OutputFormat::YAML => Some(Format::YAML),
            OutputFormat::JSON => Some(Format::JSON),
            OutputFormat::XJSON => Some(Format::XJSON),
            //OutputFormat::XML => Some(Format::XML),
            OutputFormat::CBOR => Some(Format::CBOR),
            OutputFormat::MessagePack => Some(Format::MessagePack),
            OutputFormat::Depict => None,
        }
    }
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
