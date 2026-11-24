use crate::error::Error;
use std::path::{Path, PathBuf};

pub(crate) struct Project {
    root: PathBuf,
}

impl Project {
    pub(crate) fn discover(start: impl AsRef<Path>) -> Result<Self, Error> {
        let mut current = start
            .as_ref()
            .canonicalize()
            .map_err(Error::from)
            .map_err(|error| error.context("cannot resolve current directory"))?;

        loop {
            if current.join("Cargo.toml").is_file() {
                return Ok(Self { root: current });
            }

            if !current.pop() {
                return Err(Error::new("Cargo.toml not found"));
            }
        }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn cargo_lock(&self) -> PathBuf {
        self.root.join("Cargo.lock")
    }
}
