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
            error!("Connection error: {}", e);
            info!("Reconnecting in 10 seconds...");

            thread::sleep(Duration::from_secs(10));
        }
    }
}

fn connect(config: &Config) -> Result<()> {
    let stream_res = TcpStream::connect(&config.server_address);
    match stream_res {
        Ok(stream) => {
            info!("Connected to server...");
            let _ = handle_connection(stream);
            Ok(())
        }
        Err(e) => {
            let connect_err = format!("Error connecting to the server: {}", e);
            Err(connect_err.into())
        }
    }
}

fn handle_connection(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(message) => {
                info!("Received message: {}", message);
            }
            Err(e) => {
                let msg_error = format!("Error reading message: {}", e);
                return Err(msg_error.into());
            }
        }
    }

    Ok(())
}
