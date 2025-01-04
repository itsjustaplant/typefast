use std::fs;
use std::io::Error as StandardError;
use std::path::PathBuf;

use dirs;
use rand::{seq::SliceRandom, thread_rng};
use thiserror::Error;

use crate::constants::{APP_PATH, WORD_LIST};

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("Could not get config directory path")]
    GetAppConfigPath(),
    #[error("Could not create config folder: {0}")]
    CreateConfigFolder(StandardError),
}

pub fn get_app_config_path() -> Result<PathBuf, FileSystemError> {
    dirs::config_dir()
        .map(|mut config_directory| {
            config_directory.push(APP_PATH);
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

pub fn get_words() -> Vec<&'static str> {
    WORD_LIST
        .lines()
        .collect::<Vec<&str>>()
        .choose_multiple(&mut thread_rng(), 100)
        .cloned()
        .collect()
}
