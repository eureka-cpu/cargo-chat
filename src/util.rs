use std::{io, path, process};

use serde::Deserialize;

/// The deserialized output of `cargo metadata`.
#[derive(Debug, Deserialize)]
pub(crate) struct CargoMetadata {
    /// Dependencies of the workspace root.
    pub(crate) packages: Vec<Crate>,
    /// List of paths to workspace members.
    pub(crate) workspace_members: Vec<path::PathBuf>,
}
impl CargoMetadata {
    /// Runs the `cargo metadata` command, returning its output.
    pub(crate) fn output() -> io::Result<process::Output> {
        process::Command::new("cargo").arg("metadata").output()
    }
}
impl TryFrom<&[u8]> for CargoMetadata {
    // TODO: See if there's a library for zero-copy deserialization
    type Error = serde_json::Error;
    fn try_from(stdout: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(stdout)
    }
}

/// Individual crate data deserialized from `cargo metadata`.
#[derive(Debug, Deserialize)]
pub(crate) struct Crate {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) manifest_path: String,
}
