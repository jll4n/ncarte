use std::env;
use colored::Colorize;

mod scanner;

fn intro() {
    println!();
    print!("{}", "ncarte ".bold().blue());
    print!("{}", " |  ".bold().white());
    println!("{}", "Scanner de ports en Rust".bold().red());
    println!();
}

async fn start_scan(ip: &str, port: u16) {
    println!("Démarrage du scan sur {}:{}", ip, port);
    let _ = scanner::tcpmain(ip, port, false).await;
}

async fn start_fullscan(ip: &str) {
    println!("Démarrage du scan complet sur {}", ip);
    let _ = scanner::tcpfullscan(ip).await;
}

#[tokio::main]
async fn main() {
    intro();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[2].parse::<u16>().is_ok() {
        println!("Target: {}", args[1]);
        println!("Port: {}", args[2]);
        start_scan(&args[1], args[2].parse::<u16>().unwrap()).await;
    }
    else if args.len() == 1 {
        start_fullscan(&args[1]).await;
    } 
    else if args.len() > 2 && args[2].parse::<u16>().is_err() {
        eprintln!("{}", "/!/ Le port doit être un nombre entier entre 0 et 65535 /!/".yellow());
        return;
    }      
}