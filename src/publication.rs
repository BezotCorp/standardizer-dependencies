#[derive(bincode_next::Decode)]
pub(crate) struct Publication {
    library: String,
    library_version: String,
    bin_version: String,
}

impl Publication {
    pub(crate) fn library(&self) -> &str {
        &self.library
    }

    pub(crate) fn library_version(&self) -> &str {
        &self.library_version
    }

    pub(crate) fn bin_version(&self) -> &str {
        &self.bin_version
    }
}
