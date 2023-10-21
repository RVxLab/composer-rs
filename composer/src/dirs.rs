use std::env::{current_dir, var};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub struct Directories {
    pub data_dir: Box<Path>,
    pub archive_dir: Box<Path>,
    pub cache_dir: Box<Path>,
    pub home_dir: Box<Path>,
}

#[cfg(windows)]
impl Directories {
    fn try_get_default_directories() -> Result<Directories, DirectoryError> {
        let local_app_data_dir = var("LOCALAPPDATA")
            .map_err(|_| DirectoryError::CannotDetermineDirectory("local app data".into()))?;
        let roaming_app_data_dir = var("APPDATA")
            .map_err(|_| DirectoryError::CannotDetermineDirectory("roaming app data".into()))?;

        let cache_dir = PathBuf::from(local_app_data_dir).join("Composer");
        let home_dir = PathBuf::from(roaming_app_data_dir);
        let data_dir = home_dir.join("Composer");
        let archive_dir =
            current_dir().map_err(|_| DirectoryError::CannotDetermineDirectory("current".into))?;

        Ok(Self {
            home_dir: home_dir.into_boxed_path(),
            cache_dir: cache_dir.into_boxed_path(),
            data_dir: data_dir.into_boxed_path(),
            archive_dir: archive_dir.into_boxed_path(),
        })
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl Directories {
    pub fn try_get_default_directories() -> Result<Directories, DirectoryError> {
        let home_dir = var("XDG_CONFIG_HOME").map_or_else(
            |_| {
                if let Ok(home) = var("HOME") {
                    return Ok(PathBuf::from(home).join(".composer"));
                }

                Err(DirectoryError::CannotDetermineDirectory("home".into()))
            },
            |xdg_config_home| Ok(PathBuf::from(xdg_config_home).join("composer")),
        )?;

        let data_dir = var("XDG_DATA_HOME").map_or_else(
            |_| home_dir.clone(),
            |xdg_data_home| PathBuf::from(xdg_data_home).join("composer"),
        );

        let cache_dir = var("XDG_CACHE_HOME").map_or_else(
            |_| home_dir.clone().join("cache"),
            |xdg_cache_dir| PathBuf::from(xdg_cache_dir).join("composer"),
        );

        let archive_dir =
            current_dir().map_err(|_| DirectoryError::CannotDetermineDirectory("current".into))?;

        Ok(Self {
            home_dir: home_dir.into_boxed_path(),
            data_dir: data_dir.into_boxed_path(),
            cache_dir: cache_dir.into_boxed_path(),
            archive_dir: archive_dir.into_boxed_path(),
        })
    }
}

#[cfg(target_os = "macos")]
impl Directories {
    pub fn try_get_default_directories() -> Result<Directories, DirectoryError> {
        let user_home_dir = var("HOME").map_or_else(
            |_| Err(DirectoryError::CannotDetermineDirectory("home".into())),
            |home_dir| Ok(PathBuf::from(home_dir)),
        )?;

        let home_dir = user_home_dir.join(".composer");
        let data_dir = home_dir.clone();
        let cache_dir = user_home_dir.join("Library/Caches/composer");
        let archive_dir = current_dir()
            .map_err(|_| DirectoryError::CannotDetermineDirectory("current".into()))?;

        Ok(Self {
            home_dir: home_dir.into_boxed_path(),
            data_dir: data_dir.into_boxed_path(),
            cache_dir: cache_dir.into_boxed_path(),
            archive_dir: archive_dir.into_boxed_path(),
        })
    }
}

#[derive(Error, Debug)]
pub enum DirectoryError {
    #[error("Unable to determine {0} directory")]
    CannotDetermineDirectory(String),
}
