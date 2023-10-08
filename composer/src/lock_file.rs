use crate::php::php_array::PhpArray;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct LockFile {
    _readme: Vec<String>,
    #[serde(rename = "content-hash")]
    pub content_hash: String,
    pub packages: Vec<Package>,
    pub aliases: Vec<String>, // Not entire sure what this type is yet
    #[serde(rename = "minimum-stability")]
    pub minimum_stability: String,
    #[serde(rename = "stability-flags")]
    #[serde(flatten)]
    pub stability_flags: PhpArray,
    #[serde(rename = "prefer-stable")]
    pub prefer_stable: bool,
    #[serde(rename = "prefer-lowest")]
    pub prefer_lowest: bool,
    #[serde(flatten)]
    pub platform: PhpArray,
    #[serde(rename = "platform-dev")]
    #[serde(flatten)]
    pub platform_dev: PhpArray,
    #[serde(rename = "plugin-api-version")]
    pub plugin_api_version: String,
}

impl TryInto<LockFile> for File {
    type Error = LockFileError;

    fn try_into(mut self) -> Result<LockFile, Self::Error> {
        let mut data = String::new();
        self.read_to_string(&mut data)?;

        let lock_file: LockFile = serde_json::from_str(&data)?;

        Ok(lock_file)
    }
}

#[derive(Debug)]
pub enum LockFileError {
    ReadError(std::io::Error),
    DeserializeError(serde_json::Error),
}

impl From<std::io::Error> for LockFileError {
    fn from(error: std::io::Error) -> Self {
        Self::ReadError(error)
    }
}

impl From<serde_json::Error> for LockFileError {
    fn from(error: serde_json::Error) -> Self {
        Self::DeserializeError(error)
    }
}

impl Display for LockFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to read lock file: {}",
            match self {
                LockFileError::ReadError(error) => error.to_string(),
                LockFileError::DeserializeError(error) => error.to_string(),
            }
        )
    }
}

impl Error for LockFileError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: PackageSource,
    pub dist: PackageDist,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub url: String,
    pub reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageDist {
    #[serde(rename = "type")]
    pub dist_type: String,
    pub url: String,
    pub reference: String,
    pub shasum: String,
}
