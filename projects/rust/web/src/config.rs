//! Configuration module for loading settings from CLI arguments and environment variables
//!
//! This module handles:
//! - Loading `.env` files from cwd and executable directories
//! - CLI argument parsing via clap
//! - Environment variable fallbacks for port (HTTP_PORT, PORT)
//! - wwwroot directory resolution (CLI, then search logic)

use clap::Parser;
use std::path::PathBuf;

use crate::env;
use crate::wwwroot;

/// CLI arguments for the web server
#[derive(Parser, Debug)]
#[command(name = "ansi-display-rs")]
#[command(about = "Web server for testing the ansi-to-html-rs library")]
struct Args {
    /// Port number to bind against
    #[arg(short, long)]
    port: Option<u16>,

    /// Directory for static files (absolute or relative to working directory)
    #[arg(short, long)]
    wwwroot: Option<PathBuf>,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Port number to bind the server to
    pub port: u16,
    /// Path to the wwwroot directory for static files
    pub wwwroot_path: PathBuf,
}

impl Config {
    /// Load configuration from CLI arguments and environment variables.
    ///
    /// Port resolution order:
    /// 1. `--port` or `-p` CLI argument
    /// 2. `HTTP_PORT` environment variable
    /// 3. `PORT` environment variable
    /// 4. Default: 3000
    ///
    /// wwwroot resolution order:
    /// 1. `--wwwroot` or `-w` CLI argument (resolved relative to cwd if not absolute)
    /// 2. Search logic via `wwwroot::get_wwwroot_path()`
    ///
    /// # Errors
    ///
    /// Returns `Err` with an error message if:
    /// - The specified wwwroot directory does not exist
    /// - No wwwroot directory could be found via search logic
    pub fn load() -> Result<Self, String> {
        // Load .env files first (before reading environment variables)
        // Values from .env files do NOT override existing environment variables
        env::load_env_files();

        let args = Args::parse();

        let port = resolve_port(args.port);
        let wwwroot_path = resolve_wwwroot(args.wwwroot)?;

        Ok(Config { port, wwwroot_path })
    }
}

/// Resolve the port number from CLI, environment variables, or default
fn resolve_port(cli_port: Option<u16>) -> u16 {
    // 1. CLI argument takes priority
    if let Some(port) = cli_port {
        return port;
    }

    // 2. HTTP_PORT environment variable
    if let Ok(port_str) = std::env::var("HTTP_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            return port;
        }
    }

    // 3. PORT environment variable
    if let Ok(port_str) = std::env::var("PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            return port;
        }
    }

    // 4. Default to 3000
    3000
}

/// Resolve the wwwroot directory from CLI or search logic
fn resolve_wwwroot(cli_wwwroot: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(cli_path) = cli_wwwroot {
        // CLI argument provided - resolve relative to cwd if not absolute
        let path = if cli_path.is_absolute() {
            cli_path
        } else {
            std::env::current_dir()
                .unwrap_or_default()
                .join(&cli_path)
        };

        if !path.is_dir() {
            return Err(format!(
                "Specified wwwroot directory does not exist: {}",
                path.display()
            ));
        }
        Ok(path)
    } else {
        // Use search logic
        wwwroot::get_wwwroot_path().ok_or_else(|| {
            let mut msg = String::from("Could not find wwwroot directory.\n");
            msg.push_str("Searched locations:\n");
            msg.push_str("  - WWWROOT environment variable (relative to cwd and executable)\n");
            msg.push_str("  - wwwroot/ directory in current directory and parent directories\n");
            msg.push_str("  - /var/www/html\n\n");
            msg.push_str("Use --wwwroot or -w to specify a directory explicitly.");
            msg
        })
    }
}
