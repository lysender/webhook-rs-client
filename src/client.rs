use std::{
    io::{BufRead, BufReader, Write},
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
            if let Err(conn_err) = handle_connection(stream) {
                return Err(conn_err);
            }
            Ok(())
        }
        Err(e) => {
            let connect_err = format!("Error connecting to the server: {}", e);
            Err(connect_err.into())
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    info!("Authenticating to server...");

    // Before reading incoming messages, send a message to the server first
    let auth_msg = format!("AUTH /auth WEBHOOK/1.0\r\nAuthorization: jwt_token\n");
    let write_res = stream.write_all(auth_msg.as_bytes());

    if let Err(write_err) = write_res {
        let msg = format!("Authenticating to server failed: {}", write_err);
        return Err(msg.into());
    }

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(message) => {
                info!("Received message: {}", message);
                if let Err(auth_err) = handle_auth_response(message) {
                    return Err(auth_err);
                }
            }
            Err(e) => {
                let msg_error = format!("Error reading message: {}", e);
                return Err(msg_error.into());
            }
        }
    }

    Ok(())
}

fn handle_auth_response(message: String) -> Result<()> {
    match message.as_str() {
        "WEBHOOK/1.0 200 OK" => Ok(()),
        "WEBHOOK/1.0 401 Unauthorized" => Err("Authentication failed".into()),
        _ => Ok(()),
    }
}
