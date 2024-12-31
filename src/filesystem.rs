use std::fs;
use std::io::Error;
use std::path::PathBuf;

use dirs;
use thiserror::Error;

use crate::constants;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("Could not get config directory path")]
    GetAppConfigPath(),
    #[error("Could not create config folder: {0}")]
    CreateConfigFolder(Error),
}

pub fn get_app_config_path() -> Result<PathBuf, FileSystemError> {
    dirs::config_dir()
        .map(|mut config_directory| {
            config_directory.push(constants::APP_PATH);
            config_directory
        })
        .ok_or_else(FileSystemError::GetAppConfigPath)
}

pub fn create_config_folder(app_config_path: &PathBuf) -> Result<(), FileSystemError> {
    match fs::create_dir_all(app_config_path) {
        Ok(()) => Ok(()),
        Err(e) => Err(FileSystemError::CreateConfigFolder(e)),
    }
}
