use crate::{cargo_package::CargoPackage, dependency::Dependency, error::Error};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
pub(crate) struct CargoLock {
    #[serde(default)]
    package: Vec<CargoPackage>,
}

impl CargoLock {
    pub(crate) fn read(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path)
            .map_err(Error::from)
            .map_err(|error| error.context(format!("cannot read {}", path.display())))?;

        toml::from_str(&content).map_err(|error| Error::new(format!("invalid Cargo.lock: {error}")))
    }

    pub(crate) fn dependency(&self, name: &str) -> Result<Dependency, Error> {
        let mut matching = self.package.iter().filter(|package| package.name() == name);

        let package = matching
            .next()
            .ok_or_else(|| Error::new(format!("{name} not found in Cargo.lock")))?;

        if matching.next().is_some() {
            return Err(Error::new(format!(
                "multiple locked versions of {name} found"
            )));
        }

        Ok(Dependency::new(package.name(), package.version()))
    }
}
