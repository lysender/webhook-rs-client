use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use tracing::{error, info};

use crate::config::Config;

pub fn start_client(config: &Config) {
    let stream = TcpStream::connect(&config.server_address).unwrap();
    info!("Connected to the server: {}", config.server_address);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(message) => {
                info!("Received message: {}", message);
            }
            Err(e) => {
                error!("Error reading message: {}", e);
            }
        }
    }
}
