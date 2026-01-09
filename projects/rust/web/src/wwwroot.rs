//! wwwroot directory resolution logic
//!
//! This module provides functionality to locate the wwwroot directory
//! for serving static files. The search order is:
//!
//! 1. WWWROOT environment variable (relative to cwd, then executable)
//! 2. Search upward from current directory for wwwroot/
//! 3. /var/www/html fallback

use std::path::{Path, PathBuf};

/// Get the path to the wwwroot directory.
///
/// Searches in the following order:
/// 1. WWWROOT environment variable - if set and non-empty:
///    - Relative paths: checked relative to cwd, then relative to executable
///    - Absolute paths: used directly if the directory exists
/// 2. Search upward from current working directory for wwwroot/
/// 3. /var/www/html as system fallback
///
/// Returns `None` if no valid wwwroot directory is found.
pub fn get_wwwroot_path() -> Option<PathBuf> {
    // 1. Check WWWROOT environment variable
    if let Some(path) = check_env_var() {
        return Some(path);
    }

    // 2. Search upward from current working directory for wwwroot/
    if let Some(path) = search_parent_directories() {
        return Some(path);
    }

    // 3. Try /var/www/html as fallback
    if let Some(path) = check_var_www_html() {
        return Some(path);
    }

    // 4. No wwwroot found
    None
}

/// Check the WWWROOT environment variable
fn check_env_var() -> Option<PathBuf> {
    let wwwroot_env = std::env::var("WWWROOT").ok()?;

    if wwwroot_env.is_empty() {
        return None;
    }

    let env_path = Path::new(&wwwroot_env);

    if env_path.is_relative() {
        // Try relative to current working directory first
        if let Ok(cwd) = std::env::current_dir() {
            let cwd_path = cwd.join(env_path);
            if cwd_path.is_dir() {
                return Some(cwd_path);
            }
        }

        // Then try relative to executable location
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                let exe_path = exe_dir.join(env_path);
                if exe_path.is_dir() {
                    return Some(exe_path);
                }
            }
        }
    } else {
        // Absolute path
        if env_path.is_dir() {
            return Some(env_path.to_path_buf());
        }
    }

    None
}

/// Search upward from current working directory for wwwroot/
fn search_parent_directories() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut current = cwd.as_path();

    loop {
        let wwwroot_path = current.join("wwwroot");
        if wwwroot_path.is_dir() {
            return Some(wwwroot_path);
        }

        current = current.parent()?;
    }
}

/// Check /var/www/html as system fallback
fn check_var_www_html() -> Option<PathBuf> {
    let var_www_html = Path::new("/var/www/html");
    if var_www_html.is_dir() {
        Some(var_www_html.to_path_buf())
    } else {
        None
    }
}
