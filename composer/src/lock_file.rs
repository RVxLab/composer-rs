use crate::php::php_array::PhpArray;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LockFile {
    _readme: Vec<String>,
    #[serde(rename = "content-hash")]
    pub content_hash: String,
    pub packages: Vec<Package>,
    #[serde(rename = "packages-dev")]
    pub packages_dev: Vec<Package>,
    pub aliases: PhpArray<String>,
    #[serde(rename = "minimum-stability")]
    pub minimum_stability: String,
    #[serde(rename = "stability-flags")]
    #[serde(flatten)]
    pub stability_flags: PhpArray<u8>,
    #[serde(rename = "prefer-stable")]
    pub prefer_stable: bool,
    #[serde(rename = "prefer-lowest")]
    pub prefer_lowest: bool,
    #[serde(flatten)]
    pub platform: PhpArray<String>,
    #[serde(rename = "platform-dev")]
    #[serde(flatten)]
    pub platform_dev: PhpArray<String>,
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
    pub require: PhpArray<String>,
    pub suggest: Option<HashMap<String, String>>,
    pub conflict: Option<PhpArray<String>>,
    #[serde(rename = "require-dev")]
    pub require_dev: Option<PhpArray<String>>,
    #[serde(rename = "type")]
    pub package_type: String,
    pub extra: Option<HashMap<String, Value>>,
    pub autoload: Option<AutoloadConfig>,
    #[serde(rename = "autoload-dev")]
    pub autoload_dev: Option<AutoloadConfig>,
    #[serde(rename = "notification-url")]
    pub notification_url: String,
    pub license: Vec<String>,
    pub authors: Option<Vec<Author>>,
    pub description: String,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub support: Option<HashMap<String, String>>,
    pub funding: Option<Vec<Funding>>,
    pub time: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AutoloadPath {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoloadConfig {
    pub files: Option<Vec<String>>,
    pub classmap: Option<Vec<String>>,
    #[serde(rename = "psr-0")]
    pub psr0: Option<HashMap<String, AutoloadPath>>,
    #[serde(rename = "psr-4")]
    pub psr4: Option<HashMap<String, AutoloadPath>>,
    #[serde(rename = "exclude-from-classmap")]
    pub exclude_from_classmap: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Funding {
    pub url: String,
    #[serde(rename = "type")]
    pub funding_type: String,
}

#[test]
fn test_simple_lock_file() {
    let lock_file: LockFile = File::open("./fixtures/simple.lock")
        .unwrap()
        .try_into()
        .unwrap();

    dbg!(lock_file);
}
