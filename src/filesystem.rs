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

pub fn file_exists(app_config_path: &PathBuf, file_name: &str) -> bool {
  let absolute_path = app_config_path.join(file_name);
  fs::metadata(&absolute_path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{TEST_APP_PATH, TEST_DB_NAME};

    #[test]
    fn test_file_exists() {
        let mut path = PathBuf::new();
        path.push(TEST_APP_PATH);

        let correct_result = file_exists(&path, TEST_DB_NAME);
        let wrong_result = file_exists(&path, "idk.db");

        assert_eq!(correct_result, true);
        assert_eq!(wrong_result, false);
    }

    #[test]
    fn test_config_path() {
        let app_config_path = get_app_config_path();
        assert!(app_config_path.is_ok());
    }

    #[test]
    fn test_return_ok_if_folder_exist() {
        let mut path = PathBuf::new();
        path.push(TEST_APP_PATH);

        let result = create_config_folder(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_words() {
        let words = get_words();
        assert_eq!(words.len(), 100);
    }
}
