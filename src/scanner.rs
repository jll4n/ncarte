use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::timeout;

pub async fn tcpmain(ip: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let full_ip = format!("{}:{}", ip, port);
    match timeout(Duration::from_secs(1), TcpStream::connect(&full_ip)).await {
        Ok(Ok(_)) => println!("Port TCP {} ouvert", port),
        _ => println!("Port TCP {} fermé", port),
    }
    Ok(())
}