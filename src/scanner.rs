use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::sync::Semaphore;
use std::time::Duration;
use std::sync::Arc;

pub async fn tcpmain(ip: &str, port: u16, _is_full_scan: bool) -> Result<(), Box<dyn std::error::Error>> {
    let full_ip = format!("{}:{}", ip, port);
    match timeout(Duration::from_secs(1), TcpStream::connect(&full_ip)).await {
        Ok(Ok(_)) => println!("Port TCP {} ouvert", port),
        _ => println!("Port TCP {} fermé", port),
    }
    Ok(())
}

pub async fn tcpfullscan(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(100));
    let mut handles = vec![];
    for port in 0..=65535 {
        let sem = Arc::clone(&semaphore);
        let ip2 = ip.to_string();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await;
            let _ = tcpmain(&ip2, port, true).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
    Ok(())
}