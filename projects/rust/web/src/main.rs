mod config;
mod env;
mod server;
mod wwwroot;

use config::Config;

/// Main entry point
#[tokio::main]
async fn main() {
    let config = match Config::load() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    server::run(config).await;
}
