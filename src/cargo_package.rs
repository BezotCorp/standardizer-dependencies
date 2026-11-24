use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CargoPackage {
    name: String,
    version: String,
}

impl CargoPackage {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn version(&self) -> &str {
        &self.version
    }
}
