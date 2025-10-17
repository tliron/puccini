use clap::*;

//
// CsarFormat
//

#[derive(Clone, ValueEnum)]
pub enum CsarFormat {
    #[value(name = "tar")]
    Tarball,

    #[value(name = "gz")]
    GzipTarball,

    #[value(name = "zst")]
    ZstandardTarball,

    #[value(name = "zip")]
    Zip,
}

impl CsarFormat {
    /// To Puccini format.
    pub fn to_puccini(&self) -> puccini_csar::creator::Format {
        match self {
            CsarFormat::Tarball => puccini_csar::creator::Format::Tarball,
            CsarFormat::GzipTarball => puccini_csar::creator::Format::GzipTarball,
            CsarFormat::ZstandardTarball => puccini_csar::creator::Format::ZstandardTarball,
            CsarFormat::Zip => puccini_csar::creator::Format::ZIP,
        }
    }
}

impl ToString for CsarFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
