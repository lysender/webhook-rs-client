use std::process;

mod client;
mod config;
mod error;

use client::start_client;
use config::Config;

// Re-exports
pub use error::{Error, Result};

fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "webhook_rs_client=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    if let Err(e) = run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = Config::build()?;
    start_client(&config);
    Ok(())
}
