use crate::error::Error;
use reqwest::blocking::Client;
use std::{io, path::Path};
use tempfile::NamedTempFile;

pub(crate) struct Download {
    file: NamedTempFile,
}

impl Download {
    pub(crate) fn fetch(client: &Client, url: &str, directory: &Path) -> Result<Self, Error> {
        std::fs::create_dir_all(directory)
            .map_err(Error::from)
            .map_err(|error| error.context(format!("cannot create {}", directory.display())))?;

        let mut response = client
            .get(url)
            .send()
            .map_err(|error| Error::new(format!("cannot download {url}: {error}")))?;

        if !response.status().is_success() {
            return Err(Error::new(format!(
                "cannot download {url}: HTTP {}",
                response.status()
            )));
        }

        let mut file = NamedTempFile::new_in(directory)
            .map_err(Error::from)
            .map_err(|error| error.context("cannot create temporary download"))?;

        io::copy(&mut response, &mut file)
            .map_err(Error::from)
            .map_err(|error| error.context(format!("cannot write download from {url}")))?;

        Ok(Self { file })
    }

    pub(crate) fn persist(self, destination: &Path) -> Result<(), Error> {
        self.file.persist(destination).map_err(|error| {
            Error::new(format!(
                "cannot install {}: {}",
                destination.display(),
                error.error
            ))
        })?;

        Ok(())
    }
}
