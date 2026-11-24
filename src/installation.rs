use crate::{download::Download, error::Error};
use reqwest::blocking::Client;
use std::{fs, path::Path};

pub(crate) struct Installation;

impl Installation {
    const REPOSITORY: &'static str =
        "https://raw.githubusercontent.com/BezotCorp/standardizer-data-bin";

    const FILES: [&'static str; 2] = ["routing.bin", "tools.bin"];

    pub(crate) fn run(
        client: &Client,
        project_root: &Path,
        bin_version: &str,
    ) -> Result<(), Error> {
        let data_directory = project_root.join("data");
        fs::create_dir_all(&data_directory)
            .map_err(Error::from)
            .map_err(|error| {
                error.context(format!("cannot create {}", data_directory.display()))
            })?;
        let mut downloads = Vec::new();
        for file_name in Self::FILES {
            let url = format!("{}/v{}/{}", Self::REPOSITORY, bin_version, file_name);
            downloads.push((file_name, Download::fetch(client, &url, &data_directory)?));
        }
        for (file_name, download) in downloads {
            download.persist(&data_directory.join(file_name))?;
        }
        Ok(())
    }
}
