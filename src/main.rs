use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        let _ = tx.send(());
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    match rx.recv() {
        Ok(()) => println!("CTRL-C received."),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("Exiting...");
}