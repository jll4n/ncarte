use std::env;
use colored::Colorize;

fn intro() {
    println!();
    print!("{}", "ncarte ".bold().blue());
    print!("{}", " |  ".bold().white());
    println!("{}", "Scanner de ports en Rust".bold().red());
    println!();
}

fn main() {
    intro();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[2].parse::<u16>().is_ok() {
        println!("Target: {}", args[1]);
        println!("Port: {}", args[2]);
    }
    else if args.len() < 2 {
        eprintln!("{}", "Il manque des arguments : ncarte <target> <port>".bold().yellow());
        return;
    } 
    else if args[2].parse::<u16>().is_err() {
        eprintln!("{}", "Le port doit être un nombre entier entre 0 et 65535".bold().yellow());
        return;
    }      
    println!("Arguments: {:?}", args); 
}
