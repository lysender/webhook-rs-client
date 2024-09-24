use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    thread,
    time::Duration,
};

use tracing::{error, info};

use crate::{config::Config, Result};

pub fn start_client(config: &Config) {
    loop {
        let conn = connect(&config);
        if let Err(e) = conn {
            error!("Error connecting to the server: {}", e);
            info!("Reconnecting in 10 seconds...");

            thread::sleep(Duration::from_secs(10));
        }
    }
}

fn connect(config: &Config) -> Result<()> {
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
                return Err("Error reading message".into());
            }
        }
    }

    Ok(())
}
