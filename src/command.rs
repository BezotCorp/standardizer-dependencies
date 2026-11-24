use crate::{
    cargo_lock::CargoLock, error::Error, installation::Installation, project::Project,
    provenance::Provenance,
};
use reqwest::blocking::Client;
use std::env;

pub(crate) struct Command;

impl Command {
    const LIBRARY: &'static str = "standardizer-common";

    pub(crate) fn run() -> Result<(), Error> {
        let current_directory = env::current_dir()
            .map_err(Error::from)
            .map_err(|error| error.context("cannot read current directory"))?;

        let project = Project::discover(current_directory)?;
        let cargo_lock_path = project.cargo_lock();

        if !cargo_lock_path.is_file() {
            return Err(Error::new(format!(
                "{} not found",
                cargo_lock_path.display()
            )));
        }

        let cargo_lock = CargoLock::read(&cargo_lock_path)?;
        let dependency = cargo_lock.dependency(Self::LIBRARY)?;

        let client = Client::builder()
            .build()
            .map_err(|error| Error::new(format!("cannot create HTTP client: {error}")))?;

        let provenance = Provenance::download(&client)?;

        let bin_version = provenance.latest_bin_version(dependency.name(), dependency.version())?;

        println!(
            "{} {} -> BIN {}",
            dependency.name(),
            dependency.version(),
            bin_version
        );

        Installation::run(&client, project.root(), &bin_version)?;

        println!(
            "installed BIN {} in {}",
            bin_version,
            project.root().join("data").display()
        );

        Ok(())
    }
}
