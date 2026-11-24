use crate::{error::Error, publication::Publication};
use bincode_next::{config, decode_from_slice};
use reqwest::blocking::Client;
use semver::Version;

#[derive(bincode_next::Decode)]
pub(crate) struct Provenance {
    publications: Vec<Publication>,
}

impl Provenance {
    const URL: &'static str = "https://raw.githubusercontent.com/BezotCorp/standardizer-data-orchestrator/main/provenance.bin";

    pub(crate) fn download(client: &Client) -> Result<Self, Error> {
        let response = client
            .get(Self::URL)
            .send()
            .map_err(|error| Error::new(format!("cannot download provenance: {error}")))?;

        if !response.status().is_success() {
            return Err(Error::new(format!(
                "cannot download provenance: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .map_err(|error| Error::new(format!("cannot read provenance: {error}")))?;

        let (provenance, consumed): (Self, usize) =
            decode_from_slice(bytes.as_ref(), config::standard())
                .map_err(|error| Error::new(format!("cannot decode provenance: {error}")))?;

        if consumed != bytes.len() {
            return Err(Error::new("provenance contains trailing binary data"));
        }

        Ok(provenance)
    }

    pub(crate) fn latest_bin_version(
        &self,
        library: &str,
        library_version: &str,
    ) -> Result<String, Error> {
        let mut latest: Option<(&Publication, Version)> = None;

        for publication in self.publications.iter().filter(|publication| {
            publication.library() == library && publication.library_version() == library_version
        }) {
            let version = Version::parse(publication.bin_version()).map_err(|error| {
                Error::new(format!(
                    "invalid BIN version {}: {error}",
                    publication.bin_version()
                ))
            })?;

            match &latest {
                Some((_, current)) if current >= &version => {}
                _ => latest = Some((publication, version)),
            }
        }

        latest
            .map(|(publication, _)| publication.bin_version().to_owned())
            .ok_or_else(|| {
                Error::new(format!(
                    "no BIN publication found for {library} {library_version}"
                ))
            })
    }
}
