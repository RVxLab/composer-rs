use clap::ValueEnum;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub allow_plugins: PackageSetting<bool>,
    pub allow_superuser: bool,
    pub apcu_autoloader: bool,
    pub archive_dir: Box<Path>,
    pub archive_format: String,
    pub audit: AuditConfig,
    pub autoloader_suffix: Option<String>,
    pub bearer: Option<HashMap<String, String>>,
    pub bin_compat: BinaryCompatibility,
    pub bin_dir: Box<Path>,
    pub bitbucket_oauth: Option<HashMap<String, BitbucketToken>>,
    pub cache_dir: Box<Path>,
    pub cache_files_dir: Box<Path>,
    /// The max cache files in MiB
    pub cache_files_maxsize: u32,
    pub cache_files_ttl: Duration,
    pub cache_read_only: bool,
    pub cache_repo_dir: Box<Path>,
    pub cache_vcs_dir: Box<Path>,
    pub cafile: Option<Box<Path>>,
    pub capath: Option<Box<Path>>,
    pub classmap_authoritative: bool,
    pub composer_home: Box<Path>,
    pub composer_json: Box<Path>,
    pub composer_lock: Box<Path>,
    pub data_dir: Box<Path>,
    pub disable_tls: bool,
    pub discard_changes: DiscardChanges,
    pub github_domains: Vec<String>,
    pub github_expose_hostname: bool,
    pub github_oauth: Option<HashMap<String, String>>,
    pub github_protocols: Vec<String>,
    pub gitlab_domains: Vec<String>,
    pub gitlab_oauth: Option<HashMap<String, String>>,
    pub gitlab_protocol: Option<String>,
    pub gitlab_tokens: Option<HashMap<String, GitlabToken>>,
    pub htaccess_protect: bool,
    pub http_basic: Option<HashMap<String, HttpBasicToken>>,
    pub lock: bool,
    pub notify_on_install: bool,
    pub optimize_autoloader: bool,
    pub platform: Option<HashMap<String, String>>,
    pub platform_check: PlatformCheck,
    pub preferred_install: PackageSetting<PreferredInstallMethod>,
    pub prepend_autoloader: bool,
    pub process_timeout: u32,
    pub secure_http: bool,
    pub secure_svn_domains: Vec<String>,
    pub sort_packages: bool,
    pub store_auths: Confirmation,
    pub use_github_api: bool,
    pub use_include_path: bool,
    pub use_parent_dir: Confirmation,
    pub vendor_dir: Box<Path>,
}

impl Config {
    pub fn build() -> Result<Self, ConfigError> {
        let mut config = Config::default()?;

        Ok(config)
    }
    fn default() -> Result<Self, ConfigError> {
        let cwd = std::env::current_dir()?;
        let vendor_dir = cwd.join("vendor");

        let (composer_json, composer_lock) = determine_composer_file_paths(&cwd);

        let composer_home = composer_home()?;

        Ok(Self {
            allow_plugins: PackageSetting::default(),
            allow_superuser: false,
            apcu_autoloader: false,
            archive_dir: Box<Path>,
            archive_format: "tar".into(),
            audit: AuditConfig {
                ignore: None,
                abandoned: AuditAbandoned::default(),
            },
            autoloader_suffix: None,
            bearer: None,
            bin_compat: BinaryCompatibility::default(),
            bin_dir: vendor_dir.join("bin").into_boxed_path(),
            bitbucket_oauth: None,
            cache_dir: Box<Path>,
            cache_files_dir: Box<Path>,
            cache_files_maxsize: 300,
            cache_files_ttl: Duration::from_secs(15_552_000), // 6 months
            cache_read_only: false,
            cache_repo_dir: Box<Path>,
            cache_vcs_dir: Box<Path>,
            cafile: None,
            capath: None,
            classmap_authoritative: false,
            composer_home: composer_home.into_boxed_path(),
            composer_json,
            composer_lock,
            data_dir: Box<Path>,
            disable_tls: false,
            discard_changes: DiscardChanges::default(),
            github_domains: vec!["github.com".to_string()],
            github_expose_hostname: true,
            github_oauth: None,
            github_protocols: vec!["https".to_string(), "ssh".to_string(), "git".to_string()],
            gitlab_domains: vec!["gitlab.com".to_string()],
            gitlab_oauth: None,
            gitlab_protocol: None,
            gitlab_tokens: None,
            htaccess_protect: true,
            http_basic: None,
            lock: true,
            notify_on_install: true,
            optimize_autoloader: false,
            platform: None,
            platform_check: PlatformCheck::default(),
            preferred_install: PackageSetting::Global(PreferredInstallMethod::default()),
            prepend_autoloader: true,
            process_timeout: 300,
            secure_http: true,
            secure_svn_domains: Vec::new(),
            sort_packages: false,
            store_auths: Confirmation::default(),
            use_github_api: true,
            use_include_path: true,
            use_parent_dir: Confirmation::default(),
            vendor_dir: vendor_dir.into_boxed_path(),
        })
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not determine COMPOSER_HOME location")]
    CannotDetermineComposerHome,
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum PreferredInstallMethod {
    #[default]
    Dist,
    Source,
    Auto,
}

#[derive(Debug)]
pub enum PackageSetting<T> {
    Selective(HashMap<String, T>),
    Global(T),
}

impl<T> Default for PackageSetting<T> {
    fn default() -> Self {
        Self::Selective(HashMap::new())
    }
}

#[derive(Debug)]
pub struct AuditConfig {
    pub ignore: Option<AuditIgnore>,
    pub abandoned: AuditAbandoned,
}

#[derive(Debug)]
pub enum AuditIgnore {
    Simple(Vec<String>),
    Detailed(HashMap<String, String>),
}

#[derive(Debug, Default)]
pub enum AuditAbandoned {
    #[default]
    Fail,
    Ignore,
    Report,
}

#[derive(Debug, Default)]
pub enum Confirmation {
    #[default]
    Prompt,
    Always,
    Never,
}

#[derive(Debug)]
pub enum GitlabToken {
    Token(String),
    TokenWithUsername(GitlabTokenWithUsername),
}

#[derive(Debug)]
pub struct GitlabTokenWithUsername {
    pub username: String,
    pub token: String,
}

#[derive(Debug)]
pub struct BitbucketToken {
    pub consumer_key: String,
    pub consumer_secret: String,
}

#[derive(Debug)]
pub struct HttpBasicToken {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default)]
pub enum BinaryCompatibility {
    #[default]
    Auto,
    Proxy,
    Full,
}

#[derive(Debug, Default)]
pub enum DiscardChanges {
    #[default]
    Never,
    Always,
    Stash,
}

#[derive(Debug, Default)]
pub enum PlatformCheck {
    #[default]
    PhpOnly,
    All,
    None,
}

fn determine_composer_file_paths(cwd: &PathBuf) -> (Box<Path>, Box<Path>) {
    let composer_json = std::env::var("COMPOSER").unwrap_or_else(|_| "composer.json".into());
    let composer_lock = composer_json.replace("json", "lock");

    (
        cwd.join(composer_json).into_boxed_path(),
        cwd.join(composer_lock).into_boxed_path(),
    )
}

fn composer_home() -> Result<PathBuf, ConfigError> {
    if let Ok(home) = std::env::var("COMPOSER_HOME") {
        return Ok(PathBuf::from(home));
    }

    if cfg!(windows) {
        if let Ok(app_data) = std::env::var("APPDATA") {
            return Ok(PathBuf::from(app_data))
        }

        return Err(ConfigError::CannotDetermineComposerHome);
    }

    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(xdg_config_home).join("composer"));
    }

    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home).join(".composer"));
    }

    Err(ConfigError::CannotDetermineComposerHome)
}
