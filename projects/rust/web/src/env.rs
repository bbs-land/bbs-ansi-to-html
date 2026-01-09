//! Environment file loading module
//!
//! This module handles loading `.env` files from multiple locations.
//! Values from `.env` files do NOT override existing environment variables.
//!
//! Search order:
//! 1. From current working directory, search upward up to 3 directories
//! 2. From executable directory, search upward up to 3 directories
//!
//! The dotenvy library supports multi-line quoted strings.

use std::path::{Path, PathBuf};

/// Maximum number of parent directories to search upward
const MAX_PARENT_SEARCH: usize = 3;

/// Load `.env` files from standard locations.
///
/// This function searches for `.env` files in the following order:
/// 1. From current working directory, searching upward up to 3 directories
/// 2. From executable directory, searching upward up to 3 directories
///
/// Values from `.env` files do NOT replace existing environment variables.
/// If multiple `.env` files are found, all are loaded (existing values take precedence).
pub fn load_env_files() {
    // 1. Search from current working directory
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(env_path) = find_env_file_upward(&cwd, MAX_PARENT_SEARCH) {
            load_env_file(&env_path);
        }
    }

    // 2. Search from executable directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Some(env_path) = find_env_file_upward(exe_dir, MAX_PARENT_SEARCH) {
                load_env_file(&env_path);
            }
        }
    }
}

/// Search upward from the given directory for a `.env` file.
///
/// # Arguments
///
/// * `start_dir` - The directory to start searching from
/// * `max_levels` - Maximum number of parent directories to search
///
/// # Returns
///
/// The path to the first `.env` file found, or `None` if not found.
fn find_env_file_upward(start_dir: &Path, max_levels: usize) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    for _ in 0..=max_levels {
        let env_path = current.join(".env");
        if env_path.is_file() {
            return Some(env_path);
        }

        // Move to parent directory
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    None
}

/// Load a `.env` file without overriding existing environment variables.
///
/// # Arguments
///
/// * `path` - Path to the `.env` file to load
fn load_env_file(path: &Path) {
    // dotenvy::from_path_override would override existing values
    // dotenvy::from_path does NOT override existing values (what we want)
    if let Err(e) = dotenvy::from_path(path) {
        // Silently ignore errors - the file might have been removed
        // between finding it and loading it, or have permission issues
        eprintln!("Warning: Failed to load {}: {}", path.display(), e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::env;

    #[test]
    fn test_find_env_file_upward_not_found() {
        // Test with a directory that definitely won't have a .env file
        let result = find_env_file_upward(Path::new("/nonexistent/path"), 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_env_file_upward_in_current() {
        // Create a temp directory with a .env file
        let temp_dir = env::temp_dir().join("msg_test_env_current");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let env_file = temp_dir.join(".env");
        fs::write(&env_file, "TEST_VAR=test").unwrap();

        let result = find_env_file_upward(&temp_dir, 3);
        assert_eq!(result, Some(env_file.clone()));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_find_env_file_upward_in_parent() {
        // Create a temp directory structure
        let temp_dir = env::temp_dir().join("msg_test_env_parent");
        let _ = fs::remove_dir_all(&temp_dir);
        let child_dir = temp_dir.join("child");
        fs::create_dir_all(&child_dir).unwrap();

        let env_file = temp_dir.join(".env");
        fs::write(&env_file, "TEST_VAR=test").unwrap();

        let result = find_env_file_upward(&child_dir, 3);
        assert_eq!(result, Some(env_file.clone()));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
